use futures::future::{loop_fn, ok, Future, FutureResult, Loop};
use futures::stream::Stream;
use std::os::raw::{c_char, c_int};
use std::os::windows::io::FromRawSocket;
//use std::os::windows::raw::SOCKET;
//use tokio::net::{TcpListener, TcpStream};
use std::os::windows::io::FromRawHandle;
use tokio::reactor::Handle;
use tokio::runtime::Runtime;
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
        let stream0 = tokio::net::TcpStream::from_std(std_stream, &Handle::default()).unwrap();
        let session = loop_fn(stream0, |stream| {
            let mut header_buf0 = Vec::new();
            header_buf0.resize(8, 0);

            let mut payload_buf0 = Vec::new();
            payload_buf0.resize(8, 0);

            tokio::io::read_exact(stream, header_buf0)
                .map_err(|e| eprintln!("Error: {}", e))
                .and_then(|(stream, buf)| {
                    tokio::io::read_exact(stream, payload_buf0)
                        .map_err(|e| eprintln!("Error: {}", e))
                })
                .map(|(stream, buf)| {
                    println!("{:?}", buf);
                    stream
                })
                .and_then(|stream| Ok(Loop::Continue(stream)))
        });
        runtime.spawn(session);
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
