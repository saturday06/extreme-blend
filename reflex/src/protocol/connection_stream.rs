use futures::stream::Stream;
use nix::sys::socket::*;
use std::os::unix::fs::PermissionsExt;
use std::os::unix::io::RawFd;
use std::fs;
use tokio::prelude::Async;

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
        )
        .unwrap();

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
    type Error = std::io::Error;

    fn poll(&mut self) -> Result<Async<Option<Self::Item>>, Self::Error> {
        println!("[Connection] poll");

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
            Err(err) => {
                println!("[Connection] err {:?}", err);
                return Err(std::io::Error::new(std::io::ErrorKind::Other, err));
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
                return Err(std::io::Error::new(std::io::ErrorKind::Other, err));
            }
        };
    }
}
