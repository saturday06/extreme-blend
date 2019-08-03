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
    // physical key state
    //
    // Describes the physical state of a key that produced the key event.
    pub enum KeyState {
        Released = 0, // key is not pressed
        Pressed = 1, // key is pressed
    }

    // keyboard mapping format
    //
    // This specifies the format of the keymap provided to the
    // client with the wl_keyboard.keymap event.
    pub enum KeymapFormat {
        NoKeymap = 0, // no keymap; client must understand how to interpret the raw keycode
        XkbV1 = 1, // libxkbcommon compatible; to determine the xkb keycode, clients must add 8 to the key event keycode
    }
}

pub mod events {
    use byteorder::{ByteOrder, NativeEndian};

    // enter event
    //
    // Notification that this seat's keyboard focus is on a certain
    // surface.
    pub struct Enter {
        pub sender_object_id: u32,
        pub serial: u32, // uint: serial number of the enter event
        pub surface: u32, // object: surface gaining keyboard focus
        pub keys: Vec<u8>, // array: the currently pressed keys
    }

    impl super::super::super::event::Event for Enter {
        fn encode(&self, dst: &mut bytes::BytesMut) -> Result<(), std::io::Error> {
            let total_len = 8 + 4 + 4 + (4 + (self.keys.len() + 1 + 3) / 4 * 4);
            if total_len > 0xffff {
                return Err(std::io::Error::new(std::io::ErrorKind::Other, "Oops!"));
            }

            let i = dst.len();
            dst.resize(i + total_len, 0);

            NativeEndian::write_u32(&mut dst[i..], self.sender_object_id);
            NativeEndian::write_u32(&mut dst[i + 4..], ((total_len << 16) | 1) as u32);

            NativeEndian::write_u32(&mut dst[i + 8..], self.serial);
            NativeEndian::write_u32(&mut dst[i + 8 + 4..], self.surface);
            
            NativeEndian::write_u32(&mut dst[i + 8 + 4 + 4..], self.keys.len() as u32);
            let mut aligned_keys = self.keys.clone();
            while aligned_keys.len() % 4 != 0 {
                aligned_keys.push(0u8);
            }
            dst[(i + 8 + 4 + 4 + 4)..(i + 8 + 4 + 4 + 4 + aligned_keys.len())].copy_from_slice(&aligned_keys[..]);

            Ok(())
        }
    }

    // key event
    //
    // A key was pressed or released.
    // The time argument is a timestamp with millisecond
    // granularity, with an undefined base.
    pub struct Key {
        pub sender_object_id: u32,
        pub serial: u32, // uint: serial number of the key event
        pub time: u32, // uint: timestamp with millisecond granularity
        pub key: u32, // uint: key that produced the event
        pub state: u32, // uint: physical state of the key
    }

    impl super::super::super::event::Event for Key {
        fn encode(&self, dst: &mut bytes::BytesMut) -> Result<(), std::io::Error> {
            let total_len = 8 + 4 + 4 + 4 + 4;
            if total_len > 0xffff {
                return Err(std::io::Error::new(std::io::ErrorKind::Other, "Oops!"));
            }

            let i = dst.len();
            dst.resize(i + total_len, 0);

            NativeEndian::write_u32(&mut dst[i..], self.sender_object_id);
            NativeEndian::write_u32(&mut dst[i + 4..], ((total_len << 16) | 3) as u32);

            NativeEndian::write_u32(&mut dst[i + 8..], self.serial);
            NativeEndian::write_u32(&mut dst[i + 8 + 4..], self.time);
            NativeEndian::write_u32(&mut dst[i + 8 + 4 + 4..], self.key);
            NativeEndian::write_u32(&mut dst[i + 8 + 4 + 4 + 4..], self.state);
            Ok(())
        }
    }

    // keyboard mapping
    //
    // This event provides a file descriptor to the client which can be
    // memory-mapped to provide a keyboard mapping description.
    pub struct Keymap {
        pub sender_object_id: u32,
        pub format: u32, // uint: keymap format
        pub fd: i32, // fd: keymap file descriptor
        pub size: u32, // uint: keymap size, in bytes
    }

    impl super::super::super::event::Event for Keymap {
        fn encode(&self, dst: &mut bytes::BytesMut) -> Result<(), std::io::Error> {
            let total_len = 8 + 4 + 4 + 4;
            if total_len > 0xffff {
                return Err(std::io::Error::new(std::io::ErrorKind::Other, "Oops!"));
            }

            let i = dst.len();
            dst.resize(i + total_len, 0);

            NativeEndian::write_u32(&mut dst[i..], self.sender_object_id);
            NativeEndian::write_u32(&mut dst[i + 4..], ((total_len << 16) | 0) as u32);

            NativeEndian::write_u32(&mut dst[i + 8..], self.format);
            NativeEndian::write_i32(&mut dst[i + 8 + 4..], self.fd);
            NativeEndian::write_u32(&mut dst[i + 8 + 4 + 4..], self.size);
            Ok(())
        }
    }

    // leave event
    //
    // Notification that this seat's keyboard focus is no longer on
    // a certain surface.
    // 
    // The leave notification is sent before the enter notification
    // for the new focus.
    pub struct Leave {
        pub sender_object_id: u32,
        pub serial: u32, // uint: serial number of the leave event
        pub surface: u32, // object: surface that lost keyboard focus
    }

    impl super::super::super::event::Event for Leave {
        fn encode(&self, dst: &mut bytes::BytesMut) -> Result<(), std::io::Error> {
            let total_len = 8 + 4 + 4;
            if total_len > 0xffff {
                return Err(std::io::Error::new(std::io::ErrorKind::Other, "Oops!"));
            }

            let i = dst.len();
            dst.resize(i + total_len, 0);

            NativeEndian::write_u32(&mut dst[i..], self.sender_object_id);
            NativeEndian::write_u32(&mut dst[i + 4..], ((total_len << 16) | 2) as u32);

            NativeEndian::write_u32(&mut dst[i + 8..], self.serial);
            NativeEndian::write_u32(&mut dst[i + 8 + 4..], self.surface);
            Ok(())
        }
    }

    // modifier and group state
    //
    // Notifies clients that the modifier and/or group state has
    // changed, and it should update its local state.
    pub struct Modifiers {
        pub sender_object_id: u32,
        pub serial: u32, // uint: serial number of the modifiers event
        pub mods_depressed: u32, // uint: depressed modifiers
        pub mods_latched: u32, // uint: latched modifiers
        pub mods_locked: u32, // uint: locked modifiers
        pub group: u32, // uint: keyboard layout
    }

    impl super::super::super::event::Event for Modifiers {
        fn encode(&self, dst: &mut bytes::BytesMut) -> Result<(), std::io::Error> {
            let total_len = 8 + 4 + 4 + 4 + 4 + 4;
            if total_len > 0xffff {
                return Err(std::io::Error::new(std::io::ErrorKind::Other, "Oops!"));
            }

            let i = dst.len();
            dst.resize(i + total_len, 0);

            NativeEndian::write_u32(&mut dst[i..], self.sender_object_id);
            NativeEndian::write_u32(&mut dst[i + 4..], ((total_len << 16) | 4) as u32);

            NativeEndian::write_u32(&mut dst[i + 8..], self.serial);
            NativeEndian::write_u32(&mut dst[i + 8 + 4..], self.mods_depressed);
            NativeEndian::write_u32(&mut dst[i + 8 + 4 + 4..], self.mods_latched);
            NativeEndian::write_u32(&mut dst[i + 8 + 4 + 4 + 4..], self.mods_locked);
            NativeEndian::write_u32(&mut dst[i + 8 + 4 + 4 + 4 + 4..], self.group);
            Ok(())
        }
    }

    // repeat rate and delay
    //
    // Informs the client about the keyboard's repeat rate and delay.
    // 
    // This event is sent as soon as the wl_keyboard object has been created,
    // and is guaranteed to be received by the client before any key press
    // event.
    // 
    // Negative values for either rate or delay are illegal. A rate of zero
    // will disable any repeating (regardless of the value of delay).
    // 
    // This event can be sent later on as well with a new value if necessary,
    // so clients should continue listening for the event past the creation
    // of wl_keyboard.
    pub struct RepeatInfo {
        pub sender_object_id: u32,
        pub rate: i32, // int: the rate of repeating keys in characters per second
        pub delay: i32, // int: delay in milliseconds since key down until repeating starts
    }

    impl super::super::super::event::Event for RepeatInfo {
        fn encode(&self, dst: &mut bytes::BytesMut) -> Result<(), std::io::Error> {
            let total_len = 8 + 4 + 4;
            if total_len > 0xffff {
                return Err(std::io::Error::new(std::io::ErrorKind::Other, "Oops!"));
            }

            let i = dst.len();
            dst.resize(i + total_len, 0);

            NativeEndian::write_u32(&mut dst[i..], self.sender_object_id);
            NativeEndian::write_u32(&mut dst[i + 4..], ((total_len << 16) | 5) as u32);

            NativeEndian::write_i32(&mut dst[i + 8..], self.rate);
            NativeEndian::write_i32(&mut dst[i + 8 + 4..], self.delay);
            Ok(())
        }
    }
}

pub fn dispatch_request(request: Arc<RwLock<WlKeyboard>>, session: crate::protocol::session::Session, tx: tokio::sync::mpsc::Sender<Box<super::super::event::Event + Send>>, sender_object_id: u32, opcode: u16, args: Vec<u8>) -> Box<futures::future::Future<Item = crate::protocol::session::Session, Error = ()> + Send> {
    let mut cursor = Cursor::new(&args);
    match opcode {
        0 => {
            return WlKeyboard::release(request, session, tx, sender_object_id, )
        },
        _ => {},
    };
    Box::new(futures::future::ok(session))
}

// keyboard input device
//
// The wl_keyboard interface represents one or more keyboards
// associated with a seat.
pub struct WlKeyboard {
}

impl WlKeyboard {
    // release the keyboard object
    pub fn release(
        request: Arc<RwLock<WlKeyboard>>,
        session: crate::protocol::session::Session,
        tx: tokio::sync::mpsc::Sender<Box<super::super::event::Event + Send>>,
        sender_object_id: u32,
    ) -> Box<futures::future::Future<Item = crate::protocol::session::Session, Error = ()> + Send> {
        Box::new(futures::future::ok(session))
    }
}
