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

pub mod enums {
    // seat capability bitmask
    //
    // This is a bitmask of capabilities this seat has; if a member is
    // set, then it is present on the seat.
    pub enum Capability {
        Pointer = 1, // the seat has pointer devices
        Keyboard = 2, // the seat has one or more keyboards
        Touch = 4, // the seat has touch devices
    }
}

pub mod events {
    use byteorder::{ByteOrder, NativeEndian};

    // seat capabilities changed
    //
    // This is emitted whenever a seat gains or loses the pointer,
    // keyboard or touch capabilities.  The argument is a capability
    // enum containing the complete set of capabilities this seat has.
    // 
    // When the pointer capability is added, a client may create a
    // wl_pointer object using the wl_seat.get_pointer request. This object
    // will receive pointer events until the capability is removed in the
    // future.
    // 
    // When the pointer capability is removed, a client should destroy the
    // wl_pointer objects associated with the seat where the capability was
    // removed, using the wl_pointer.release request. No further pointer
    // events will be received on these objects.
    // 
    // In some compositors, if a seat regains the pointer capability and a
    // client has a previously obtained wl_pointer object of version 4 or
    // less, that object may start sending pointer events again. This
    // behavior is considered a misinterpretation of the intended behavior
    // and must not be relied upon by the client. wl_pointer objects of
    // version 5 or later must not send events if created before the most
    // recent event notifying the client of an added pointer capability.
    // 
    // The above behavior also applies to wl_keyboard and wl_touch with the
    // keyboard and touch capabilities, respectively.
    pub struct Capabilities {
        pub sender_object_id: u32,
        pub capabilities: u32, // uint: capabilities of the seat
    }

    impl super::super::super::event::Event for Capabilities {
        fn encode(&self, dst: &mut bytes::BytesMut) -> Result<(), std::io::Error> {
            let total_len = 8 + 4;
            if total_len > 0xffff {
                return Err(std::io::Error::new(std::io::ErrorKind::Other, "Oops!"));
            }

            let i = dst.len();
            dst.resize(i + total_len, 0);

            NativeEndian::write_u32(&mut dst[i..], self.sender_object_id);
            NativeEndian::write_u32(&mut dst[i + 4..], ((total_len << 16) | 0) as u32);

            NativeEndian::write_u32(&mut dst[i + 8..], self.capabilities);
            Ok(())
        }
    }

    // unique identifier for this seat
    //
    // In a multiseat configuration this can be used by the client to help
    // identify which physical devices the seat represents. Based on
    // the seat configuration used by the compositor.
    pub struct Name {
        pub sender_object_id: u32,
        pub name: String, // string: seat identifier
    }

    impl super::super::super::event::Event for Name {
        fn encode(&self, dst: &mut bytes::BytesMut) -> Result<(), std::io::Error> {
            let total_len = 8 + (4 + (self.name.len() + 1 + 3) / 4 * 4);
            if total_len > 0xffff {
                return Err(std::io::Error::new(std::io::ErrorKind::Other, "Oops!"));
            }

            let i = dst.len();
            dst.resize(i + total_len, 0);

            NativeEndian::write_u32(&mut dst[i..], self.sender_object_id);
            NativeEndian::write_u32(&mut dst[i + 4..], ((total_len << 16) | 1) as u32);

            
            NativeEndian::write_u32(&mut dst[i + 8..], self.name.len() as u32);
            let mut aligned_name = self.name.clone();
            aligned_name.push(0u8.into());
            while aligned_name.len() % 4 != 0 {
                aligned_name.push(0u8.into());
            }
            dst[(i + 8 + 4)..(i + 8 + 4 + aligned_name.len())].copy_from_slice(aligned_name.as_bytes());

            Ok(())
        }
    }
}

pub fn dispatch_request(request: Arc<RwLock<WlSeat>>, session: RwLock<super::super::session::Session>, tx: tokio::sync::mpsc::Sender<Box<super::super::event::Event + Send>>, sender_object_id: u32, opcode: u16, args: Vec<u8>) -> Box<futures::future::Future<Item = (), Error = ()>> {
    let mut cursor = Cursor::new(&args);
    match opcode {
        0 => {
            let id = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
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
            return WlSeat::get_pointer(request, session, tx, sender_object_id, id)
        },
        1 => {
            let id = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
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
            return WlSeat::get_keyboard(request, session, tx, sender_object_id, id)
        },
        2 => {
            let id = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
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
            return WlSeat::get_touch(request, session, tx, sender_object_id, id)
        },
        3 => {
            return WlSeat::release(request, session, tx, sender_object_id, )
        },
        _ => {},
    };
    Box::new(futures::future::ok(()))
}

// group of input devices
//
// A seat is a group of keyboards, pointer and touch devices. This
// object is published as a global during start up, or when such a
// device is hot plugged.  A seat typically has a pointer and
// maintains a keyboard focus and a pointer focus.
pub struct WlSeat {
}

impl WlSeat {
    // return keyboard object
    //
    // The ID provided will be initialized to the wl_keyboard interface
    // for this seat.
    // 
    // This request only takes effect if the seat has the keyboard
    // capability, or has had the keyboard capability in the past.
    // It is a protocol violation to issue this request on a seat that has
    // never had the keyboard capability.
    pub fn get_keyboard(
        request: Arc<RwLock<WlSeat>>,
        session: RwLock<super::super::session::Session>,
        tx: tokio::sync::mpsc::Sender<Box<super::super::event::Event + Send>>,
        sender_object_id: u32,
        id: u32, // new_id: seat keyboard
    ) -> Box<futures::future::Future<Item = (), Error = ()>> {
        Box::new(futures::future::ok(()))
    }

    // return pointer object
    //
    // The ID provided will be initialized to the wl_pointer interface
    // for this seat.
    // 
    // This request only takes effect if the seat has the pointer
    // capability, or has had the pointer capability in the past.
    // It is a protocol violation to issue this request on a seat that has
    // never had the pointer capability.
    pub fn get_pointer(
        request: Arc<RwLock<WlSeat>>,
        session: RwLock<super::super::session::Session>,
        tx: tokio::sync::mpsc::Sender<Box<super::super::event::Event + Send>>,
        sender_object_id: u32,
        id: u32, // new_id: seat pointer
    ) -> Box<futures::future::Future<Item = (), Error = ()>> {
        Box::new(futures::future::ok(()))
    }

    // return touch object
    //
    // The ID provided will be initialized to the wl_touch interface
    // for this seat.
    // 
    // This request only takes effect if the seat has the touch
    // capability, or has had the touch capability in the past.
    // It is a protocol violation to issue this request on a seat that has
    // never had the touch capability.
    pub fn get_touch(
        request: Arc<RwLock<WlSeat>>,
        session: RwLock<super::super::session::Session>,
        tx: tokio::sync::mpsc::Sender<Box<super::super::event::Event + Send>>,
        sender_object_id: u32,
        id: u32, // new_id: seat touch interface
    ) -> Box<futures::future::Future<Item = (), Error = ()>> {
        Box::new(futures::future::ok(()))
    }

    // release the seat object
    //
    // Using this request a client can tell the server that it is not going to
    // use the seat object anymore.
    pub fn release(
        request: Arc<RwLock<WlSeat>>,
        session: RwLock<super::super::session::Session>,
        tx: tokio::sync::mpsc::Sender<Box<super::super::event::Event + Send>>,
        sender_object_id: u32,
    ) -> Box<futures::future::Future<Item = (), Error = ()>> {
        Box::new(futures::future::ok(()))
    }
}
