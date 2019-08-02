use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use crate::protocol::wl_resource::WlResource;
use crate::protocol::wayland::wl_registry::WlRegistry;
use crate::protocol::wayland::wl_shm::WlShm;
use crate::protocol::wayland::wl_compositor::WlCompositor;
use crate::protocol::xdg_shell::xdg_wm_base::XdgWmBase;

pub struct SessionState {
    object_map: HashMap<u32, WlResource>,
    wl_registry: Arc<RwLock<WlRegistry>>,
    wl_compositor: Arc<RwLock<WlCompositor>>,
    wl_shm: Arc<RwLock<WlShm>>,
    xdg_wm_base: Arc<RwLock<XdgWmBase>>,
}
