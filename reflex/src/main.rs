use byteorder::{ByteOrder, NativeEndian, ReadBytesExt};
use bytes::buf::BufMut;
use bytes::BytesMut;
use futures::future::{loop_fn, Future, Loop};
use futures::sink::Sink;
use futures::stream::Stream;
use std::collections::HashMap;
use std::fmt::Write;
use std::io::Cursor;
use std::io::Read;
use std::os::raw::{c_char, c_int};
use std::os::windows::io::FromRawSocket;
use tokio::codec::BytesCodec;
use tokio::io::AsyncRead;
use tokio::reactor::Handle;
use tokio::runtime::Runtime;
use tokio::sync::mpsc::Receiver;
use winapi::shared::minwindef::MAKEWORD;
use winapi::shared::ws2def::{ADDRESS_FAMILY, AF_UNIX, SOCKADDR};
use winapi::um::winsock2::*;

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

trait WaylandProtocol: Send {
    fn handle(&mut self, opcode: u16, args: Vec<u8>) -> Box<Future<Item = (), Error = ()> + Send>;
}

struct WaylandRequest {
    pub sender_object_id: u32,
    pub opcode: u16,
    pub args: Vec<u8>,
}

trait WaylandEvent: Send {
    fn encode(&self, dst: &mut BytesMut) -> Result<(), std::io::Error>;
}

struct WlDisplayError {
    pub sender_object_id: u32,
}

impl WaylandEvent for WlDisplayError {
    fn encode(&self, dst: &mut BytesMut) -> Result<(), std::io::Error> {
        let message = "invalid object\x00\x00"; // error message. 4-byte aligned.
        let total_len = 8 + 4 + 4 + 4 + message.len();
        if total_len > 0xffff {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Oops!"));
        }
        let i = dst.len();
        dst.resize(i + total_len, 0);
        NativeEndian::write_u32(&mut dst[i..], self.sender_object_id);
        NativeEndian::write_u32(&mut dst[i + 4..], ((total_len << 16) | 0) as u32);
        NativeEndian::write_u32(&mut dst[i + 8..], self.sender_object_id);
        NativeEndian::write_u32(&mut dst[i + 12..], 0); // invalid object
        NativeEndian::write_u32(&mut dst[i + 16..], message.len() as u32);
        dst[i + 20..].copy_from_slice(message.as_bytes());

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
    type Item = Box<WaylandEvent>;
    type Error = std::io::Error;

    fn encode(&mut self, res: Box<WaylandEvent>, dst: &mut BytesMut) -> Result<(), Self::Error> {
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
                "read id={} opcode={} args={:?}",
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
    counter: u32,
    object_map: HashMap<u32, Box<WaylandProtocol>>,
}

fn main() {
    let mut server_socket = ServerSocket::bind().unwrap();
    let mut runtime = Runtime::new().unwrap();
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
        let mut session_state = SessionState {
            counter: 0,
            object_map: HashMap::new(),
        };
        let (tx0, rx0) = tokio::sync::mpsc::channel::<Box<WaylandEvent>>(1000);

        let (writer0, reader0) = stream.framed(WaylandCodec::new()).split();
        let output_session = rx0
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Oops!"))
            .forward(writer0)
            .map_err(|_| ())
            .and_then(|_| Ok(()));
        runtime.spawn(output_session);

        let input_session = reader0
            .for_each(move |req: WaylandRequest| {
                let payload_buf = Vec::new();
                let opcode = 1;
                let obj = session_state.object_map.get_mut(&1);
                let h = if let Some(o) = obj {
                    o.handle(opcode, payload_buf)
                } else {
                    tx0.clone()
                        .send(Box::new(WlDisplayError {
                            sender_object_id: req.sender_object_id,
                        }))
                        .map_err(|_| ())
                        .map(|tx| ())
                        .boxed()
                };
                h.map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Oops!"))
            })
            .map_err(|_| ());
        runtime.spawn(input_session);
        /*
        let input_session = loop_fn(
            (reader0, session_state0, tx0),
            move |(reader, mut session_state, tx)| {
                // https://wayland.freedesktop.org/docs/html/ch04.html#sect-Protocol-Wire-Format
                //let session_state1 = session_state.clone();
                let process_input = futures::future::ok(())
                    .and_then(|_| {
                        let mut header_buf = Vec::new();
                        header_buf.resize(8, 0);
                        tokio::io::read_exact(reader, header_buf)
                            .map_err(|e| eprintln!("Error: {}", e))
                    })
                    .and_then(move |(reader, header_buf)| {
                        let mut cursor = Cursor::new(&header_buf);
                        let sender_object_id = cursor.read_u32::<NativeEndian>().unwrap();
                        let message_size_and_opcode = cursor.read_u32::<NativeEndian>().unwrap();
                        let message_size = (message_size_and_opcode >> 16) as usize;
                        let opcode = (0x0000ffff & message_size_and_opcode) as u16;

                        if message_size < header_buf.len() {
                            return Err(());
                        }
                        let mut payload_buf = Vec::new();
                        payload_buf.resize(message_size - header_buf.len(), 0);
                        Ok(tokio::io::read_exact(reader, payload_buf)
                            .map_err(|e| eprintln!("Error: {}", e))
                            .and_then(move |(reader, payload_buf)| {
                                println!(
                                    "id={} opcode={} {:?}",
                                    sender_object_id, opcode, payload_buf
                                );

                                let obj = session_state.object_map.get_mut(&sender_object_id);
                                let out = if let Some(o) = obj {
                                    o.handle(opcode, payload_buf)
                                } else {
                                    ObjectNotFound {}.handle(opcode, payload_buf)
                                };

                                Ok((reader, session_state, out))
                            }))
                    })
                    .and_then(|x| x)
                    .and_then(|(r, ss, out)| {
                        if let Some(o) = out {
                            tx.send(o)
                                .map_err(|_| ())
                                .map(|tx| Loop::Continue((r, ss, tx)))
                                .boxed()
                        } else {
                            futures::future::ok(Loop::Continue((r, ss, tx))).boxed()
                        }
                    });
                process_input
            },
        );
        runtime.spawn(input_session);
        */
    }

    //for stream in std_listener.incoming() {
    //    stream.unwrap();
    //}
    //let listener = mio::net::TcpListener::from_std(std_listener).unwrap();
    //mio::miow::
    //let xx = std_listener.local_addr();
    //println!("{:?}", xx);
    //let handle = Handle::default();
    //let listener = TcpListener::from_std(std_listener, &handle).expect("Oops!");
    /*
    let server = listener
        .incoming()
        .map_err(|e| eprintln!("failed to accept socket; error = {:?}", e))
        .for_each(|stream0| {
            let session = loop_fn(stream0, |stream| {
                let mut buf = Vec::new();
                buf.resize(5, 0);
                tokio::io::read_exact(stream, buf)
                    .map_err(|e| eprintln!("{}", e))
                    .map(|(stream, buf)| {
                        println!("{:?}", buf);
                        stream
                    })
                    .and_then(|stream| Ok(Loop::Continue(stream)))
            });
            tokio::spawn(session)
        });
    tokio::run(server);
    */
}
