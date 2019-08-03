use super::event::Event;
use super::resource::Resource;
use super::wayland::wl_compositor::WlCompositor;
use super::wayland::wl_registry::WlRegistry;
use super::wayland::wl_shm::WlShm;
use super::xdg_shell::xdg_wm_base::XdgWmBase;
use std::collections::HashMap;
use tokio::sync::mpsc::Sender;

pub struct Session {
    pub resources: HashMap<u32, Resource>,
    pub wl_registry: WlRegistry,
    pub wl_compositor: WlCompositor,
    pub wl_shm: WlShm,
    pub xdg_wm_base: XdgWmBase,
    pub tx: Sender<Box<Event + Send>>,
    pub callback_data: u32,
}

pub struct Context<T>
where
    T: Into<Resource>,
{
    pub sender_object_id: u32,
    pub sender_object: T,

    pub resources: HashMap<u32, Resource>,
    pub wl_registry: WlRegistry,
    pub wl_compositor: WlCompositor,
    pub wl_shm: WlShm,
    pub xdg_wm_base: XdgWmBase,
    pub tx: Sender<Box<Event + Send>>,
    pub callback_data: u32,
}

impl<T> Context<T>
where
    T: Into<Resource>,
{
    pub fn new(session: Session, sender_object: T, sender_object_id: u32) -> Self {
        Self {
            resources: session.resources,
            wl_registry: session.wl_registry,
            wl_compositor: session.wl_compositor,
            wl_shm: session.wl_shm,
            xdg_wm_base: session.xdg_wm_base,
            tx: session.tx,
            callback_data: session.callback_data,
            sender_object_id,
            sender_object,
        }
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
            wl_registry: self.wl_registry,
            wl_compositor: self.wl_compositor,
            wl_shm: self.wl_shm,
            xdg_wm_base: self.xdg_wm_base,
            tx: self.tx,
            callback_data: self.callback_data,
        }
    }
}
