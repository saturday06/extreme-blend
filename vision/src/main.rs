use byteorder::{ByteOrder, NativeEndian, ReadBytesExt};
use bytes::BytesMut;
use futures::future::Future;
use futures::sink::Sink;
use futures::stream::Stream;
use protocol::wayland::wl_buffer::WlBuffer;
use protocol::wayland_event::WaylandEvent;
use protocol::wayland_request::WaylandRequest;
use std::collections::HashMap;
use std::io::Cursor;
use std::io::Read;
use std::os::raw::{c_char, c_int};
use std::os::windows::io::FromRawSocket;
use std::sync::{Arc, RwLock};
use tokio::codec::Decoder;
use tokio::reactor::Handle;
use tokio::runtime::Runtime;
use winapi::shared::minwindef::MAKEWORD;
use winapi::shared::ws2def::{ADDRESS_FAMILY, AF_UNIX, SOCKADDR};
use winapi::um::winsock2::*;

mod protocol;
mod session_state;

#[repr(C)]
struct SOCKADDR_UN {
    sun_family: ADDRESS_FAMILY,
    sun_path: [c_char; 108],
}

struct ServerSocket {
    native_socket: SOCKET,
}

impl ServerSocket {
    pub fn bind() -> Option<ServerSocket> {
        let mut sockaddr = SOCKADDR_UN {
            sun_family: AF_UNIX as ADDRESS_FAMILY,
            sun_path: [0; 108],
        };

        let socket_path = "c:\\Temp\\temp.unix";
        let mut sun_path = format!("{}\0", socket_path)
            .as_bytes()
            .iter()
            .map(|c| *c as i8)
            .collect::<Vec<_>>();
        sun_path.resize(sockaddr.sun_path.len(), 0);
        sockaddr.sun_path.copy_from_slice(&sun_path);

        unsafe {
            let mut wsa_data = WSADATA::default();
            let x = WSAStartup(MAKEWORD(2, 2), &mut wsa_data);
            if x != 0 {
                return None;
            };

            let server_socket = socket(AF_UNIX, SOCK_STREAM, 0);
            if server_socket == INVALID_SOCKET {
                return None;
            }

            let _ = std::fs::remove_file(socket_path);
            let b = bind(
                server_socket,
                &sockaddr as *const SOCKADDR_UN as *const SOCKADDR,
                std::mem::size_of::<SOCKADDR_UN>() as c_int,
            );
            if b == SOCKET_ERROR {
                return None;
            }

            let l = listen(server_socket, SOMAXCONN);
            if l == SOCKET_ERROR {
                panic!("listen");
            }

            Some(ServerSocket {
                native_socket: server_socket,
            })
        }
    }

    pub fn accept(&mut self) -> Option<SOCKET> {
        let client_socket = unsafe {
            accept(
                self.native_socket,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            )
        };
        if client_socket == INVALID_SOCKET {
            None
        } else {
            Some(client_socket)
        }
    }
}

impl Drop for ServerSocket {
    fn drop(&mut self) {
        unsafe {
            closesocket(self.native_socket);
        }
    }
}

fn main() {
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
    }
}
