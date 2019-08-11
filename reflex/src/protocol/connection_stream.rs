use crate::protocol::request::Request;
use byteorder::{NativeEndian, ReadBytesExt};
//use futures::future::Future;
//use futures::future::{err, ok};
use futures::stream::Stream;
//use mio::Ready;
use nix::cmsg_space;
use nix::sys::socket::*;
use nix::sys::uio::IoVec;
//use nix::unistd::{close, dup, pipe};
use std::os::unix::fs::PermissionsExt;
//use std::error::Error;
//use std::fs;
//use std::io;
use std::io::{Cursor, Read};
//use std::mem;
//use std::os::raw::c_void;
use std::os::unix::io::AsRawFd;
//use std::os::unix::io::FromRawFd;
//use std::os::unix::io::IntoRawFd;
use std::os::unix::io::RawFd;
//use std::path::Path;
//use std::time::Duration;
use nix::unistd::dup;
use std::sync::Arc;
use tokio::net::UnixStream;
use tokio::prelude::Async;
use tokio::reactor::Registration;
use std::path::Path;
use std::time::Duration;
use std::fs;

pub struct ConnectionStream {
    fd: RawFd,
    tokio_registration: tokio::reactor::Registration,
}

impl ConnectionStream {
    pub fn bind(path: String) -> ConnectionStream {
        let unix_addr = UnixAddr::new(path.as_bytes()).unwrap();
        println!("unix_addr={:?}", unix_addr);
        let sock_addr = SockAddr::Unix(unix_addr);
        println!("sa={:?} f={:?}", sock_addr, sock_addr.family());

        let fd = socket(
            AddressFamily::Unix,
            SockType::Stream,
            SockFlag::SOCK_NONBLOCK,
            None,
        ).unwrap();

        bind(fd, &sock_addr).expect("bind");
        listen(fd, 1024).expect("listen");

        let mut perms = fs::metadata(&path).unwrap().permissions();
        perms.set_mode(0700);
        fs::set_permissions(&path, perms).unwrap();

        let tokio_registration = tokio::reactor::Registration::new();
        tokio_registration
            .register(&mio::unix::EventedFd(&fd))
            .expect("register request fd");

        ConnectionStream {
            fd,
            tokio_registration,
        }
    }
}

impl Stream for ConnectionStream {
    type Item = RawFd;
    type Error = ();

    fn poll(&mut self) -> Result<Async<Option<Self::Item>>, Self::Error> {
        println!(
            "[Connection] poll");

        match self.tokio_registration.poll_read_ready() {
            Ok(Async::Ready(ready)) if ready.is_readable() => {
                println!("[Connection] read ready");
                ()
            }
            Ok(Async::Ready(_)) => {
                println!("[Connection] read ready ERROR");
                //return Ok(Async::NotReady);
            }
            Ok(Async::NotReady) => {
                println!("[Connection] read not ready");
                //return Ok(Async::NotReady);
            }
            Err(e) => {
                println!("[Connection] err {:?}", e);
                return Err(());
            }
        }

        match accept(self.fd) {
            Ok(client_fd) => {
                println!("[Connection] ready");
                return Ok(Async::Ready(Some(client_fd)));
            }
            Err(nix::Error::Sys(nix::errno::Errno::EAGAIN)) => {
                println!("[Connection] EAGAIN not ready");
                return Ok(Async::NotReady);
            }
            Err(err) => {
                println!("[Connection] err2: {:?}", err);
                return Err(());
            }
        };
    }
}
