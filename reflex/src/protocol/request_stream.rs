use crate::protocol::request::Request;
use byteorder::{NativeEndian, ReadBytesExt};
use futures::stream::Stream;
use std::io::{Cursor, Read};
use std::os::unix::io::RawFd;
use std::sync::Arc;
use tokio::prelude::Async;
use tokio::reactor::Registration;
use crate::protocol::fd_drop::FdDrop;

pub struct RequestStream {
    fd: RawFd,
    _fd_drop: Arc<FdDrop>,
    tokio_registration: Arc<tokio::reactor::Registration>,
    //_tokio_stream: Arc<UnixStream>,
    pending_bytes: Vec<u8>,
    pending_fds: Vec<RawFd>,
    pending_requests: Vec<Request>,
}

impl RequestStream {
    pub(crate) fn new(fd: RawFd,   fd_drop: Arc<FdDrop>, tokio_registration: Arc<Registration>) -> RequestStream {
        RequestStream {
            fd,
            _fd_drop: fd_drop,
            tokio_registration,
            //_tokio_stream: tokio_stream,
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
        let mut fd_len = 8;
        loop {
            let buf_len = buf.len();

            unsafe {
                let cmsg_space = std::cmp::max(
                    libc::CMSG_SPACE(std::mem::size_of::<std::os::raw::c_int>() as u32 * fd_len),
                    std::mem::size_of::<libc::cmsghdr>() as u32
                );
                {
                    let mut msg_control: Vec<u8> = Vec::new();
                    msg_control.resize(cmsg_space as usize, 0);

                    let mut io_vec = libc::iovec {
                        iov_len: buf.len(),
                        iov_base: buf.as_mut_ptr() as *mut std::os::raw::c_void,
                    };
                    let mut msg_hdr = libc::msghdr {
                        msg_name: std::ptr::null_mut(),
                        msg_namelen: 0,
                        msg_iov: &mut io_vec,
                        msg_iovlen: 1,
                        msg_control: msg_control.as_mut_ptr() as *mut std::os::raw::c_void,
                        msg_controllen: msg_control.len(),
                        msg_flags: 0,
                    };
                    let read = libc::recvmsg(self.fd, &mut msg_hdr, libc::MSG_PEEK);
                    if read < 0 {
                        let errno = *libc::__errno_location();
                        if errno == libc::EAGAIN || errno == libc::EWOULDBLOCK {
                            println!("[Stream] EAGAIN not ready");
                            return Ok(Async::NotReady);
                        }
                        println!("[Stream] err {}", errno);
                        return Err(());
                    }
                    if (msg_hdr.msg_flags & libc::MSG_TRUNC) != 0 {
                        buf.resize(buf.len() * 2, 0);
                        continue;
                    }
                    if (msg_hdr.msg_flags & libc::MSG_CTRUNC) != 0 {
                        fd_len = fd_len * 2;
                        continue;
                    }
                    if read as usize == buf_len {
                        buf.resize(buf.len() * 2, 0);
                        continue;
                    }
                }
                {
                    let mut msg_control: Vec<u8> = Vec::new();
                    msg_control.resize(cmsg_space as usize, 0);

                    let mut io_vec = libc::iovec {
                        iov_len: buf.len(),
                        iov_base: buf.as_mut_ptr() as *mut std::os::raw::c_void,
                    };
                    let mut msg_hdr = libc::msghdr {
                        msg_name: std::ptr::null_mut(),
                        msg_namelen: 0,
                        msg_iov: &mut io_vec,
                        msg_iovlen: 1,
                        msg_control: msg_control.as_mut_ptr() as *mut std::os::raw::c_void,
                        msg_controllen: msg_control.len(),
                        msg_flags: 0,
                    };
                    let read = libc::recvmsg(self.fd, &mut msg_hdr, 0);
                    if read < 0 {
                        let errno = *libc::__errno_location();
                        if errno == libc::EAGAIN || errno == libc::EWOULDBLOCK {
                            println!("[Stream] EAGAIN not ready");
                            return Ok(Async::NotReady);
                        }
                        println!("[Stream] err {}", errno);
                        return Err(());
                    }
                    if (msg_hdr.msg_flags & libc::MSG_TRUNC) != 0 {
                        println!("[Stream] Oops MSG_TRUNC");
                    }
                    if (msg_hdr.msg_flags & libc::MSG_CTRUNC) != 0 {
                        println!("[Stream] Oops MSG_CTRUNC");
                    }

                    buf.resize(read as usize, 0);

                    let mut cmsg_hdr = libc::CMSG_FIRSTHDR(&msg_hdr);
                    while cmsg_hdr != std::ptr::null_mut() {
                        println!("[Stream] cmsg");
                        let cmsg_level = (*cmsg_hdr).cmsg_level;
                        let cmsg_type = (*cmsg_hdr).cmsg_type;
                        if cmsg_level == libc::SOL_SOCKET &&cmsg_type == libc::SCM_RIGHTS {
                            println!("[Stream] SCM_RIGHTS cmsg_len={}", (*cmsg_hdr).cmsg_len);
                            let received_fds_ptr = libc::CMSG_DATA(cmsg_hdr) as *mut std::os::raw::c_int;
                            let mut received_fds_len = 1;
                            while libc::CMSG_LEN(std::mem::size_of::<std::os::raw::c_int>() as u32 * received_fds_len) <= (*cmsg_hdr).cmsg_len as u32 {
                                println!("[Stream] SCM_RIGHTS  vs {}",
                                libc::CMSG_LEN(std::mem::size_of::<std::os::raw::c_int>() as u32 * received_fds_len));
                                received_fds_len += 1;
                            }
                            received_fds_len -= 1;
                            println!("[Stream] len={}", received_fds_len);
                            for offset in 0..received_fds_len {
                                received_fds.push(*received_fds_ptr.offset(offset as isize));
                            }
                        } else {
                            println!("[Stream] UNHANDLED CMSG: level={} type={}", cmsg_level, cmsg_type);
                        }
                        cmsg_hdr = libc::CMSG_NXTHDR(&msg_hdr, cmsg_hdr);
                    }
                }
            }

            /*
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
            */
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
                "[Stream] decode: id={} opcode={} args={:?} fds={:?}",
                sender_object_id, opcode, &args, &self.pending_fds
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
