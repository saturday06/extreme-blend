use crate::protocol::wayland_event::WaylandEvent;
use crate::protocol::wayland_request::WaylandRequest;
use crate::protocol::wl_resource::WlResource;
use crate::session_state::SessionState;
use futures::future::Future;
use std::sync::{Arc, RwLock};
use bytes::BytesMut;
use std::io::Cursor;
use byteorder::NativeEndian;

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
