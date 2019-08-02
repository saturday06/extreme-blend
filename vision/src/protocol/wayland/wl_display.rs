use crate::protocol::wayland_event::WaylandEvent;
use crate::protocol::wayland_request::WaylandRequest;
use crate::protocol::wl_registry::WlRegistryGlobal;
use crate::protocol::wl_resource::WlResource;
use crate::session_state::SessionState;
use byteorder::NativeEndian;
use bytes::BytesMut;
use futures::future::Future;
use std::io::Cursor;
use std::sync::{Arc, RwLock};

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

pub struct WlDisplay {
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
