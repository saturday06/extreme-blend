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
#[allow(dead_code)]
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

        let mut encode_offset = dst.len();
        dst.resize(encode_offset + total_len, 0);

        NativeEndian::write_u32(&mut dst[encode_offset..], self.sender_object_id);
        let event_opcode = 0;
        NativeEndian::write_u32(
            &mut dst[encode_offset + 4..],
            ((total_len << 16) | event_opcode) as u32,
        );

        encode_offset += 8;
        NativeEndian::write_u32(&mut dst[encode_offset..], self.capabilities);
        encode_offset += 4;
        let _ = encode_offset;
        Ok(())
    }
}

// unique identifier for this seat
//
// In a multiseat configuration this can be used by the client to help
// identify which physical devices the seat represents. Based on
// the seat configuration used by the compositor.
#[allow(dead_code)]
pub struct Name {
    pub sender_object_id: u32,
    pub name: String, // string: seat identifier
}

impl super::super::super::event::Event for Name {
    fn encode(&self, dst: &mut bytes::BytesMut) -> Result<(), std::io::Error> {
        let total_len = 8 + { 4 + (self.name.len() + 1 + 3) / 4 * 4 };
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
        NativeEndian::write_u32(&mut dst[encode_offset..], (self.name.len() + 1) as u32);
        {
            let mut aligned = self.name.clone();
            aligned.push(0u8.into());
            while aligned.len() % 4 != 0 {
                aligned.push(0u8.into());
            }
            dst[(encode_offset + 4)..(encode_offset + 4 + aligned.len())]
                .copy_from_slice(aligned.as_bytes());
        }

        encode_offset += { 4 + (self.name.len() + 1 + 3) / 4 * 4 };
        let _ = encode_offset;
        Ok(())
    }
}
