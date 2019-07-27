use futures::future::{loop_fn, ok, Future, FutureResult, Loop};
use futures::stream::Stream;
use std::os::raw::{c_char, c_int};
use std::os::windows::io::FromRawSocket;
use std::os::windows::raw::SOCKET;
//use tokio::net::{TcpListener, TcpStream};
use tokio::reactor::Handle;
use winapi::shared::minwindef::MAKEWORD;
use winapi::shared::ws2def::{ADDRESS_FAMILY, AF_UNIX, SOCKADDR};
use winapi::um::winsock2::*;

#[repr(C)]
struct SOCKADDR_UN {
    sun_family: ADDRESS_FAMILY,
    sun_path: [c_char; 108],
}

fn main() {
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

    let std_listener = unsafe {
        let mut wsa_data = WSADATA::default();
        let x = WSAStartup(MAKEWORD(2, 2), &mut wsa_data);
        if x != 0 {
            panic!(format!("WSAStartup: {}", x));
        };

        let server_socket = socket(AF_UNIX, SOCK_STREAM, 0);
        if server_socket == INVALID_SOCKET {
            panic!("invalid socket");
        }

        let _ = std::fs::remove_file(socket_path);
        let b = bind(
            server_socket,
            &sockaddr as *const SOCKADDR_UN as *const SOCKADDR,
            std::mem::size_of::<SOCKADDR_UN>() as c_int,
        );
        if b == SOCKET_ERROR {
            panic!("bind");
        }

        let l = listen(server_socket, SOMAXCONN);
        if l == SOCKET_ERROR {
            panic!("listen");
        }

        /*
        let client_socket = accept(server_socket, std::ptr::null_mut(), std::ptr::null_mut());
        if client_socket == INVALID_SOCKET {
            panic!("client socket");
        }
        */
        std::net::TcpListener::from_raw_socket(server_socket as SOCKET)
        //closesocket(server_socket);
    };
    let listener = mio::net::TcpListener::from_std(std_listener).unwrap();
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
