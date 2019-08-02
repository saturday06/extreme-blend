use super::wayland::wl_buffer::WlBuffer;
use super::wayland::wl_compositor::WlCompositor;
use super::wayland::wl_display::WlDisplay;
use super::wayland::wl_registry::WlRegistry;
use super::wayland::wl_shm::WlShm;
use super::wayland::wl_shm_pool::WlShmPool;
use super::wayland::wl_surface::WlSurface;
use super::wayland_event::WaylandEvent;
use super::wayland_request::WaylandRequest;
use super::xdg_shell::xdg_surface::XdgSurface;
use super::xdg_shell::xdg_toplevel::XdgToplevel;
use super::xdg_shell::xdg_wm_base::XdgWmBase;
use crate::session_state::SessionState;
use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub enum WlResource {
    WlDisplay(Arc<RwLock<WlDisplay>>),
    WlRegistry(Arc<RwLock<WlRegistry>>),
    WlShm(Arc<RwLock<WlShm>>),
    WlCompositor(Arc<RwLock<WlCompositor>>),
    WlSurface(Arc<RwLock<WlSurface>>),
    XdgWmBase(Arc<RwLock<XdgWmBase>>),
    XdgSurface(Arc<RwLock<XdgSurface>>),
    XdgToplevel(Arc<RwLock<XdgToplevel>>),
    WlBuffer(Arc<RwLock<WlBuffer>>),
    WlShmPool(Arc<RwLock<WlShmPool>>),
}

pub fn handle(
    o: WlResource,
    session_state: Arc<RwLock<SessionState>>,
    tx: tokio::sync::mpsc::Sender<Box<WaylandEvent + Send>>,
    sender_object_id: u32,
    opcode: u16,
    args: Vec<u8>,
) -> Box<Future<Item = (), Error = ()> + Send> {
    match o {
        WlResource::WlCompositor(obj) => {
            WlCompositor::handle(obj, session_state, tx, sender_object_id, opcode, args)
        }
        WlResource::WlShm(obj) => {
            WlShm::handle(obj, session_state, tx, sender_object_id, opcode, args)
        }
        WlResource::WlRegistry(obj) => {
            WlRegistry::handle(obj, session_state, tx, sender_object_id, opcode, args)
        }
        WlResource::WlSurface(obj) => {
            WlSurface::handle(obj, session_state, tx, sender_object_id, opcode, args)
        }
        WlResource::WlDisplay(obj) => {
            WlDisplay::handle(obj, session_state, tx, sender_object_id, opcode, args)
        }
        WlResource::XdgWmBase(obj) => {
            XdgWmBase::handle(obj, session_state, tx, sender_object_id, opcode, args)
        }
        WlResource::XdgSurface(obj) => {
            XdgSurface::handle(obj, session_state, tx, sender_object_id, opcode, args)
        }
        WlResource::XdgToplevel(obj) => {
            XdgToplevel::handle(obj, session_state, tx, sender_object_id, opcode, args)
        }
        WlResource::WlBuffer(obj) => {
            WlBuffer::handle(obj, session_state, tx, sender_object_id, opcode, args)
        }
        WlResource::WlShmPool(obj) => {
            WlShmPool::handle(obj, session_state, tx, sender_object_id, opcode, args)
        }
    }
}
