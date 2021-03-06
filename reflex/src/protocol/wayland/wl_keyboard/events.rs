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

use byteorder::{ByteOrder, NativeEndian};

// enter event
//
// Notification that this seat's keyboard focus is on a certain
// surface.
#[allow(dead_code)]
pub struct Enter {
    pub sender_object_id: u32,
    pub serial: u32,   // uint: serial number of the enter event
    pub surface: u32,  // object: surface gaining keyboard focus
    pub keys: Vec<u8>, // array: the currently pressed keys
}

impl super::super::super::event::Event for Enter {
    fn encode(&self, dst: &mut bytes::BytesMut) -> Result<(), std::io::Error> {
        let total_len = 8 + 4 + 4 + { 4 + (self.keys.len() + 1 + 3) / 4 * 4 };
        if total_len > 0xffff {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Oops!"));
        }

        let mut encode_offset = dst.len();
        dst.resize(encode_offset + total_len, 0);

        NativeEndian::write_u32(&mut dst[encode_offset..], self.sender_object_id);
        let event_opcode = 1;
        NativeEndian::write_u32(
            &mut dst[encode_offset + 4..],
            ((total_len << 16) | event_opcode) as u32,
        );

        encode_offset += 8;
        NativeEndian::write_u32(&mut dst[encode_offset..], self.serial);
        encode_offset += 4;
        NativeEndian::write_u32(&mut dst[encode_offset..], self.surface);
        encode_offset += 4;

        NativeEndian::write_u32(&mut dst[encode_offset..], self.keys.len() as u32);
        {
            let mut aligned_keys = self.keys.clone();
            while aligned_keys.len() % 4 != 0 {
                aligned_keys.push(0u8);
            }
            dst[(encode_offset + 4)..(encode_offset + 4 + aligned_keys.len())]
                .copy_from_slice(&aligned_keys[..]);
        }

        encode_offset += { 4 + (self.keys.len() + 1 + 3) / 4 * 4 };
        let _ = encode_offset;
        Ok(())
    }
}

// key event
//
// A key was pressed or released.
// The time argument is a timestamp with millisecond
// granularity, with an undefined base.
#[allow(dead_code)]
pub struct Key {
    pub sender_object_id: u32,
    pub serial: u32, // uint: serial number of the key event
    pub time: u32,   // uint: timestamp with millisecond granularity
    pub key: u32,    // uint: key that produced the event
    pub state: u32,  // uint: physical state of the key
}

impl super::super::super::event::Event for Key {
    fn encode(&self, dst: &mut bytes::BytesMut) -> Result<(), std::io::Error> {
        let total_len = 8 + 4 + 4 + 4 + 4;
        if total_len > 0xffff {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Oops!"));
        }

        let mut encode_offset = dst.len();
        dst.resize(encode_offset + total_len, 0);

        NativeEndian::write_u32(&mut dst[encode_offset..], self.sender_object_id);
        let event_opcode = 3;
        NativeEndian::write_u32(
            &mut dst[encode_offset + 4..],
            ((total_len << 16) | event_opcode) as u32,
        );

        encode_offset += 8;
        NativeEndian::write_u32(&mut dst[encode_offset..], self.serial);
        encode_offset += 4;
        NativeEndian::write_u32(&mut dst[encode_offset..], self.time);
        encode_offset += 4;
        NativeEndian::write_u32(&mut dst[encode_offset..], self.key);
        encode_offset += 4;
        NativeEndian::write_u32(&mut dst[encode_offset..], self.state);
        encode_offset += 4;
        let _ = encode_offset;
        Ok(())
    }
}

// keyboard mapping
//
// This event provides a file descriptor to the client which can be
// memory-mapped to provide a keyboard mapping description.
#[allow(dead_code)]
pub struct Keymap {
    pub sender_object_id: u32,
    pub format: u32, // uint: keymap format
    pub fd: i32,     // fd: keymap file descriptor
    pub size: u32,   // uint: keymap size, in bytes
}

impl super::super::super::event::Event for Keymap {
    fn encode(&self, dst: &mut bytes::BytesMut) -> Result<(), std::io::Error> {
        let total_len = 8 + 4 + 0 + 4;
        if total_len > 0xffff {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Oops!"));
        }

        let mut encode_offset = dst.len();
        dst.resize(encode_offset + total_len, 0);

        NativeEndian::write_u32(&mut dst[encode_offset..], self.sender_object_id);
        let event_opcode = 0;
        NativeEndian::write_u32(
            &mut dst[encode_offset + 4..],
            ((total_len << 16) | event_opcode) as u32,
        );

        encode_offset += 8;
        NativeEndian::write_u32(&mut dst[encode_offset..], self.format);
        encode_offset += 4;
        println!("UNIMPLEMENTED!!!!!");
        encode_offset += 0;
        NativeEndian::write_u32(&mut dst[encode_offset..], self.size);
        encode_offset += 4;
        let _ = encode_offset;
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
#[allow(dead_code)]
pub struct Leave {
    pub sender_object_id: u32,
    pub serial: u32,  // uint: serial number of the leave event
    pub surface: u32, // object: surface that lost keyboard focus
}

impl super::super::super::event::Event for Leave {
    fn encode(&self, dst: &mut bytes::BytesMut) -> Result<(), std::io::Error> {
        let total_len = 8 + 4 + 4;
        if total_len > 0xffff {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Oops!"));
        }

        let mut encode_offset = dst.len();
        dst.resize(encode_offset + total_len, 0);

        NativeEndian::write_u32(&mut dst[encode_offset..], self.sender_object_id);
        let event_opcode = 2;
        NativeEndian::write_u32(
            &mut dst[encode_offset + 4..],
            ((total_len << 16) | event_opcode) as u32,
        );

        encode_offset += 8;
        NativeEndian::write_u32(&mut dst[encode_offset..], self.serial);
        encode_offset += 4;
        NativeEndian::write_u32(&mut dst[encode_offset..], self.surface);
        encode_offset += 4;
        let _ = encode_offset;
        Ok(())
    }
}

// modifier and group state
//
// Notifies clients that the modifier and/or group state has
// changed, and it should update its local state.
#[allow(dead_code)]
pub struct Modifiers {
    pub sender_object_id: u32,
    pub serial: u32,         // uint: serial number of the modifiers event
    pub mods_depressed: u32, // uint: depressed modifiers
    pub mods_latched: u32,   // uint: latched modifiers
    pub mods_locked: u32,    // uint: locked modifiers
    pub group: u32,          // uint: keyboard layout
}

impl super::super::super::event::Event for Modifiers {
    fn encode(&self, dst: &mut bytes::BytesMut) -> Result<(), std::io::Error> {
        let total_len = 8 + 4 + 4 + 4 + 4 + 4;
        if total_len > 0xffff {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Oops!"));
        }

        let mut encode_offset = dst.len();
        dst.resize(encode_offset + total_len, 0);

        NativeEndian::write_u32(&mut dst[encode_offset..], self.sender_object_id);
        let event_opcode = 4;
        NativeEndian::write_u32(
            &mut dst[encode_offset + 4..],
            ((total_len << 16) | event_opcode) as u32,
        );

        encode_offset += 8;
        NativeEndian::write_u32(&mut dst[encode_offset..], self.serial);
        encode_offset += 4;
        NativeEndian::write_u32(&mut dst[encode_offset..], self.mods_depressed);
        encode_offset += 4;
        NativeEndian::write_u32(&mut dst[encode_offset..], self.mods_latched);
        encode_offset += 4;
        NativeEndian::write_u32(&mut dst[encode_offset..], self.mods_locked);
        encode_offset += 4;
        NativeEndian::write_u32(&mut dst[encode_offset..], self.group);
        encode_offset += 4;
        let _ = encode_offset;
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
#[allow(dead_code)]
pub struct RepeatInfo {
    pub sender_object_id: u32,
    pub rate: i32,  // int: the rate of repeating keys in characters per second
    pub delay: i32, // int: delay in milliseconds since key down until repeating starts
}

impl super::super::super::event::Event for RepeatInfo {
    fn encode(&self, dst: &mut bytes::BytesMut) -> Result<(), std::io::Error> {
        let total_len = 8 + 4 + 4;
        if total_len > 0xffff {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Oops!"));
        }

        let mut encode_offset = dst.len();
        dst.resize(encode_offset + total_len, 0);

        NativeEndian::write_u32(&mut dst[encode_offset..], self.sender_object_id);
        let event_opcode = 5;
        NativeEndian::write_u32(
            &mut dst[encode_offset + 4..],
            ((total_len << 16) | event_opcode) as u32,
        );

        encode_offset += 8;
        NativeEndian::write_i32(&mut dst[encode_offset..], self.rate);
        encode_offset += 4;
        NativeEndian::write_i32(&mut dst[encode_offset..], self.delay);
        encode_offset += 4;
        let _ = encode_offset;
        Ok(())
    }
}
