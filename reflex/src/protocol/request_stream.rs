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
use nix::fcntl::fcntl;
use nix::fcntl::FcntlArg::F_GETFD;
use nix::unistd::dup;
use std::sync::Arc;
use tokio::net::UnixStream;
use tokio::prelude::Async;
use tokio::reactor::Registration;

pub struct RequestStream {
    fd: RawFd,
    tokio_registration: Arc<tokio::reactor::Registration>,
    pending_bytes: Vec<u8>,
    pending_fds: Vec<RawFd>,
    pending_requests: Vec<Request>,
}

impl RequestStream {
    pub(crate) fn new(fd: RawFd, tokio_registration: Arc<Registration>) -> RequestStream {
        RequestStream {
            fd,
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
        println!(
            "[Stream] poll: pending_requests={}",
            self.pending_requests.len()
        );

        if self.pending_requests.len() > 0 {
            let rest = self.pending_requests.split_off(1);
            let first = self.pending_requests.pop().expect("pending requests error");
            self.pending_requests = rest;
            println!("[Stream] return pending request {:?}", first);
            return Ok(Async::Ready(Some(first)));
        }

        match self.tokio_registration.poll_read_ready() {
            Ok(Async::Ready(ready)) if ready.is_readable() => {
                println!("[Stream] read ready");
                ()
            }
            Ok(Async::Ready(_)) => {
                println!("[Stream] read ready ERROR");
                //return Ok(Async::NotReady);
            }
            Ok(Async::NotReady) => {
                println!("[Stream] read not ready");
                //return Ok(Async::NotReady);
            }
            Err(e) => {
                println!("[Stream] err {:?}", e);
                return Err(());
            }
        }

        let mut received_fds: Vec<RawFd> = Vec::new();
        let mut buf: Vec<u8> = Vec::new();
        buf.resize(16, 0);
        loop {
            println!("[Stream] ----- read loop -----");
            let buf_len = buf.len();
            let iov = [IoVec::from_mut_slice(&mut buf[..])];
            let mut cmsgspace = cmsg_space!([RawFd; 8]);

            let flags = fcntl(self.fd, F_GETFD);
            println!("flags={:?}", flags);
            let poll_msg = match recvmsg(self.fd, &iov, Some(&mut cmsgspace), MsgFlags::MSG_PEEK) {
                Ok(ok) => ok,
                Err(nix::Error::Sys(nix::errno::Errno::EAGAIN)) => {
                    println!("[Stream] EAGAIN not ready");
                    return Ok(Async::NotReady);
                }
                Err(err) => {
                    println!("[Stream] err1: {:?}", err);
                    return Err(());
                }
            };
            println!(
                "[Stream] msg flag={:?} bytes={}",
                poll_msg.flags, poll_msg.bytes
            );
            if poll_msg.flags.intersects(MsgFlags::MSG_TRUNC) {
                buf.resize(buf_len * 2, 0);
                println!("[Stream] msg_trunc {} -> {}", buf_len, buf.len());
                continue;
            }
            if poll_msg.bytes == buf_len {
                buf.resize(buf_len * 2, 0);
                println!("[Stream] msg_trunc WORKAROUND {} -> {}", buf_len, buf.len());
                continue;
            }
            if poll_msg.flags.intersects(MsgFlags::MSG_CTRUNC) {
                println!("[Stream] ctrunc ERROR");
                return Err(());
            }

            let flags = fcntl(self.fd, F_GETFD);
            println!("[Stream] flags={:?}", flags);
            let msg = match recvmsg(self.fd, &iov, Some(&mut cmsgspace), MsgFlags::empty()) {
                Ok(ok) => ok,
                Err(nix::Error::Sys(nix::errno::Errno::EAGAIN)) => {
                    println!("[Stream] EAGAIN not ready");
                    return Ok(Async::NotReady);
                }
                Err(err) => {
                    println!("[Stream] err2: {:?}", err);
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

        println!("[Stream] data={:?} fds={:?}", buf, received_fds);
        self.pending_bytes.extend(buf);
        self.pending_fds.extend(received_fds);

        let header_size = 8;
        loop {
            if self.pending_bytes.len() < header_size {
                println!(
                    "[Stream] break pending_bytes={} < header_size={}",
                    self.pending_bytes.len(),
                    header_size
                );
                break;
            }

            // https://wayland.freedesktop.org/docs/html/ch04.html#sect-Protocol-Wire-Format
            let mut cursor = Cursor::new(&self.pending_bytes);
            let sender_object_id = cursor.read_u32::<NativeEndian>().unwrap();
            let message_size_and_opcode = cursor.read_u32::<NativeEndian>().unwrap();
            let message_size = (message_size_and_opcode >> 16) as usize;
            if self.pending_bytes.len() < message_size {
                println!(
                    "[Stream] break pending_bytes={} < message_size={}",
                    self.pending_bytes.len(),
                    message_size
                );
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
                "[Stream] decode: id={} opcode={} args={:?}",
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

        if self.pending_requests.is_empty() {
            println!("[Stream] pending request is empty");
            return Ok(Async::NotReady);
        }

        {
            let rest = self.pending_requests.split_off(1);
            let first = self.pending_requests.pop().expect("pending requests error");
            self.pending_requests = rest;
            println!("[Stream] ok {:?}", first);
            return Ok(Async::Ready(Some(first)));
        }
    }
}
