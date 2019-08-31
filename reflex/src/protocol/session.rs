use super::event::Event;
use super::resource::Resource;
use super::wayland::wl_compositor::WlCompositor;
use super::wayland::wl_display::WlDisplay;
use super::wayland::wl_registry::WlRegistry;
use super::wayland::wl_shm::WlShm;
use super::xdg_shell::xdg_wm_base::XdgWmBase;
use crate::protocol::wayland::wl_data_device_manager::WlDataDeviceManager;
use futures::future::Future;
use futures::sink::Sink;
use std::collections::HashMap;
use std::os::unix::io::RawFd;
use std::sync::{Arc, RwLock};
use tokio::net::UnixStream;
use tokio::sync::mpsc::Sender;
use tokio::io::WriteHalf;

pub enum NextAction {
    Nop,
    Relay,
    RelayWait,
}

pub struct Session {
    pub resources: HashMap<u32, Resource>,
    pub wl_display: Arc<RwLock<WlDisplay>>,
    pub wl_compositor: Arc<RwLock<WlCompositor>>,
    pub wl_shm: Arc<RwLock<WlShm>>,
    pub wl_registry: Arc<RwLock<WlRegistry>>,
    pub wl_data_device_manager: Arc<RwLock<WlDataDeviceManager>>,
    pub xdg_wm_base: Arc<RwLock<XdgWmBase>>,
    pub tx: Sender<Box<dyn Event + Send>>,
    pub callback_data: u32,
    pub fds: Vec<RawFd>,
    pub unix_stream: WriteHalf<UnixStream>,
}

pub struct Context<T>
where
    T: Into<Resource>,
{
    pub sender_object_id: u32,
    pub sender_object: T,

    pub resources: HashMap<u32, Resource>,
    pub wl_display: Arc<RwLock<WlDisplay>>,
    pub wl_compositor: Arc<RwLock<WlCompositor>>,
    pub wl_shm: Arc<RwLock<WlShm>>,
    pub wl_registry: Arc<RwLock<WlRegistry>>,
    pub wl_data_device_manager: Arc<RwLock<WlDataDeviceManager>>,
    pub xdg_wm_base: Arc<RwLock<XdgWmBase>>,
    pub tx: Sender<Box<dyn Event + Send>>,
    pub callback_data: u32,
    pub fds: Vec<RawFd>,
    pub unix_stream: WriteHalf<UnixStream>,
}

impl<T> Context<T>
where
    T: Into<Resource>,
{
    pub fn new(session: Session, sender_object: T, sender_object_id: u32) -> Self {
        Self {
            resources: session.resources,
            wl_display: session.wl_display,
            wl_registry: session.wl_registry,
            wl_compositor: session.wl_compositor,
            wl_shm: session.wl_shm,
            wl_data_device_manager: session.wl_data_device_manager,
            xdg_wm_base: session.xdg_wm_base,
            tx: session.tx,
            callback_data: session.callback_data,
            fds: session.fds,
            sender_object_id,
            sender_object,
            unix_stream: session.unix_stream,
        }
    }

    pub fn ok(
        self,
    ) -> Box<dyn futures::future::Future<Item = (Session, NextAction), Error = ()> + Send> {
        Box::new(futures::future::ok((self.into(), NextAction::Relay)))
    }

    fn create_invalid_method_error(
        &self,
        message: String,
    ) -> crate::protocol::wayland::wl_display::events::Error {
        crate::protocol::wayland::wl_display::events::Error {
            sender_object_id: 1,
            object_id: self.sender_object_id,
            code: crate::protocol::wayland::wl_display::enums::Error::InvalidMethod as u32,
            message,
        }
    }

    pub fn invalid_method(
        self,
        message: String,
    ) -> Box<dyn futures::future::Future<Item = (Session, NextAction), Error = ()> + Send> {
        let tx = self.tx.clone();
        let error = self.create_invalid_method_error(message);
        let session: Session = self.into();

        Box::new(
            tx.send(Box::new(error))
                .map_err(|_| ())
                .map(|_| (session, NextAction::Nop)),
        )
    }

    pub fn invalid_method_dispatch(
        self,
        message: String,
    ) -> Box<dyn futures::future::Future<Item = Session, Error = ()> + Send> {
        let tx = self.tx.clone();
        let error = self.create_invalid_method_error(message);
        let session: Session = self.into();

        Box::new(tx.send(Box::new(error)).map_err(|_| ()).map(|_| session))
    }
}

impl<T> Into<Session> for Context<T>
where
    T: Into<Resource>,
{
    fn into(mut self) -> Session {
        self.resources
            .insert(self.sender_object_id, self.sender_object.into());
        Session {
            resources: self.resources,
            wl_display: self.wl_display,
            wl_registry: self.wl_registry,
            wl_compositor: self.wl_compositor,
            wl_shm: self.wl_shm,
            wl_data_device_manager: self.wl_data_device_manager,
            xdg_wm_base: self.xdg_wm_base,
            tx: self.tx,
            callback_data: self.callback_data,
            fds: self.fds,
            unix_stream: self.unix_stream,
        }
    }
}

impl Session {
    pub fn relay(
        self,
        buf: Vec<u8>,
    ) -> Box<dyn futures::future::Future<Item = Session, Error = ()> + Send> {
        let (relay_session, unix_stream) = self.into_relay_session();
        Box::new(
            tokio::io::write_all(unix_stream, buf)
                .map_err(|err| panic!("relay err: {:?}", err))
                .and_then(|(u, _): (WriteHalf<UnixStream>, Vec<u8>)| {
                    futures::future::ok(Session::from_relay_session(relay_session, u))
                }),
        )
        //Box::new(futures::future::ok(self))
    }

    pub fn relay_wait(
        self,
        buf: Vec<u8>,
    ) -> Box<dyn futures::future::Future<Item = Session, Error = ()> + Send> {
        let (relay_session, unix_stream) = self.into_relay_session();
        Box::new(
            tokio::io::write_all(unix_stream, buf)
                .map_err(|err| panic!("relay_wait err: {:?}", err))
                .and_then(|(u, _): (WriteHalf<UnixStream>, Vec<u8>)| {
                    futures::future::ok(Session::from_relay_session(relay_session, u))
                }),
        )
        //Box::new(futures::future::ok(self))
    }

    fn from_relay_session(relay_session: RelaySession, unix_stream: WriteHalf<UnixStream>) -> Session {
        Session {
            resources: relay_session.resources,
            wl_display: relay_session.wl_display,
            wl_registry: relay_session.wl_registry,
            wl_compositor: relay_session.wl_compositor,
            wl_shm: relay_session.wl_shm,
            wl_data_device_manager: relay_session.wl_data_device_manager,
            xdg_wm_base: relay_session.xdg_wm_base,
            tx: relay_session.tx,
            callback_data: relay_session.callback_data,
            fds: relay_session.fds,
            unix_stream,
        }
    }

    fn into_relay_session(self) -> (RelaySession, WriteHalf<UnixStream>) {
        let unix_stream = self.unix_stream;
        let relay_session = RelaySession {
            resources: self.resources,
            wl_display: self.wl_display,
            wl_registry: self.wl_registry,
            wl_compositor: self.wl_compositor,
            wl_shm: self.wl_shm,
            wl_data_device_manager: self.wl_data_device_manager,
            xdg_wm_base: self.xdg_wm_base,
            tx: self.tx,
            callback_data: self.callback_data,
            fds: self.fds,
        };
        (relay_session, unix_stream)
    }
}

struct RelaySession {
    pub resources: HashMap<u32, Resource>,
    pub wl_display: Arc<RwLock<WlDisplay>>,
    pub wl_compositor: Arc<RwLock<WlCompositor>>,
    pub wl_shm: Arc<RwLock<WlShm>>,
    pub wl_registry: Arc<RwLock<WlRegistry>>,
    pub wl_data_device_manager: Arc<RwLock<WlDataDeviceManager>>,
    pub xdg_wm_base: Arc<RwLock<XdgWmBase>>,
    pub tx: Sender<Box<dyn Event + Send>>,
    pub callback_data: u32,
    pub fds: Vec<RawFd>,
}
