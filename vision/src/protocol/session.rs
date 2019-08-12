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
    pub tx: Sender<Box<Event + Send>>,
    pub callback_data: u32,
    pub fds: Vec<RawFd>,
    pub unix_stream: UnixStream,
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
    pub tx: Sender<Box<Event + Send>>,
    pub callback_data: u32,
    pub fds: Vec<RawFd>,
    pub unix_stream: UnixStream,
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
    ) -> Box<futures::future::Future<Item = (Session, NextAction), Error = ()> + Send> {
        Box::new(futures::future::ok((self.into(), NextAction::Relay)))
    }

    fn create_invalid_method_error(
        &self,
        message: String,
    ) -> crate::protocol::wayland::wl_display::events::Error {
        crate::protocol::wayland::wl_display::events::Error {
            sender_object_id: 1,
            object_id: self.sender_object_id.clone(),
            code: crate::protocol::wayland::wl_display::enums::Error::InvalidMethod as u32,
            message,
        }
    }

    pub fn invalid_method(
        self,
        message: String,
    ) -> Box<futures::future::Future<Item = (Session, NextAction), Error = ()> + Send> {
        let tx = self.tx.clone();
        let error = self.create_invalid_method_error(message);
        let session: Session = self.into();
        return Box::new(
            tx.send(Box::new(error))
                .map_err(|_| ())
                .map(|_| (session, NextAction::Nop)),
        );
    }

    pub fn invalid_method_dispatch(
        self,
        message: String,
    ) -> Box<futures::future::Future<Item = Session, Error = ()> + Send> {
        let tx = self.tx.clone();
        let error = self.create_invalid_method_error(message);
        let session: Session = self.into();
        return Box::new(tx.send(Box::new(error)).map_err(|_| ()).map(|_| session));
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
