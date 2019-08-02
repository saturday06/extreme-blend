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

use byteorder::{NativeEndian, ReadBytesExt};
use futures::future::Future;
use futures::sink::Sink;
use std::io::{Cursor, Read};
use std::sync::Arc;
use std::cell::RefCell;

pub mod enums {
    // global error values
    //
    // These errors are global and can be emitted in response to any
    // server request.
    pub enum Error {
        InvalidObject = 0, // server couldn't find object
        InvalidMethod = 1, // method doesn't exist on the specified interface
        NoMemory = 2, // server is out of memory
    }
}

pub mod events {
    use byteorder::{ByteOrder, NativeEndian};

    // acknowledge object ID deletion
    //
    // This event is used internally by the object ID management
    // logic.  When a client deletes an object, the server will send
    // this event to acknowledge that it has seen the delete request.
    // When the client receives this event, it will know that it can
    // safely reuse the object ID.
    pub struct DeleteId {
        pub sender_object_id: u32,
        pub id: u32, // uint: deleted object ID
    }

    impl super::super::super::event::Event for DeleteId {
        fn encode(&self, dst: &mut bytes::BytesMut) -> Result<(), std::io::Error> {
            let total_len = 8 + 4;
            if total_len > 0xffff {
                return Err(std::io::Error::new(std::io::ErrorKind::Other, "Oops!"));
            }

            let i = dst.len();
            dst.resize(i + total_len, 0);

            NativeEndian::write_u32(&mut dst[i..], self.sender_object_id);
            NativeEndian::write_u32(&mut dst[i + 4..], ((total_len << 16) | 1) as u32);

            NativeEndian::write_u32(&mut dst[i + 8..], self.id);
            Ok(())
        }
    }

    // fatal error event
    //
    // The error event is sent out when a fatal (non-recoverable)
    // error has occurred.  The object_id argument is the object
    // where the error occurred, most often in response to a request
    // to that object.  The code identifies the error and is defined
    // by the object interface.  As such, each interface defines its
    // own set of error codes.  The message is a brief description
    // of the error, for (debugging) convenience.
    pub struct Error {
        pub sender_object_id: u32,
        pub object_id: u32, // object: object where the error occurred
        pub code: u32, // uint: error code
        pub message: String, // string: error description
    }

    impl super::super::super::event::Event for Error {
        fn encode(&self, dst: &mut bytes::BytesMut) -> Result<(), std::io::Error> {
            let total_len = 8 + 4 + 4 + (4 + (self.message.len() + 1 + 3) / 4 * 4);
            if total_len > 0xffff {
                return Err(std::io::Error::new(std::io::ErrorKind::Other, "Oops!"));
            }

            let i = dst.len();
            dst.resize(i + total_len, 0);

            NativeEndian::write_u32(&mut dst[i..], self.sender_object_id);
            NativeEndian::write_u32(&mut dst[i + 4..], ((total_len << 16) | 0) as u32);

            NativeEndian::write_u32(&mut dst[i + 8..], self.object_id);
            NativeEndian::write_u32(&mut dst[i + 8 + 4..], self.code);
            
            NativeEndian::write_u32(&mut dst[i + 8 + 4 + 4..], self.message.len() as u32);
            let mut aligned_message = self.message.clone();
            aligned_message.push(0u8.into());
            while aligned_message.len() % 4 != 0 {
                aligned_message.push(0u8.into());
            }
            dst[(i + 8 + 4 + 4 + 4)..(i + 8 + 4 + 4 + 4 + aligned_message.len())].copy_from_slice(aligned_message.as_bytes());

            Ok(())
        }
    }
}

pub fn dispatch_request(request: Arc<RefCell<WlDisplay>>, session: &mut super::super::session::Session, tx: tokio::sync::mpsc::Sender<Box<super::super::event::Event + Send>>, sender_object_id: u32, opcode: u16, args: Vec<u8>) -> Box<futures::future::Future<Item = (), Error = ()>> {
    let mut cursor = Cursor::new(&args);
    match opcode {
        0 => {
            let callback = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
                x 
            } else {
                return Box::new(tx.send(Box::new(super::super::wayland::wl_display::events::Error {
                    sender_object_id: 1,
                    object_id: sender_object_id,
                    code: super::super::wayland::wl_display::enums::Error::InvalidMethod as u32,
                    message: format!(
                        "@{} opcode={} args={:?} not found",
                        sender_object_id, opcode, args
                    ),
                })).map_err(|_| ()).map(|_tx| ()));

            };
            return WlDisplay::sync(request, session, tx, sender_object_id, callback)
        },
        1 => {
            let registry = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
                x 
            } else {
                return Box::new(tx.send(Box::new(super::super::wayland::wl_display::events::Error {
                    sender_object_id: 1,
                    object_id: sender_object_id,
                    code: super::super::wayland::wl_display::enums::Error::InvalidMethod as u32,
                    message: format!(
                        "@{} opcode={} args={:?} not found",
                        sender_object_id, opcode, args
                    ),
                })).map_err(|_| ()).map(|_tx| ()));

            };
            return WlDisplay::get_registry(request, session, tx, sender_object_id, registry)
        },
        _ => {},
    };
    Box::new(futures::future::ok(()))
}

// core global object
//
// The core global object.  This is a special singleton object.  It
// is used for internal Wayland protocol features.
pub struct WlDisplay {
}

impl WlDisplay {
    // get global registry object
    //
    // This request creates a registry object that allows the client
    // to list and bind the global objects available from the
    // compositor.
    // 
    // It should be noted that the server side resources consumed in
    // response to a get_registry request can only be released when the
    // client disconnects, not when the client side proxy is destroyed.
    // Therefore, clients should invoke get_registry as infrequently as
    // possible to avoid wasting memory.
    pub fn get_registry(
        request: Arc<RefCell<WlDisplay>>,
        session: &mut super::super::session::Session,
        tx: tokio::sync::mpsc::Sender<Box<super::super::event::Event + Send>>,
        sender_object_id: u32,
        registry: u32, // new_id: global registry object
    ) -> Box<futures::future::Future<Item = (), Error = ()>> {
        Box::new(futures::future::ok(()))
    }

    // asynchronous roundtrip
    //
    // The sync request asks the server to emit the 'done' event
    // on the returned wl_callback object.  Since requests are
    // handled in-order and events are delivered in-order, this can
    // be used as a barrier to ensure all previous requests and the
    // resulting events have been handled.
    // 
    // The object returned by this request will be destroyed by the
    // compositor after the callback is fired and as such the client must not
    // attempt to use it after that point.
    // 
    // The callback_data passed in the callback is the event serial.
    pub fn sync(
        request: Arc<RefCell<WlDisplay>>,
        session: &mut super::super::session::Session,
        tx: tokio::sync::mpsc::Sender<Box<super::super::event::Event + Send>>,
        sender_object_id: u32,
        callback: u32, // new_id: callback object for the sync request
    ) -> Box<futures::future::Future<Item = (), Error = ()>> {
        Box::new(futures::future::ok(()))
    }
}
