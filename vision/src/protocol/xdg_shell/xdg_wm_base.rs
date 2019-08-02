use crate::protocol::wayland_event::WaylandEvent;
use crate::protocol::wayland_request::WaylandRequest;
use crate::protocol::wl_resource::WlResource;
use crate::session_state::SessionState;
use byteorder::NativeEndian;
use bytes::BytesMut;
use futures::future::Future;
use std::io::Cursor;
use std::sync::{Arc, RwLock};

pub struct XdgWmBase {
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
