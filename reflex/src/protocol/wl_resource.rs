use std::sync::{Arc, RwLock};

#[derive(Clone)]
enum WlResource {
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
