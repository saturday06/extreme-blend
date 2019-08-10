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
use tokio::net::UnixStream;
use tokio::prelude::Async;
use std::sync::Arc;
use nix::unistd::dup;

pub struct RequestStream {
    fd: RawFd,
    _tokio_stream: Arc<UnixStream>,
    tokio_registration: tokio::reactor::Registration,
    pending_bytes: Vec<u8>,
    pending_fds: Vec<RawFd>,
    pending_requests: Vec<Request>,
}

impl RequestStream {
    pub(crate) fn new(tokio_stream: Arc<UnixStream>) -> RequestStream {
        let tokio_registration = tokio::reactor::Registration::new();
        let fd2 = tokio_stream.as_raw_fd();
        let fd = dup(fd2).expect("dup2");
        tokio_registration
            .register(&mio::unix::EventedFd(&fd))
            .expect("register request fd");
        RequestStream {
            fd,
            _tokio_stream: tokio_stream,
            tokio_registration,
            pending_bytes: Vec::new(),
            pending_fds: Vec::new(),
            pending_requests: Vec::new(),
        }
    }
}

impl Stream for RequestStream {
    type Item = Request;
    type Error = ();

    fn poll(&mut self) -> Result<Async<Option<Self::Item>>, Self::Error> {
        if let Some(r) = self.pending_requests.pop() {
            return Ok(Async::Ready(Some(r)));
        }
        match self.tokio_registration.poll_read_ready() {
            Ok(Async::Ready(ready)) if ready.is_readable() => {
                println!("read ready");
                ()
            }
            Ok(Async::Ready(_)) => {
                println!("read ready 2");
                return Ok(Async::NotReady);
            }
            Ok(Async::NotReady) => {
                println!("read not ready");
                return Ok(Async::NotReady);
            }
            Err(e) => {
                println!("err {:?}", e);
                return Err(());
            }
        }

        let mut received_fds: Vec<RawFd> = Vec::new();
        let mut buf: Vec<u8> = Vec::new();
        buf.resize(16, 0);
        loop {
            println!("----- loop -----");
            let buf_len = buf.len();
            let iov = [IoVec::from_mut_slice(&mut buf[..])];
            let mut cmsgspace = cmsg_space!([RawFd; 8]);

            let poll_msg = match recvmsg(self.fd, &iov, Some(&mut cmsgspace), MsgFlags::MSG_PEEK) {
                Ok(ok) => ok,
                Err(nix::Error::Sys(nix::errno::Errno::EAGAIN)) => return Ok(Async::NotReady),
                Err(err) => {
                    println!("err1: {:?}", err);
                    return Err(());
                }
            };
            println!("msg flag={:?} bytes={}", poll_msg.flags, poll_msg.bytes);
            if poll_msg.flags.intersects(MsgFlags::MSG_TRUNC) {
                println!("msg_trunc");
                buf.resize(buf_len * 2, 0);
                continue;
            }
            if poll_msg.bytes == buf_len {
                println!("msg_trunc 2");
                buf.resize(buf_len * 2, 0);
                continue;
            }
            if poll_msg.flags.intersects(MsgFlags::MSG_CTRUNC) {
                println!("msg_ctrunc");
                return Err(());
            }

            let msg = match recvmsg(self.fd, &iov, Some(&mut cmsgspace), MsgFlags::empty()) {
                Ok(ok) => ok,
                Err(nix::Error::Sys(nix::errno::Errno::EAGAIN)) => return Ok(Async::NotReady),
                Err(err) => {
                    println!("err2: {:?}", err);
                    return Err(());
                }
            };

            for cmsg in msg.cmsgs() {
                if let ControlMessageOwned::ScmRights(fds) = cmsg {
                    received_fds.extend(fds);
                }
            }
            //assert_eq!(msg.bytes, 5);
            assert!(!msg
                .flags
                .intersects(MsgFlags::MSG_TRUNC | MsgFlags::MSG_CTRUNC));
            buf.resize(msg.bytes, 0);
            break;
        }

        println!("data={:?} fds={:?}", buf, received_fds);
        self.pending_bytes.extend(buf);
        self.pending_fds.extend(received_fds);

        let header_size = 8;
        loop {
            if self.pending_bytes.len() < header_size {
                break;
            }

            // https://wayland.freedesktop.org/docs/html/ch04.html#sect-Protocol-Wire-Format
            let mut cursor = Cursor::new(&self.pending_bytes);
            let sender_object_id = cursor.read_u32::<NativeEndian>().unwrap();
            let message_size_and_opcode = cursor.read_u32::<NativeEndian>().unwrap();
            let message_size = (message_size_and_opcode >> 16) as usize;
            if self.pending_bytes.len() < message_size {
                break;
            }

            let opcode = (0x0000ffff & message_size_and_opcode) as u16;
            if message_size < header_size {
                return Err(());
            }
            let mut args = Vec::new();
            args.resize(message_size - header_size, 0);
            cursor.read_exact(&mut args).unwrap();
            println!(
                "decode: id={} opcode={} args={:?}",
                sender_object_id, opcode, &args
            );
            let fds = self.pending_fds.clone();
            self.pending_fds.clear();
            self.pending_bytes = self.pending_bytes.split_off(message_size);
            self.pending_requests.push(Request {
                sender_object_id,
                opcode,
                args,
                fds,
            });
        }

        if self.pending_requests.len() == 0 {
            return Ok(Async::NotReady);
        } else    if let Some(r) = self.pending_requests.split_off(1).pop() {
            return Ok(Async::Ready(Some(r)));
        } else {
            return Ok(Async::NotReady);
        }
    }
}
