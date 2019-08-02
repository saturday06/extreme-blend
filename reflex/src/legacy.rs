use byteorder::{ByteOrder, NativeEndian, ReadBytesExt};
use bytes::BytesMut;
use futures::future::Future;
use futures::sink::Sink;
use futures::stream::Stream;
use std::collections::HashMap;
use std::io::Cursor;
use std::io::Read;
use std::os::raw::{c_char, c_int};
use std::sync::{Arc, RwLock};
use tokio::codec::Decoder;
use tokio::reactor::Handle;
use tokio::runtime::Runtime;

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


pub struct WlCompositor {
    name: u32,
}

impl WlCompositor {
    fn create_surface(
        _sender_object: Arc<RwLock<Self>>,
        session_state: Arc<RwLock<SessionState>>,
        _tx: tokio::sync::mpsc::Sender<Box<WaylandEvent + Send>>,
        _sender_object_id: u32,
        wl_surface_id: u32,
    ) -> Box<Future<Item = (), Error = ()> + Send> {
        session_state.write().unwrap().object_map.insert(
            wl_surface_id,
            WlResource::WlSurface(Arc::new(RwLock::new(WlSurface {}))),
        );
        Box::new(futures::future::ok(()))
    }

    fn handle(
        sender_object: Arc<RwLock<Self>>,
        session_state: Arc<RwLock<SessionState>>,
        tx: tokio::sync::mpsc::Sender<Box<WaylandEvent + Send>>,
        sender_object_id: u32,
        opcode: u16,
        args: Vec<u8>,
    ) -> Box<Future<Item = (), Error = ()> + Send> {
        let mut cursor = Cursor::new(&args);
        match opcode {
            0 if args.len() == 4 => {
                return Self::create_surface(
                    sender_object,
                    session_state,
                    tx,
                    sender_object_id,
                    cursor.read_u32::<NativeEndian>().unwrap(),
                );
            }
            _ => {}
        }
        Box::new(
            tx.send(Box::new(WlDisplayError {
                object_id: sender_object_id,
                code: WL_DISPLAY_ERROR_INVALID_METHOD,
                message: format!(
                    "WlCompositor@{} opcode={} args={:?} not found",
                    sender_object_id, opcode, args
                ),
            }))
                .map_err(|_| ())
                .map(|_tx| ()),
        )
    }
}


struct WlSurface {}

impl WlSurface {
    fn commit(
        _sender_object: Arc<RwLock<Self>>,
        _session_state: Arc<RwLock<SessionState>>,
        tx: tokio::sync::mpsc::Sender<Box<WaylandEvent + Send>>,
        sender_object_id: u32,
    ) -> Box<Future<Item = (), Error = ()> + Send> {
        Box::new(futures::future::ok(()))
    }

    fn destroy(
        _sender_object: Arc<RwLock<Self>>,
        session_state: Arc<RwLock<SessionState>>,
        _tx: tokio::sync::mpsc::Sender<Box<WaylandEvent + Send>>,
        sender_object_id: u32,
    ) -> Box<Future<Item = (), Error = ()> + Send> {
        let removed = {
            let object_map = &mut session_state.write().unwrap().object_map;
            if let Some(WlResource::WlSurface(_)) = object_map.get(&sender_object_id) {
                object_map.remove(&sender_object_id);
                true
            } else {
                false
            }
        };
        if removed {
            Box::new(futures::future::ok(()))
        } else {
            Box::new(futures::future::err(()))
        }
    }

    fn handle(
        sender_object: Arc<RwLock<Self>>,
        session_state: Arc<RwLock<SessionState>>,
        tx: tokio::sync::mpsc::Sender<Box<WaylandEvent + Send>>,
        sender_object_id: u32,
        opcode: u16,
        args: Vec<u8>,
    ) -> Box<Future<Item = (), Error = ()> + Send> {
        let mut cursor = Cursor::new(&args);
        match opcode {
            1 if args.len() == 0 => {
                return Self::destroy(sender_object, session_state, tx, sender_object_id);
            }
            6 if args.len() == 0 => {
                return Self::commit(sender_object, session_state, tx, sender_object_id);
            }
            _ => {}
        }
        Box::new(
            tx.send(Box::new(WlDisplayError {
                object_id: sender_object_id,
                code: WL_DISPLAY_ERROR_INVALID_METHOD,
                message: format!(
                    "WlSurface@{} opcode={} args={:?} not found",
                    sender_object_id, opcode, args
                ),
            }))
            .map_err(|_| ())
            .map(|_tx| ()),
        )
    }
}

struct WlShm {
    name: u32,
}

impl WlShm {}

impl WlShm {
    fn create_pool(
        sender_object: Arc<RwLock<Self>>,
        session_state: Arc<RwLock<SessionState>>,
        tx: tokio::sync::mpsc::Sender<Box<WaylandEvent + Send>>,
        sender_object_id: u32,
        wl_shm_pool_id: u32,
        fd: u32,
        size: i32,
    ) -> Box<Future<Item = (), Error = ()> + Send> {
        session_state.write().unwrap().object_map.insert(
            wl_shm_pool_id,
            WlResource::WlShmPool(Arc::new(RwLock::new(WlShmPool { fd, size }))),
        );
        Box::new(futures::future::ok(()))
    }

    fn destroy(
        _sender_object: Arc<RwLock<Self>>,
        session_state: Arc<RwLock<SessionState>>,
        tx: tokio::sync::mpsc::Sender<Box<WaylandEvent + Send>>,
        sender_object_id: u32,
    ) -> Box<Future<Item = (), Error = ()> + Send> {
        let removed = {
            let object_map = &mut session_state.write().unwrap().object_map;
            if let Some(WlResource::WlShmPool(_)) = object_map.get(&sender_object_id) {
                object_map.remove(&sender_object_id);
                true
            } else {
                false
            }
        };
        if removed {
            Box::new(futures::future::ok(()))
        } else {
            Box::new(futures::future::err(()))
        }
    }

    fn handle(
        sender_object: Arc<RwLock<Self>>,
        session_state: Arc<RwLock<SessionState>>,
        tx: tokio::sync::mpsc::Sender<Box<WaylandEvent + Send>>,
        sender_object_id: u32,
        opcode: u16,
        args: Vec<u8>,
    ) -> Box<Future<Item = (), Error = ()> + Send> {
        let mut cursor = Cursor::new(&args);
        match opcode {
            0 if args.len() == 12 => {
                return Self::create_pool(
                    sender_object,
                    session_state,
                    tx,
                    sender_object_id,
                    cursor.read_u32::<NativeEndian>().unwrap(),
                    cursor.read_u32::<NativeEndian>().unwrap(),
                    cursor.read_i32::<NativeEndian>().unwrap(),
                );
            }
            1 if args.len() == 0 => {
                return Self::destroy(sender_object, session_state, tx, sender_object_id);
            }
            _ => {}
        };

        Box::new(
            tx.send(Box::new(WlDisplayError {
                object_id: sender_object_id,
                code: WL_DISPLAY_ERROR_INVALID_METHOD,
                message: format!(
                    "WlShm@{} opcode={} args={:?} not found",
                    sender_object_id, opcode, args,
                ),
            }))
            .map_err(|_| ())
            .map(|_tx| ()),
        )
    }
}

struct XdgSurface {
    wl_surface: Arc<RwLock<WlSurface>>,
}

impl XdgSurface {
    fn get_toplevel(
        sender_object: Arc<RwLock<Self>>,
        session_state: Arc<RwLock<SessionState>>,
        tx: tokio::sync::mpsc::Sender<Box<WaylandEvent + Send>>,
        sender_object_id: u32,
        xdg_toplevel_id: u32,
    ) -> Box<Future<Item = (), Error = ()> + Send> {
        session_state.write().unwrap().object_map.insert(
            xdg_toplevel_id,
            WlResource::XdgToplevel(Arc::new(RwLock::new(XdgToplevel {}))),
        );
        Box::new(futures::future::ok(()))
    }

    fn destroy(
        _sender_object: Arc<RwLock<Self>>,
        session_state: Arc<RwLock<SessionState>>,
        _tx: tokio::sync::mpsc::Sender<Box<WaylandEvent + Send>>,
        sender_object_id: u32,
    ) -> Box<Future<Item = (), Error = ()> + Send> {
        let removed = {
            let object_map = &mut session_state.write().unwrap().object_map;
            if let Some(WlResource::XdgSurface(_)) = object_map.get(&sender_object_id) {
                object_map.remove(&sender_object_id);
                true
            } else {
                false
            }
        };
        if removed {
            Box::new(futures::future::ok(()))
        } else {
            Box::new(futures::future::err(()))
        }
    }

    fn handle(
        sender_object: Arc<RwLock<Self>>,
        session_state: Arc<RwLock<SessionState>>,
        tx: tokio::sync::mpsc::Sender<Box<WaylandEvent + Send>>,
        sender_object_id: u32,
        opcode: u16,
        args: Vec<u8>,
    ) -> Box<Future<Item = (), Error = ()> + Send> {
        let mut cursor = Cursor::new(&args);
        match opcode {
            0 if args.len() == 0 => {
                return Self::destroy(sender_object, session_state, tx, sender_object_id);
            }
            1 if args.len() == 4 => {
                return Self::get_toplevel(
                    sender_object,
                    session_state,
                    tx,
                    sender_object_id,
                    cursor.read_u32::<NativeEndian>().unwrap(),
                );
            }
            _ => {}
        };
        Box::new(
            tx.send(Box::new(WlDisplayError {
                object_id: sender_object_id,
                code: WL_DISPLAY_ERROR_INVALID_METHOD,
                message: format!(
                    "XdgSurface@{} opcode={} args={:?} not found",
                    sender_object_id, opcode, args,
                ),
            }))
            .map_err(|_| ())
            .map(|_tx| ()),
        )
    }
}

struct XdgToplevel {}

impl XdgToplevel {
    fn destroy(
        _sender_object: Arc<RwLock<Self>>,
        session_state: Arc<RwLock<SessionState>>,
        _tx: tokio::sync::mpsc::Sender<Box<WaylandEvent + Send>>,
        sender_object_id: u32,
    ) -> Box<Future<Item = (), Error = ()> + Send> {
        let removed = {
            let object_map = &mut session_state.write().unwrap().object_map;
            if let Some(WlResource::XdgToplevel(_)) = object_map.get(&sender_object_id) {
                object_map.remove(&sender_object_id);
                true
            } else {
                false
            }
        };
        if removed {
            Box::new(futures::future::ok(()))
        } else {
            Box::new(futures::future::err(()))
        }
    }

    fn handle(
        sender_object: Arc<RwLock<Self>>,
        session_state: Arc<RwLock<SessionState>>,
        tx: tokio::sync::mpsc::Sender<Box<WaylandEvent + Send>>,
        sender_object_id: u32,
        opcode: u16,
        args: Vec<u8>,
    ) -> Box<Future<Item = (), Error = ()> + Send> {
        let mut cursor = Cursor::new(&args);
        match opcode {
            0 if args.len() == 0 => {
                return Self::destroy(sender_object, session_state, tx, sender_object_id);
            }
            _ => {}
        };

        Box::new(
            tx.send(Box::new(WlDisplayError {
                object_id: sender_object_id,
                code: WL_DISPLAY_ERROR_INVALID_METHOD,
                message: format!(
                    "XdgToplevel@{} opcode={} args={:?} not found",
                    sender_object_id, opcode, args,
                ),
            }))
            .map_err(|_| ())
            .map(|_tx| ()),
        )
    }
}

struct WlBuffer {
    offset: i32,
    width: i32,
    height: i32,
    stride: i32,
    format: u32,
}

impl WlBuffer {
    fn destroy(
        _sender_object: Arc<RwLock<Self>>,
        session_state: Arc<RwLock<SessionState>>,
        tx: tokio::sync::mpsc::Sender<Box<WaylandEvent + Send>>,
        sender_object_id: u32,
    ) -> Box<Future<Item = (), Error = ()> + Send> {
        let removed = {
            let object_map = &mut session_state.write().unwrap().object_map;
            if let Some(WlResource::WlBuffer(_)) = object_map.get(&sender_object_id) {
                object_map.remove(&sender_object_id);
                true
            } else {
                false
            }
        };
        if removed {
            Box::new(futures::future::ok(()))
        } else {
            Box::new(futures::future::err(()))
        }
    }

    fn handle(
        sender_object: Arc<RwLock<Self>>,
        session_state: Arc<RwLock<SessionState>>,
        tx: tokio::sync::mpsc::Sender<Box<WaylandEvent + Send>>,
        sender_object_id: u32,
        opcode: u16,
        args: Vec<u8>,
    ) -> Box<Future<Item = (), Error = ()> + Send> {
        let mut cursor = Cursor::new(&args);
        match opcode {
            0 if args.len() == 0 => {
                return Self::destroy(sender_object, session_state, tx, sender_object_id);
            }
            _ => {}
        };

        Box::new(
            tx.send(Box::new(WlDisplayError {
                object_id: sender_object_id,
                code: WL_DISPLAY_ERROR_INVALID_METHOD,
                message: format!(
                    "WlBuffer@{} opcode={} args={:?} not found",
                    sender_object_id, opcode, args,
                ),
            }))
            .map_err(|_| ())
            .map(|_tx| ()),
        )
    }
}

struct WlShmPool {
    fd: u32,
    size: i32,
}

impl WlShmPool {
    fn create_buffer(
        sender_object: Arc<RwLock<Self>>,
        session_state: Arc<RwLock<SessionState>>,
        tx: tokio::sync::mpsc::Sender<Box<WaylandEvent + Send>>,
        sender_object_id: u32,
        wl_buffer_id: u32,
        offset: i32,
        width: i32,
        height: i32,
        stride: i32,
        format: u32,
    ) -> Box<Future<Item = (), Error = ()> + Send> {
        session_state.write().unwrap().object_map.insert(
            wl_buffer_id,
            WlResource::WlBuffer(Arc::new(RwLock::new(WlBuffer {
                offset,
                width,
                height,
                stride,
                format,
            }))),
        );
        Box::new(futures::future::ok(()))
    }

    fn destroy(
        _sender_object: Arc<RwLock<Self>>,
        session_state: Arc<RwLock<SessionState>>,
        _tx: tokio::sync::mpsc::Sender<Box<WaylandEvent + Send>>,
        sender_object_id: u32,
    ) -> Box<Future<Item = (), Error = ()> + Send> {
        let removed = {
            let object_map = &mut session_state.write().unwrap().object_map;
            if let Some(WlResource::WlShmPool(_)) = object_map.get(&sender_object_id) {
                object_map.remove(&sender_object_id);
                true
            } else {
                false
            }
        };
        if removed {
            Box::new(futures::future::ok(()))
        } else {
            Box::new(futures::future::err(()))
        }
    }

    fn handle(
        sender_object: Arc<RwLock<Self>>,
        session_state: Arc<RwLock<SessionState>>,
        tx: tokio::sync::mpsc::Sender<Box<WaylandEvent + Send>>,
        sender_object_id: u32,
        opcode: u16,
        args: Vec<u8>,
    ) -> Box<Future<Item = (), Error = ()> + Send> {
        let mut cursor = Cursor::new(&args);
        match opcode {
            0 if args.len() == 24 => {
                return Self::create_buffer(
                    sender_object,
                    session_state,
                    tx,
                    sender_object_id,
                    cursor.read_u32::<NativeEndian>().unwrap(),
                    cursor.read_i32::<NativeEndian>().unwrap(),
                    cursor.read_i32::<NativeEndian>().unwrap(),
                    cursor.read_i32::<NativeEndian>().unwrap(),
                    cursor.read_i32::<NativeEndian>().unwrap(),
                    cursor.read_u32::<NativeEndian>().unwrap(),
                );
            }
            1 if args.len() == 0 => {
                return Self::destroy(sender_object, session_state, tx, sender_object_id);
            }
            _ => {}
        };

        Box::new(
            tx.send(Box::new(WlDisplayError {
                object_id: sender_object_id,
                code: WL_DISPLAY_ERROR_INVALID_METHOD,
                message: format!(
                    "WlShmPool@{} opcode={} args={:?} not found",
                    sender_object_id, opcode, args,
                ),
            }))
            .map_err(|_| ())
            .map(|_tx| ()),
        )
    }
}

struct XdgWmBase {
    name: u32,
}

impl XdgWmBase {
    fn destroy(
        _sender_object: Arc<RwLock<Self>>,
        session_state: Arc<RwLock<SessionState>>,
        _tx: tokio::sync::mpsc::Sender<Box<WaylandEvent + Send>>,
        sender_object_id: u32,
    ) -> Box<Future<Item = (), Error = ()> + Send> {
        let removed = {
            let object_map = &mut session_state.write().unwrap().object_map;
            if let Some(WlResource::XdgWmBase(_)) = object_map.get(&sender_object_id) {
                object_map.remove(&sender_object_id);
                true
            } else {
                false
            }
        };
        if removed {
            Box::new(futures::future::ok(()))
        } else {
            Box::new(futures::future::err(()))
        }
    }

    fn create_positioner(
        _sender_object: Arc<RwLock<Self>>,
        session_state: Arc<RwLock<SessionState>>,
        tx: tokio::sync::mpsc::Sender<Box<WaylandEvent + Send>>,
        sender_object_id: u32,
        xdg_positioner_id: u32,
    ) -> Box<Future<Item = (), Error = ()> + Send> {
        return Box::new(
            tx.send(Box::new(WlDisplayError {
                object_id: sender_object_id,
                code: WL_DISPLAY_ERROR_INVALID_METHOD,
                message: format!(
                    "XdgWmBase@{}::create_positioner(xdg_positioner_id={}): wl_surface not found",
                    sender_object_id, xdg_positioner_id
                ),
            }))
            .map_err(|_| ())
            .map(|_tx| ()),
        );
    }

    fn get_xdg_surface(
        _sender_object: Arc<RwLock<Self>>,
        session_state: Arc<RwLock<SessionState>>,
        tx: tokio::sync::mpsc::Sender<Box<WaylandEvent + Send>>,
        sender_object_id: u32,
        xdg_surface_id: u32,
        wl_surface_id: u32,
    ) -> Box<Future<Item = (), Error = ()> + Send> {
        let wl_surface = if let Some(WlResource::WlSurface(x)) =
            session_state.read().unwrap().object_map.get(&wl_surface_id)
        {
            x.clone()
        } else {
            return Box::new(
            tx.send(Box::new(WlDisplayError {
                object_id: sender_object_id,
                code: WL_DISPLAY_ERROR_INVALID_METHOD,
                message: format!(
                    "XdgWmBase@{}::get_xdg_surface(xdg_surface_id={} wl_surface_id={}): wl_surface not found",
                    sender_object_id, xdg_surface_id, wl_surface_id
                ),
            }))
            .map_err(|_| ())
            .map(|_tx| ()));
        };

        session_state.write().unwrap().object_map.insert(
            xdg_surface_id,
            WlResource::XdgSurface(Arc::new(RwLock::new(XdgSurface {
                wl_surface: wl_surface,
            }))),
        );
        Box::new(futures::future::ok(()))
    }

    fn handle(
        sender_object: Arc<RwLock<Self>>,
        session_state: Arc<RwLock<SessionState>>,
        tx: tokio::sync::mpsc::Sender<Box<WaylandEvent + Send>>,
        sender_object_id: u32,
        opcode: u16,
        args: Vec<u8>,
    ) -> Box<Future<Item = (), Error = ()> + Send> {
        let mut cursor = Cursor::new(&args);
        match opcode {
            0 if args.len() == 0 => {
                return Self::destroy(sender_object, session_state, tx, sender_object_id);
            }
            1 if args.len() == 4 => {
                return Self::create_positioner(
                    sender_object,
                    session_state,
                    tx,
                    sender_object_id,
                    cursor.read_u32::<NativeEndian>().unwrap(),
                );
            }
            2 if args.len() == 8 => {
                return Self::get_xdg_surface(
                    sender_object,
                    session_state,
                    tx,
                    sender_object_id,
                    cursor.read_u32::<NativeEndian>().unwrap(),
                    cursor.read_u32::<NativeEndian>().unwrap(),
                );
            }
            _ => {}
        }
        Box::new(
            tx.send(Box::new(WlDisplayError {
                object_id: sender_object_id,
                code: WL_DISPLAY_ERROR_INVALID_METHOD,
                message: format!(
                    "XdgWmBase@{} opcode={} args={:?} not found",
                    sender_object_id, opcode, args,
                ),
            }))
            .map_err(|_| ())
            .map(|_tx| ()),
        )
    }
}

struct WlRegistry {}

impl WlRegistry {
    fn bind2(
        sender_object: Arc<RwLock<Self>>,
        session_state: Arc<RwLock<SessionState>>,
        tx: tokio::sync::mpsc::Sender<Box<WaylandEvent + Send>>,
        sender_object_id: u32,
        name: u32,
        _name_buf: Vec<u8>,
        _version: u32,
        id: u32,
    ) -> Box<Future<Item = (), Error = ()> + Send> {
        Self::bind(sender_object, session_state, tx, sender_object_id, name, id)
    }

    fn bind(
        _sender_object: Arc<RwLock<Self>>,
        session_state: Arc<RwLock<SessionState>>,
        tx: tokio::sync::mpsc::Sender<Box<WaylandEvent + Send>>,
        sender_object_id: u32,
        name: u32,
        id: u32,
    ) -> Box<Future<Item = (), Error = ()> + Send> {
        println!("WlRegistry::bind(name: {}, id: {})", name, id);
        if name
            == session_state
                .read()
                .unwrap()
                .wl_compositor
                .read()
                .unwrap()
                .name
        {
            let mut lock = session_state.write().unwrap();
            let wl_compositor = lock.wl_compositor.clone();
            lock.object_map
                .insert(id, WlResource::WlCompositor(wl_compositor));
            return Box::new(futures::future::ok(()));
        } else if name == session_state.read().unwrap().wl_shm.read().unwrap().name {
            let mut lock = session_state.write().unwrap();
            let wl_shm = lock.wl_shm.clone();
            lock.object_map.insert(id, WlResource::WlShm(wl_shm));
            return Box::new(
                tx.send(Box::new(WlShmFormat {
                    sender_object_id: id,
                    format: 0, // argb8888
                }))
                .and_then(move |tx1| {
                    tx1.send(Box::new(WlShmFormat {
                        sender_object_id: id,
                        format: 1, // xrgb8888
                    }))
                })
                .map_err(|_| ())
                .map(|_tx| ()),
            );
        } else if name
            == session_state
                .read()
                .unwrap()
                .xdg_wm_base
                .read()
                .unwrap()
                .name
        {
            let mut lock = session_state.write().unwrap();
            let xdg_wm_base = lock.xdg_wm_base.clone();
            lock.object_map
                .insert(id, WlResource::XdgWmBase(xdg_wm_base));
            return Box::new(futures::future::ok(()));
        }

        Box::new(
            tx.send(Box::new(WlDisplayError {
                object_id: sender_object_id,
                code: WL_DISPLAY_ERROR_INVALID_METHOD,
                message: format!(
                    "WlRegistry@{}.bind(name={}, id={}) not found",
                    sender_object_id, name, id
                ),
            }))
            .map_err(|_| ())
            .map(|_tx| ()),
        )
    }
}

impl WlRegistry {
    fn handle(
        sender_object: Arc<RwLock<Self>>,
        session_state: Arc<RwLock<SessionState>>,
        tx: tokio::sync::mpsc::Sender<Box<WaylandEvent + Send>>,
        sender_object_id: u32,
        opcode: u16,
        args: Vec<u8>,
    ) -> Box<Future<Item = (), Error = ()> + Send> {
        let mut cursor = Cursor::new(&args);
        match opcode {
            0 if args.len() == 8 => {
                return Self::bind(
                    sender_object,
                    session_state,
                    tx,
                    sender_object_id,
                    cursor.read_u32::<NativeEndian>().unwrap(),
                    cursor.read_u32::<NativeEndian>().unwrap(),
                );
            }
            0 if args.len() > 8 => {
                let name = cursor.read_u32::<NativeEndian>().unwrap();
                let name_buf_len = cursor.read_u32::<NativeEndian>().unwrap() as usize;
                let name_buf_len_with_pad = (name_buf_len + 3) / 4 * 4;
                let mut name_buf = Vec::new();
                name_buf.resize(name_buf_len, 0);
                cursor.read_exact(&mut name_buf).unwrap();
                cursor.set_position(
                    cursor.position() + (name_buf_len_with_pad - name_buf_len) as u64,
                );
                let version = cursor.read_u32::<NativeEndian>().unwrap();
                let id = cursor.read_u32::<NativeEndian>().unwrap();
                if args.len() == 4 + 4 + name_buf_len_with_pad + 4 + 4 {
                    return Self::bind2(
                        sender_object,
                        session_state,
                        tx,
                        sender_object_id,
                        name,
                        name_buf,
                        version,
                        id,
                    );
                }
            }
            _ => (),
        }
        Box::new(
            tx.send(Box::new(WlDisplayError {
                object_id: sender_object_id,
                code: WL_DISPLAY_ERROR_INVALID_METHOD,
                message: format!(
                    "WlRegistry@{} opcode={} args={:?} not found",
                    sender_object_id, opcode, args
                ),
            }))
            .map_err(|_| ())
            .map(|_tx| ()),
        )
    }
}

struct WlDisplay {
    next_callback_data: u32,
}

impl WlDisplay {
    fn sync(
        sender_object: Arc<RwLock<Self>>,
        _session_state: Arc<RwLock<SessionState>>,
        tx: tokio::sync::mpsc::Sender<Box<WaylandEvent + Send>>,
        _sender_object_id: u32,
        wl_callback_id: u32,
    ) -> Box<Future<Item = (), Error = ()> + Send> {
        println!("WlDisplay::sync({})", wl_callback_id);
        let callback_data = {
            let mut obj = sender_object.write().unwrap();
            obj.next_callback_data += 1;
            obj.next_callback_data
        };
        Box::new(
            tx.send(Box::new(WlCallbackDone {
                sender_object_id: wl_callback_id,
                callback_data,
            }))
            .map_err(|_| ())
            .map(|_tx| ()),
        )
    }

    fn get_registry(
        _sender_object: Arc<RwLock<Self>>,
        session_state: Arc<RwLock<SessionState>>,
        tx0: tokio::sync::mpsc::Sender<Box<WaylandEvent + Send>>,
        _sender_object_id: u32,
        wl_registry_id: u32,
    ) -> Box<Future<Item = (), Error = ()> + Send> {
        println!("WlDisplay::get_registry({})", wl_registry_id);
        let compositor_name: u32;
        let shm_name: u32;
        let xdg_wm_base_name: u32;
        {
            let mut lock = session_state.write().unwrap();
            let registry = lock.wl_registry.clone();
            compositor_name = lock.wl_compositor.read().unwrap().name.clone();
            shm_name = lock.wl_shm.read().unwrap().name.clone();
            xdg_wm_base_name = lock.xdg_wm_base.read().unwrap().name.clone();
            lock.object_map
                .insert(wl_registry_id, WlResource::WlRegistry(registry));
        };
        Box::new(
            futures::future::ok(tx0)
                .and_then(move |tx| {
                    tx.send(Box::new(WlRegistryGlobal {
                        sender_object_id: wl_registry_id,
                        name: compositor_name,
                        interface: "wl_compositor".to_owned(),
                        version: 1,
                    }))
                })
                .and_then(move |tx| {
                    tx.send(Box::new(WlRegistryGlobal {
                        sender_object_id: wl_registry_id,
                        name: shm_name,
                        interface: "wl_shm".to_owned(),
                        version: 1,
                    }))
                })
                .and_then(move |tx| {
                    tx.send(Box::new(WlRegistryGlobal {
                        sender_object_id: wl_registry_id,
                        name: xdg_wm_base_name,
                        interface: "xdg_wm_base".to_owned(),
                        version: 2,
                    }))
                })
                .map_err(|_| ())
                .map(|_tx| ()),
        )
    }

    fn handle(
        sender_object: Arc<RwLock<Self>>,
        session_state: Arc<RwLock<SessionState>>,
        tx: tokio::sync::mpsc::Sender<Box<WaylandEvent + Send>>,
        sender_object_id: u32,
        opcode: u16,
        args: Vec<u8>,
    ) -> Box<Future<Item = (), Error = ()> + Send> {
        let mut cursor = Cursor::new(&args);
        match opcode {
            0 if args.len() >= 4 => {
                return Self::sync(
                    sender_object,
                    session_state,
                    tx,
                    sender_object_id,
                    cursor.read_u32::<NativeEndian>().unwrap(),
                );
            }
            1 if args.len() >= 4 => {
                return Self::get_registry(
                    sender_object,
                    session_state,
                    tx,
                    sender_object_id,
                    cursor.read_u32::<NativeEndian>().unwrap(),
                );
            }
            _ => {}
        }
        Box::new(
            tx.send(Box::new(WlDisplayError {
                object_id: sender_object_id,
                code: WL_DISPLAY_ERROR_INVALID_METHOD,
                message: format!(
                    "WlDisplay@{} opcode={} args={:?} not found",
                    1, opcode, args
                ),
            }))
            .map_err(|_| ())
            .map(|_tx| ()),
        )
    }
}

struct WaylandRequest {
    pub sender_object_id: u32,
    pub opcode: u16,
    pub args: Vec<u8>,
}

trait WaylandEvent {
    fn encode(&self, dst: &mut BytesMut) -> Result<(), std::io::Error>;
}

const WL_DISPLAY_ERROR_INVALID_OBJECT: u32 = 0;
const WL_DISPLAY_ERROR_INVALID_METHOD: u32 = 1;
const _WL_DISPLAY_ERROR_NO_MEMORY: u32 = 2;

struct WlDisplayError {
    object_id: u32,
    code: u32,
    message: String,
}

impl WaylandEvent for WlDisplayError {
    fn encode(&self, dst: &mut BytesMut) -> Result<(), std::io::Error> {
        let mut message = self.message.clone();
        message.push(0 as char);
        while message.len() % 4 != 0 {
            message.push(0 as char);
        }

        let total_len = 8 + 4 + 4 + 4 + message.len();
        if total_len > 0xffff {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Oops!"));
        }
        let i = dst.len();
        dst.resize(i + total_len, 0);
        NativeEndian::write_u32(&mut dst[i..], 1);
        NativeEndian::write_u32(&mut dst[i + 4..], ((total_len << 16) | 0) as u32);
        NativeEndian::write_u32(&mut dst[i + 8..], self.object_id);
        NativeEndian::write_u32(&mut dst[i + 12..], self.code); // invalid object
        NativeEndian::write_u32(&mut dst[i + 16..], message.len() as u32);
        dst[i + 20..].copy_from_slice(message.as_bytes());

        Ok(())
    }
}

struct WlShmFormat {
    sender_object_id: u32,
    format: u32,
}

impl WaylandEvent for WlShmFormat {
    fn encode(&self, dst: &mut BytesMut) -> Result<(), std::io::Error> {
        let total_len = 8 + 4;
        let i = dst.len();
        dst.resize(i + total_len, 0);
        NativeEndian::write_u32(&mut dst[i..], self.sender_object_id);
        NativeEndian::write_u32(&mut dst[i + 4..], ((total_len << 16) | 0) as u32);
        NativeEndian::write_u32(&mut dst[i + 8..], self.format);

        Ok(())
    }
}

struct WlRegistryGlobal {
    pub sender_object_id: u32,
    pub name: u32,
    pub interface: String,
    pub version: u32,
}

impl WaylandEvent for WlRegistryGlobal {
    fn encode(&self, dst: &mut BytesMut) -> Result<(), std::io::Error> {
        let mut aligned_interface = self.interface.clone();
        aligned_interface.push(0 as char);
        while aligned_interface.len() % 4 != 0 {
            aligned_interface.push(0 as char);
        }

        let total_len = 8 + 4 + 4 + aligned_interface.len() + 4;
        if total_len > 0xffff {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Oops!"));
        }

        let i = dst.len();
        dst.resize(i + total_len, 0);
        NativeEndian::write_u32(&mut dst[i..], self.sender_object_id);
        NativeEndian::write_u32(&mut dst[i + 4..], ((total_len << 16) | 0) as u32);
        NativeEndian::write_u32(&mut dst[i + 8..], self.name);
        NativeEndian::write_u32(&mut dst[i + 12..], aligned_interface.len() as u32);
        dst[i + 16..(i + 16 + aligned_interface.len())]
            .copy_from_slice(aligned_interface.as_bytes());
        NativeEndian::write_u32(&mut dst[i + 16 + aligned_interface.len()..], self.version);

        Ok(())
    }
}

struct WlCallbackDone {
    pub sender_object_id: u32,
    pub callback_data: u32,
}

impl WaylandEvent for WlCallbackDone {
    fn encode(&self, dst: &mut BytesMut) -> Result<(), std::io::Error> {
        let total_len = 8 + 4;
        let i = dst.len();
        dst.resize(i + total_len, 0);
        NativeEndian::write_u32(&mut dst[i..], self.sender_object_id);
        NativeEndian::write_u32(&mut dst[i + 4..], ((total_len << 16) | 0) as u32);
        NativeEndian::write_u32(&mut dst[i + 8..], self.callback_data);

        Ok(())
    }
}

struct WaylandCodec;

impl WaylandCodec {
    pub fn new() -> WaylandCodec {
        WaylandCodec {}
    }
}

impl tokio::codec::Encoder for WaylandCodec {
    type Item = Box<WaylandEvent + Send>;
    type Error = std::io::Error;

    fn encode(
        &mut self,
        res: Box<WaylandEvent + Send>,
        dst: &mut BytesMut,
    ) -> Result<(), Self::Error> {
        res.encode(dst)
    }
}

impl tokio::codec::Decoder for WaylandCodec {
    type Item = WaylandRequest;
    type Error = std::io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<WaylandRequest>, Self::Error> {
        let header_size = 8;
        if src.is_empty() || src.len() < header_size {
            return Ok(None);
        }

        // https://wayland.freedesktop.org/docs/html/ch04.html#sect-Protocol-Wire-Format
        let (req, message_size) = {
            let mut cursor = Cursor::new(&src);
            let sender_object_id = cursor.read_u32::<NativeEndian>().unwrap();
            let message_size_and_opcode = cursor.read_u32::<NativeEndian>().unwrap();
            let message_size = (message_size_and_opcode >> 16) as usize;
            if src.len() < message_size {
                return Ok(None);
            }

            let opcode = (0x0000ffff & message_size_and_opcode) as u16;
            if message_size < header_size {
                return Err(std::io::Error::new(std::io::ErrorKind::Other, "Oops!"));
            }
            let mut args = Vec::new();
            args.resize(message_size - header_size, 0);
            cursor.read_exact(&mut args).unwrap();
            println!(
                "decode: id={} opcode={} args={:?}",
                sender_object_id, opcode, &args
            );
            (
                WaylandRequest {
                    sender_object_id,
                    opcode,
                    args,
                },
                message_size,
            )
        };
        src.advance(message_size);
        return Ok(Some(req));
    }
}

struct SessionState {
    object_map: HashMap<u32, WlResource>,
    wl_registry: Arc<RwLock<WlRegistry>>,
    wl_compositor: Arc<RwLock<WlCompositor>>,
    wl_shm: Arc<RwLock<WlShm>>,
    xdg_wm_base: Arc<RwLock<XdgWmBase>>,
}

fn handle(
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

fn main() {
/*
    let mut server_socket = ServerSocket::bind().unwrap();
    let mut runtime = Runtime::new().unwrap();
    let wl_display = Arc::new(RwLock::new(WlDisplay {
        next_callback_data: 0,
    }));
    loop {
        let client_socket = if let Some(x) = server_socket.accept() {
            x
        } else {
            eprintln!("Oops!");
            continue;
        };
        let std_stream = unsafe {
            std::net::TcpStream::from_raw_socket(client_socket as std::os::windows::raw::SOCKET)
        };
        let stream = tokio::net::TcpStream::from_std(std_stream, &Handle::default()).unwrap();
        let session_state0 = Arc::new(RwLock::new(SessionState {
            wl_registry: Arc::new(RwLock::new(WlRegistry {})),
            wl_compositor: Arc::new(RwLock::new(WlCompositor { name: 1 })),
            wl_shm: Arc::new(RwLock::new(WlShm { name: 2 })),
            xdg_wm_base: Arc::new(RwLock::new(XdgWmBase { name: 3 })),
            object_map: HashMap::new(),
        }));
        session_state0
            .write()
            .unwrap()
            .object_map
            .insert(1, WlResource::WlDisplay(wl_display.clone()));
        let (tx0, rx0) = tokio::sync::mpsc::channel::<Box<WaylandEvent + Send>>(1000);
        let (writer0, reader0) = WaylandCodec::new().framed(stream).split();
        let output_session = rx0
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Oops!"))
            .forward(writer0)
            .map_err(|_| ())
            .and_then(|_| Ok(()));
        runtime.spawn(output_session);

        let input_session = reader0
            .for_each(move |req: WaylandRequest| {
                let session_state = session_state0.clone();
                let obj = session_state
                    .read()
                    .unwrap()
                    .object_map
                    .get(&req.sender_object_id)
                    .map(|r| r.clone());
                let tx = tx0.clone();
                let h = if let Some(o) = obj {
                    handle(
                        o,
                        session_state,
                        tx,
                        req.sender_object_id,
                        req.opcode,
                        req.args,
                    )
                } else {
                    Box::new(
                        tx.send(Box::new(WlDisplayError {
                            object_id: 1,
                            code: WL_DISPLAY_ERROR_INVALID_OBJECT,
                            message: format!(
                                "object_id={} opcode={} args={:?} not found",
                                req.sender_object_id, req.opcode, req.args
                            ),
                        }))
                        .map_err(|_| ())
                        .map(|_tx| ()),
                    )
                };
                h.map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Oops!"))
            })
            .map_err(|_| ());
        runtime.spawn(input_session);
    }*/
}
