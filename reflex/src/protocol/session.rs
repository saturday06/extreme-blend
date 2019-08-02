use super::resource::Resource;
use super::wayland::wl_compositor::WlCompositor;
use super::wayland::wl_registry::WlRegistry;
use super::wayland::wl_shm::WlShm;
use super::xdg_shell::xdg_wm_base::XdgWmBase;
use std::collections::HashMap;

pub struct Session {
    pub resources: HashMap<u32, Resource>,
    pub wl_registry: WlRegistry,
    pub wl_compositor: WlCompositor,
    pub wl_shm: WlShm,
    pub xdg_wm_base: XdgWmBase,
}
