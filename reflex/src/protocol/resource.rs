use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone)]
pub enum Resource {
    WlBuffer(Rc<RefCell<super::wayland::wl_buffer::WlBuffer>>),
    WlCallback(Rc<RefCell<super::wayland::wl_callback::WlCallback>>),
    WlCompositor(Rc<RefCell<super::wayland::wl_compositor::WlCompositor>>),
    WlDataDevice(Rc<RefCell<super::wayland::wl_data_device::WlDataDevice>>),
    WlDataDeviceManager(Rc<RefCell<super::wayland::wl_data_device_manager::WlDataDeviceManager>>),
    WlDataOffer(Rc<RefCell<super::wayland::wl_data_offer::WlDataOffer>>),
    WlDataSource(Rc<RefCell<super::wayland::wl_data_source::WlDataSource>>),
    WlDisplay(Rc<RefCell<super::wayland::wl_display::WlDisplay>>),
    WlKeyboard(Rc<RefCell<super::wayland::wl_keyboard::WlKeyboard>>),
    WlOutput(Rc<RefCell<super::wayland::wl_output::WlOutput>>),
    WlPointer(Rc<RefCell<super::wayland::wl_pointer::WlPointer>>),
    WlRegion(Rc<RefCell<super::wayland::wl_region::WlRegion>>),
    WlRegistry(Rc<RefCell<super::wayland::wl_registry::WlRegistry>>),
    WlSeat(Rc<RefCell<super::wayland::wl_seat::WlSeat>>),
    WlShell(Rc<RefCell<super::wayland::wl_shell::WlShell>>),
    WlShellSurface(Rc<RefCell<super::wayland::wl_shell_surface::WlShellSurface>>),
    WlShm(Rc<RefCell<super::wayland::wl_shm::WlShm>>),
    WlShmPool(Rc<RefCell<super::wayland::wl_shm_pool::WlShmPool>>),
    WlSubcompositor(Rc<RefCell<super::wayland::wl_subcompositor::WlSubcompositor>>),
    WlSubsurface(Rc<RefCell<super::wayland::wl_subsurface::WlSubsurface>>),
    WlSurface(Rc<RefCell<super::wayland::wl_surface::WlSurface>>),
    WlTouch(Rc<RefCell<super::wayland::wl_touch::WlTouch>>),
    XdgPopup(Rc<RefCell<super::xdg_shell::xdg_popup::XdgPopup>>),
    XdgPositioner(Rc<RefCell<super::xdg_shell::xdg_positioner::XdgPositioner>>),
    XdgSurface(Rc<RefCell<super::xdg_shell::xdg_surface::XdgSurface>>),
    XdgToplevel(Rc<RefCell<super::xdg_shell::xdg_toplevel::XdgToplevel>>),
    XdgWmBase(Rc<RefCell<super::xdg_shell::xdg_wm_base::XdgWmBase>>),
}

pub fn dispatch_request(resource: Resource, session: &mut super::session::Session, tx: tokio::sync::mpsc::Sender<Box<super::event::Event + Send>>, sender_object_id: u32, opcode: u16, args: Vec<u8>) -> Box<futures::future::Future<Item = (), Error = ()>> {
    match resource {
        Resource::WlBuffer(object) => {
            super::wayland::wl_buffer::dispatch_request(object)
        }
        Resource::WlCallback(object) => {
            super::wayland::wl_callback::dispatch_request(object)
        }
        Resource::WlCompositor(object) => {
            super::wayland::wl_compositor::dispatch_request(object)
        }
        Resource::WlDataDevice(object) => {
            super::wayland::wl_data_device::dispatch_request(object)
        }
        Resource::WlDataDeviceManager(object) => {
            super::wayland::wl_data_device_manager::dispatch_request(object)
        }
        Resource::WlDataOffer(object) => {
            super::wayland::wl_data_offer::dispatch_request(object)
        }
        Resource::WlDataSource(object) => {
            super::wayland::wl_data_source::dispatch_request(object)
        }
        Resource::WlDisplay(object) => {
            super::wayland::wl_display::dispatch_request(object)
        }
        Resource::WlKeyboard(object) => {
            super::wayland::wl_keyboard::dispatch_request(object)
        }
        Resource::WlOutput(object) => {
            super::wayland::wl_output::dispatch_request(object)
        }
        Resource::WlPointer(object) => {
            super::wayland::wl_pointer::dispatch_request(object)
        }
        Resource::WlRegion(object) => {
            super::wayland::wl_region::dispatch_request(object)
        }
        Resource::WlRegistry(object) => {
            super::wayland::wl_registry::dispatch_request(object)
        }
        Resource::WlSeat(object) => {
            super::wayland::wl_seat::dispatch_request(object)
        }
        Resource::WlShell(object) => {
            super::wayland::wl_shell::dispatch_request(object)
        }
        Resource::WlShellSurface(object) => {
            super::wayland::wl_shell_surface::dispatch_request(object)
        }
        Resource::WlShm(object) => {
            super::wayland::wl_shm::dispatch_request(object)
        }
        Resource::WlShmPool(object) => {
            super::wayland::wl_shm_pool::dispatch_request(object)
        }
        Resource::WlSubcompositor(object) => {
            super::wayland::wl_subcompositor::dispatch_request(object)
        }
        Resource::WlSubsurface(object) => {
            super::wayland::wl_subsurface::dispatch_request(object)
        }
        Resource::WlSurface(object) => {
            super::wayland::wl_surface::dispatch_request(object)
        }
        Resource::WlTouch(object) => {
            super::wayland::wl_touch::dispatch_request(object)
        }
        Resource::XdgPopup(object) => {
            super::xdg_shell::xdg_popup::dispatch_request(object)
        }
        Resource::XdgPositioner(object) => {
            super::xdg_shell::xdg_positioner::dispatch_request(object)
        }
        Resource::XdgSurface(object) => {
            super::xdg_shell::xdg_surface::dispatch_request(object)
        }
        Resource::XdgToplevel(object) => {
            super::xdg_shell::xdg_toplevel::dispatch_request(object)
        }
        Resource::XdgWmBase(object) => {
            super::xdg_shell::xdg_wm_base::dispatch_request(object)
        }
    }
}
