use std::sync::{Arc, RwLock};

pub enum Resource {
    WlBuffer(crate::protocol::wayland::wl_buffer::WlBuffer),
    WlCallback(crate::protocol::wayland::wl_callback::WlCallback),
    WlCompositor(Arc<RwLock<crate::protocol::wayland::wl_compositor::WlCompositor>>),
    WlDataDevice(crate::protocol::wayland::wl_data_device::WlDataDevice),
    WlDataDeviceManager(crate::protocol::wayland::wl_data_device_manager::WlDataDeviceManager),
    WlDataOffer(crate::protocol::wayland::wl_data_offer::WlDataOffer),
    WlDataSource(crate::protocol::wayland::wl_data_source::WlDataSource),
    WlDisplay(Arc<RwLock<crate::protocol::wayland::wl_display::WlDisplay>>),
    WlKeyboard(crate::protocol::wayland::wl_keyboard::WlKeyboard),
    WlOutput(crate::protocol::wayland::wl_output::WlOutput),
    WlPointer(crate::protocol::wayland::wl_pointer::WlPointer),
    WlRegion(crate::protocol::wayland::wl_region::WlRegion),
    WlRegistry(crate::protocol::wayland::wl_registry::WlRegistry),
    WlSeat(crate::protocol::wayland::wl_seat::WlSeat),
    WlShell(crate::protocol::wayland::wl_shell::WlShell),
    WlShellSurface(crate::protocol::wayland::wl_shell_surface::WlShellSurface),
    WlShm(crate::protocol::wayland::wl_shm::WlShm),
    WlShmPool(crate::protocol::wayland::wl_shm_pool::WlShmPool),
    WlSubcompositor(crate::protocol::wayland::wl_subcompositor::WlSubcompositor),
    WlSubsurface(crate::protocol::wayland::wl_subsurface::WlSubsurface),
    WlSurface(crate::protocol::wayland::wl_surface::WlSurface),
    WlTouch(crate::protocol::wayland::wl_touch::WlTouch),
    XdgPopup(crate::protocol::xdg_shell::xdg_popup::XdgPopup),
    XdgPositioner(crate::protocol::xdg_shell::xdg_positioner::XdgPositioner),
    XdgSurface(crate::protocol::xdg_shell::xdg_surface::XdgSurface),
    XdgToplevel(crate::protocol::xdg_shell::xdg_toplevel::XdgToplevel),
    XdgWmBase(Arc<RwLock<crate::protocol::xdg_shell::xdg_wm_base::XdgWmBase>>),
}

pub fn dispatch_request(resource: Resource, session: crate::protocol::session::Session, sender_object_id: u32, opcode: u16, args: Vec<u8>) -> Box<futures::future::Future<Item = crate::protocol::session::Session, Error = ()> + Send> {
    match resource {
        Resource::WlBuffer(object) => {
            super::wayland::wl_buffer::dispatch_request(crate::protocol::session::Context::new(session, object, sender_object_id), opcode, args)
        }
        Resource::WlCallback(object) => {
            super::wayland::wl_callback::dispatch_request(crate::protocol::session::Context::new(session, object, sender_object_id), opcode, args)
        }
        Resource::WlCompositor(object) => {
            super::wayland::wl_compositor::dispatch_request(crate::protocol::session::Context::new(session, object, sender_object_id), opcode, args)
        }
        Resource::WlDataDevice(object) => {
            super::wayland::wl_data_device::dispatch_request(crate::protocol::session::Context::new(session, object, sender_object_id), opcode, args)
        }
        Resource::WlDataDeviceManager(object) => {
            super::wayland::wl_data_device_manager::dispatch_request(crate::protocol::session::Context::new(session, object, sender_object_id), opcode, args)
        }
        Resource::WlDataOffer(object) => {
            super::wayland::wl_data_offer::dispatch_request(crate::protocol::session::Context::new(session, object, sender_object_id), opcode, args)
        }
        Resource::WlDataSource(object) => {
            super::wayland::wl_data_source::dispatch_request(crate::protocol::session::Context::new(session, object, sender_object_id), opcode, args)
        }
        Resource::WlDisplay(object) => {
            super::wayland::wl_display::dispatch_request(crate::protocol::session::Context::new(session, object, sender_object_id), opcode, args)
        }
        Resource::WlKeyboard(object) => {
            super::wayland::wl_keyboard::dispatch_request(crate::protocol::session::Context::new(session, object, sender_object_id), opcode, args)
        }
        Resource::WlOutput(object) => {
            super::wayland::wl_output::dispatch_request(crate::protocol::session::Context::new(session, object, sender_object_id), opcode, args)
        }
        Resource::WlPointer(object) => {
            super::wayland::wl_pointer::dispatch_request(crate::protocol::session::Context::new(session, object, sender_object_id), opcode, args)
        }
        Resource::WlRegion(object) => {
            super::wayland::wl_region::dispatch_request(crate::protocol::session::Context::new(session, object, sender_object_id), opcode, args)
        }
        Resource::WlRegistry(object) => {
            super::wayland::wl_registry::dispatch_request(crate::protocol::session::Context::new(session, object, sender_object_id), opcode, args)
        }
        Resource::WlSeat(object) => {
            super::wayland::wl_seat::dispatch_request(crate::protocol::session::Context::new(session, object, sender_object_id), opcode, args)
        }
        Resource::WlShell(object) => {
            super::wayland::wl_shell::dispatch_request(crate::protocol::session::Context::new(session, object, sender_object_id), opcode, args)
        }
        Resource::WlShellSurface(object) => {
            super::wayland::wl_shell_surface::dispatch_request(crate::protocol::session::Context::new(session, object, sender_object_id), opcode, args)
        }
        Resource::WlShm(object) => {
            super::wayland::wl_shm::dispatch_request(crate::protocol::session::Context::new(session, object, sender_object_id), opcode, args)
        }
        Resource::WlShmPool(object) => {
            super::wayland::wl_shm_pool::dispatch_request(crate::protocol::session::Context::new(session, object, sender_object_id), opcode, args)
        }
        Resource::WlSubcompositor(object) => {
            super::wayland::wl_subcompositor::dispatch_request(crate::protocol::session::Context::new(session, object, sender_object_id), opcode, args)
        }
        Resource::WlSubsurface(object) => {
            super::wayland::wl_subsurface::dispatch_request(crate::protocol::session::Context::new(session, object, sender_object_id), opcode, args)
        }
        Resource::WlSurface(object) => {
            super::wayland::wl_surface::dispatch_request(crate::protocol::session::Context::new(session, object, sender_object_id), opcode, args)
        }
        Resource::WlTouch(object) => {
            super::wayland::wl_touch::dispatch_request(crate::protocol::session::Context::new(session, object, sender_object_id), opcode, args)
        }
        Resource::XdgPopup(object) => {
            super::xdg_shell::xdg_popup::dispatch_request(crate::protocol::session::Context::new(session, object, sender_object_id), opcode, args)
        }
        Resource::XdgPositioner(object) => {
            super::xdg_shell::xdg_positioner::dispatch_request(crate::protocol::session::Context::new(session, object, sender_object_id), opcode, args)
        }
        Resource::XdgSurface(object) => {
            super::xdg_shell::xdg_surface::dispatch_request(crate::protocol::session::Context::new(session, object, sender_object_id), opcode, args)
        }
        Resource::XdgToplevel(object) => {
            super::xdg_shell::xdg_toplevel::dispatch_request(crate::protocol::session::Context::new(session, object, sender_object_id), opcode, args)
        }
        Resource::XdgWmBase(object) => {
            super::xdg_shell::xdg_wm_base::dispatch_request(crate::protocol::session::Context::new(session, object, sender_object_id), opcode, args)
        }
    }
}
