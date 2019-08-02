use crate::protocol::wayland_event::WaylandEvent;
use crate::protocol::wayland_request::WaylandRequest;
use crate::protocol::wayland::wl_shm::WlShmFormat;
use crate::protocol::wl_resource::WlResource;
use crate::session_state::SessionState;
use byteorder::NativeEndian;
use bytes::BytesMut;
use futures::future::Future;
use std::io::Cursor;
use std::sync::{Arc, RwLock};

pub struct WlRegistryGlobal {
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

pub struct WlRegistry {}

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
