use crate::protocol::wayland_event::WaylandEvent;
use crate::protocol::wayland_request::WaylandRequest;
use crate::protocol::wl_resource::WlResource;
use crate::session_state::SessionState;
use futures::future::Future;
use std::sync::{Arc, RwLock};
use byteorder::NativeEndian;
use bytes::BytesMut;
use std::io::Cursor;

pub struct WlShm {
    name: u32,
}

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

pub struct WlShmFormat {
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
