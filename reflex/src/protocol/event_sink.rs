use crate::protocol::event::Event;
use byteorder::{NativeEndian, ReadBytesExt};
use futures::future::Future;
use futures::future::{err, ok};
use futures::sink::Sink;
use nix::sys::socket::*;
use nix::sys::uio::IoVec;
use nix::unistd::{close, dup, pipe};
use std::error::Error;
use std::fs;
use std::io;
use std::io::{Cursor, Read};
use std::os::raw::c_void;
use std::os::unix::io::AsRawFd;
use std::os::unix::io::RawFd;
use tokio::net::{UnixListener, UnixStream};
use tokio::prelude::{Async, Poll};
use std::sync::Arc;
use futures::AsyncSink;
use bytes::BytesMut;

pub struct EventSink {
    fd: RawFd,
    _tokio_stream: Arc<UnixStream>,
    tokio_registration: tokio::reactor::Registration,
    pending_bytes: Vec<u8>,
    pending_fds: Vec<RawFd>,
    pending_events: Vec<Box<Event + Send>>,
    closed: bool,
}

impl EventSink {
    pub(crate) fn new(tokio_stream: Arc<UnixStream>) -> EventSink {
        let tokio_registration = tokio::reactor::Registration::new();
        let fd2 = tokio_stream.as_raw_fd();
        let fd = dup(fd2).expect("dup2");
        tokio_registration
            .register(&mio::unix::EventedFd(&fd))
            .expect("register request fd");
        EventSink {
            fd,
            _tokio_stream: tokio_stream,
            tokio_registration,
            pending_bytes: Vec::new(),
            pending_fds: Vec::new(),
            pending_events: Vec::new(),
            closed: false,
        }
    }
}

impl Sink for EventSink {
    type SinkItem = Box<Event + Send>;
    type SinkError = std::io::Error;

    fn start_send(&mut self, item: Self::SinkItem) -> Result<AsyncSink<Self::SinkItem>, Self::SinkError> {
        println!("START SEND");
        self.pending_events.push(item);
        Ok(AsyncSink::Ready)
    }

    fn poll_complete(&mut self) -> Result<Async<()>, Self::SinkError> {
        println!("POLL COMPLETE");
        match self.tokio_registration.poll_write_ready() {
            Ok(Async::Ready(ready)) if ready.is_writable() => {
                println!("write ready");
                ()
            }
            Ok(Async::Ready(_)) => {
                println!("write ready 2");
                return Ok(Async::NotReady);
            }
            Ok(Async::NotReady) => {
                println!("write not ready");
                return Ok(Async::NotReady);
            }
            Err(err) => {
                println!("err {:?}", err);
                return Err(std::io::Error::new(std::io::ErrorKind::Other, err));
            }
        }

        let mut bytes = BytesMut::new();
        if self.pending_bytes.len() == 0 && self.pending_fds.len() == 0 {
            if self.pending_events.len() == 0 {
                return Ok(Async::Ready(()));
            } else if let Some(event) = self.pending_events.split_off(1).pop() {
                match event.encode(&mut bytes) {
                    Ok(()) => {},
                    Err(err) => {
                        println!("err3 {:?}", err);
                        return Err(std::io::Error::new(std::io::ErrorKind::Other, err));
                    }
                }
            } else {
                return Ok(Async::Ready(()));
            }
        }

        self.pending_bytes.extend(bytes);
        println!("WRITE: {:?}", &self.pending_bytes);
        let sent_bytes = match send(self.fd, &self.pending_bytes[..], MsgFlags::empty()) {
            Ok(sent_bytes) => sent_bytes,
            Err(nix::Error::Sys(nix::errno::Errno::EAGAIN)) => return Ok(Async::NotReady),
            Err(err) => {
                println!("err2 {:?}", err);
                return Err(std::io::Error::new(std::io::ErrorKind::Other, err));
            },
        };

        if self.pending_bytes.len() > sent_bytes {
            self.pending_bytes.clear();
        } else {
            let _ = self.pending_bytes.split_off(sent_bytes);
        }

        return Ok(Async::Ready(()));
    }

    fn close(&mut self) -> Result<Async<()>, Self::SinkError> {
        self.closed = true;
        Ok(Async::Ready(()))
    }
}
