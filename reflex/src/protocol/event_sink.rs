use crate::protocol::event::Event;
use byteorder::{NativeEndian, ReadBytesExt};
use bytes::BytesMut;
use futures::future::Future;
use futures::future::{err, ok};
use futures::sink::Sink;
use futures::AsyncSink;
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
use std::sync::Arc;
use tokio::net::{UnixListener, UnixStream};
use tokio::prelude::{Async, Poll};

pub struct EventSink {
    fd: RawFd,
    _tokio_stream: Arc<UnixStream>,
    tokio_registration: Arc<tokio::reactor::Registration>,
    pending_bytes: Vec<u8>,
    pending_fds: Vec<RawFd>,
    pending_events: Vec<Box<Event + Send>>,
    closed: bool,
}

impl EventSink {
    pub(crate) fn new(
        tokio_stream: Arc<UnixStream>,
        fd: RawFd,
        tokio_registration: Arc<tokio::reactor::Registration>,
    ) -> EventSink {
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

    fn start_send(
        &mut self,
        item: Self::SinkItem,
    ) -> Result<AsyncSink<Self::SinkItem>, Self::SinkError> {
        self.pending_events.push(item);
        println!("[Sink] start_send events.len={}", self.pending_events.len());
        Ok(AsyncSink::Ready)
    }

    fn poll_complete(&mut self) -> Result<Async<()>, Self::SinkError> {
        loop {
            println!("[Sink] poll complete events.len={}", self.pending_events.len());
            match self.tokio_registration.poll_write_ready() {
                Ok(Async::Ready(ready)) if ready.is_writable() => {
                    println!("[Sink] write ready");
                    ()
                }
                Ok(Async::Ready(_)) => {
                    println!("[Sink] write ready ERROR");
                    // return Ok(Async::NotReady);
                }
                Ok(Async::NotReady) => {
                    println!("[Sink] write not ready");
                    //return Ok(Async::NotReady);
                }
                Err(err) => {
                    println!("[Sink] write ERROR {:?}", err);
                    return Err(std::io::Error::new(std::io::ErrorKind::Other, err));
                }
            }

            let mut bytes = BytesMut::new();
            if self.pending_bytes.len() == 0 && self.pending_fds.len() == 0 {
                println!("[Sink] pending bytes is empty");
                if self.pending_events.len() == 0 {
                    println!("[Sink] pending event is empty");
                    return Ok(Async::Ready(()));
                } else {
                    let rest = self.pending_events.split_off(1);
                    let first = self.pending_events.pop().expect("pending events error");
                    self.pending_events = rest;

                    match first.encode(&mut bytes) {
                        Ok(()) => {
                            println!("[Sink] encoded");
                        }
                        Err(err) => {
                            println!("[Sink] err3 {:?}", err);
                            return Err(std::io::Error::new(std::io::ErrorKind::Other, err));
                        }
                    }
                }
            }

            self.pending_bytes.extend(bytes);
            println!("[Sink] write {:?}", &self.pending_bytes);
            let sent_bytes = match send(self.fd, &self.pending_bytes[..], MsgFlags::empty()) {
                Ok(sent_bytes) => sent_bytes,
                Err(nix::Error::Sys(nix::errno::Errno::EAGAIN)) => return Ok(Async::NotReady),
                Err(err) => {
                    println!("[Sink] err2 {:?}", err);
                    return Err(std::io::Error::new(std::io::ErrorKind::Other, err));
                }
            };

            println!("[Sink] write {} bytes written", sent_bytes);
            if self.pending_bytes.len() <= sent_bytes {
                self.pending_bytes.clear();
            } else if sent_bytes > 0 {
                self.pending_bytes = self.pending_bytes.split_off(sent_bytes);
            }
            println!("[Sink] new pending_bytes={:?}", self.pending_bytes);
        }
    }

    fn close(&mut self) -> Result<Async<()>, Self::SinkError> {
        self.closed = true;
        Ok(Async::Ready(()))
    }
}
