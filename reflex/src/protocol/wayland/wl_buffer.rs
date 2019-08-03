// Copyright © 2008-2011 Kristian Høgsberg
// Copyright © 2010-2011 Intel Corporation
// Copyright © 2012-2013 Collabora, Ltd.
// 
// Permission is hereby granted, free of charge, to any person
// obtaining a copy of this software and associated documentation files
// (the "Software"), to deal in the Software without restriction,
// including without limitation the rights to use, copy, modify, merge,
// publish, distribute, sublicense, and/or sell copies of the Software,
// and to permit persons to whom the Software is furnished to do so,
// subject to the following conditions:
// 
// The above copyright notice and this permission notice (including the
// next paragraph) shall be included in all copies or substantial
// portions of the Software.
// 
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT.  IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
// BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
// ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

#[allow(unused_imports)] use byteorder::{NativeEndian, ReadBytesExt};
#[allow(unused_imports)] use futures::future::Future;
#[allow(unused_imports)] use futures::sink::Sink;
#[allow(unused_imports)] use std::io::{Cursor, Read};
#[allow(unused_imports)] use std::sync::{Arc, RwLock};

pub mod events {
    use byteorder::{ByteOrder, NativeEndian};

    // compositor releases buffer
    //
    // Sent when this wl_buffer is no longer used by the compositor.
    // The client is now free to reuse or destroy this buffer and its
    // backing storage.
    // 
    // If a client receives a release event before the frame callback
    // requested in the same wl_surface.commit that attaches this
    // wl_buffer to a surface, then the client is immediately free to
    // reuse the buffer and its backing storage, and does not need a
    // second buffer for the next surface content update. Typically
    // this is possible, when the compositor maintains a copy of the
    // wl_surface contents, e.g. as a GL texture. This is an important
    // optimization for GL(ES) compositors with wl_shm clients.
    pub struct Release {
        pub sender_object_id: u32,
    }

    impl super::super::super::event::Event for Release {
        fn encode(&self, dst: &mut bytes::BytesMut) -> Result<(), std::io::Error> {
            let total_len = 8;
            if total_len > 0xffff {
                return Err(std::io::Error::new(std::io::ErrorKind::Other, "Oops!"));
            }

            let i = dst.len();
            dst.resize(i + total_len, 0);

            NativeEndian::write_u32(&mut dst[i..], self.sender_object_id);
            NativeEndian::write_u32(&mut dst[i + 4..], ((total_len << 16) | 0) as u32);

            Ok(())
        }
    }
}

pub fn dispatch_request(request: Arc<RwLock<WlBuffer>>, session: crate::protocol::session::Session, sender_object_id: u32, opcode: u16, args: Vec<u8>) -> Box<futures::future::Future<Item = crate::protocol::session::Session, Error = ()> + Send> {
    let mut cursor = Cursor::new(&args);
    match opcode {
        0 => {
            return WlBuffer::destroy(request, session, sender_object_id, )
        },
        _ => {},
    };
    Box::new(futures::future::ok(session))
}

// content for a wl_surface
//
// A buffer provides the content for a wl_surface. Buffers are
// created through factory interfaces such as wl_drm, wl_shm or
// similar. It has a width and a height and can be attached to a
// wl_surface, but the mechanism by which a client provides and
// updates the contents is defined by the buffer factory interface.
pub struct WlBuffer {
}

impl WlBuffer {
    // destroy a buffer
    //
    // Destroy a buffer. If and how you need to release the backing
    // storage is defined by the buffer factory interface.
    // 
    // For possible side-effects to a surface, see wl_surface.attach.
    pub fn destroy(
        request: Arc<RwLock<WlBuffer>>,
        session: crate::protocol::session::Session,
        sender_object_id: u32,
    ) -> Box<futures::future::Future<Item = crate::protocol::session::Session, Error = ()> + Send> {
        Box::new(futures::future::ok(session))
    }
}
