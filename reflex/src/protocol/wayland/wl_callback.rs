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

#[allow(unused_imports)]
use byteorder::{NativeEndian, ReadBytesExt};
#[allow(unused_imports)]
use futures::future::Future;
#[allow(unused_imports)]
use futures::sink::Sink;
#[allow(unused_imports)]
use std::io::{Cursor, Read};
#[allow(unused_imports)]
use std::sync::{Arc, RwLock};

pub mod events {
    use byteorder::{ByteOrder, NativeEndian};

    // done event
    //
    // Notify the client when the related request is done.
    pub struct Done {
        pub sender_object_id: u32,
        pub callback_data: u32, // uint: request-specific data for the callback
    }

    impl super::super::super::event::Event for Done {
        fn encode(&self, dst: &mut bytes::BytesMut) -> Result<(), std::io::Error> {
            let total_len = 8 + 4;
            if total_len > 0xffff {
                return Err(std::io::Error::new(std::io::ErrorKind::Other, "Oops!"));
            }

            let i = dst.len();
            dst.resize(i + total_len, 0);

            NativeEndian::write_u32(&mut dst[i..], self.sender_object_id);
            NativeEndian::write_u32(&mut dst[i + 4..], ((total_len << 16) | 0) as u32);

            NativeEndian::write_u32(&mut dst[i + 8..], self.callback_data);
            Ok(())
        }
    }
}

pub fn dispatch_request(
    request: Arc<RwLock<WlCallback>>,
    session: RwLock<super::super::session::Session>,
    tx: tokio::sync::mpsc::Sender<Box<super::super::event::Event + Send>>,
    sender_object_id: u32,
    opcode: u16,
    args: Vec<u8>,
) -> Box<futures::future::Future<Item = (), Error = ()> + Send> {
    Box::new(futures::future::ok(()))
}

// callback object
//
// Clients can handle the 'done' event to get notified when
// the related request is done.
pub struct WlCallback {}
