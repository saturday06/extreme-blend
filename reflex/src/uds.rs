use mio::event::Evented;
use mio::{Poll, PollOpt, Ready, Registration, Token};

use std::io;
use std::thread;
use std::time::{Instant, Duration};
use std::ops::Add;
use futures::future::Future;
use futures::future::lazy;
use std::net::Shutdown;
use std::net::SocketAddr;
use tokio::net::TcpStream;
use tokio::prelude::*;



pub struct Deadline {
    when: Instant,
    mio_registration: mio::Registration,
}

impl Deadline {
    pub fn new(when: Instant) -> Deadline {
        let (registration, set_readiness) = Registration::new2();
        set_readiness.set_readiness(Ready::empty());

        thread::spawn(move || {
            println!("waiting");
            let now = Instant::now();

            if now < when {
                thread::sleep(when - now);
            }

            println!("set readiness");
            set_readiness.set_readiness(Ready::readable());
        });
        let d = Deadline {
            when: when,
            mio_registration: registration,
        };
        d
    }

    pub fn is_elapsed(&self) -> bool {
        Instant::now() >= self.when
    }
}

impl Evented for Deadline {
    fn register(
        &self,
        poll: &Poll,
        token: Token,
        interest: Ready,
        opts: PollOpt,
    ) -> io::Result<()> {
        self.mio_registration.register(poll, token, interest, opts)
    }

    fn reregister(
        &self,
        poll: &Poll,
        token: Token,
        interest: Ready,
        opts: PollOpt,
    ) -> io::Result<()> {
        self.mio_registration.reregister(poll, token, interest, opts)
    }

    fn deregister(&self, poll: &Poll) -> io::Result<()> {
        self.mio_registration.deregister(poll)
    }
}

struct Deadline2 {
    tokio_registration: tokio::reactor::Registration,
    deadline: Deadline,
}

impl Deadline2 {
    fn new() -> Deadline2 {
        let d = Deadline::new(Instant::now().add(Duration::from_millis(5000)));
        let r = tokio::reactor::Registration::new();
        r.register(&d);

        Deadline2 {
            deadline: d,
            tokio_registration: r,
        }
    }
}

impl Future for Deadline2 {
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Result<Async<()>, ()> {
        match self.tokio_registration.poll_read_ready() {
            Ok(Async::Ready(socket)) => {
               // println!("peer address = {}", socket.peer_addr().unwrap());
                Ok(Async::Ready(()))
            }
            Ok(Async::NotReady) => Ok(Async::NotReady),
            Err(e) => {
                println!("failed to connect: {}", e);
                Ok(Async::Ready(()))
            }
        }
    }
}

#[test]
fn it_works() {
    let mut dea = Deadline2::new();

    println!("start");
    /*
    tokio::run(lazy(|| {
        panic!("oops");

        for i in 0..4 {
            tokio::spawn(lazy(move || {
                eprintln!("Hello from task {}", i);
                Ok(())
            }));
        }
        Ok(())
    }));
    tokio::run(futures::future::ok(reg)
        .map(|r| {println!("ok1"); r})
        .and_then(|r| {println!("ok2"); Ok(r)})
        .and_then(|r| r.poll_read_ready())
        .map(|_| println!("ok3"))
        .map_err(|_: std::io::Error| ()));
*/
    tokio::run(dea);
    println!("end");
    assert!(false);
}
