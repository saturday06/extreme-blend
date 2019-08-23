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

// announce global object
//
// Notify the client of global objects.
//
// The event notifies the client that a global object with
// the given name is now available, and it implements the
// given version of the given interface.
#[allow(dead_code)]
pub struct Global {
    pub sender_object_id: u32,
    pub name: u32,         // uint: numeric name of the global object
    pub interface: String, // string: interface implemented by the object
    pub version: u32,      // uint: interface version
}

impl super::super::super::event::Event for Global {
    fn encode(&self, dst: &mut bytes::BytesMut) -> Result<(), std::io::Error> {
        let total_len = 8 + 4 + { 4 + (self.interface.len() + 1 + 3) / 4 * 4 } + 4;
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
        NativeEndian::write_u32(&mut dst[encode_offset..], self.name);
        encode_offset += 4;
        NativeEndian::write_u32(&mut dst[encode_offset..], (self.interface.len() + 1) as u32);
        {
            let mut aligned = self.interface.clone();
            aligned.push(0u8.into());
            while aligned.len() % 4 != 0 {
                aligned.push(0u8.into());
            }
            dst[(encode_offset + 4)..(encode_offset + 4 + aligned.len())]
                .copy_from_slice(aligned.as_bytes());
        }

        encode_offset += { 4 + (self.interface.len() + 1 + 3) / 4 * 4 };
        NativeEndian::write_u32(&mut dst[encode_offset..], self.version);
        encode_offset += 4;
        let _ = encode_offset;
        Ok(())
    }
}

// announce removal of global object
//
// Notify the client of removed global objects.
//
// This event notifies the client that the global identified
// by name is no longer available.  If the client bound to
// the global using the bind request, the client should now
// destroy that object.
//
// The object remains valid and requests to the object will be
// ignored until the client destroys it, to avoid races between
// the global going away and a client sending a request to it.
#[allow(dead_code)]
pub struct GlobalRemove {
    pub sender_object_id: u32,
    pub name: u32, // uint: numeric name of the global object
}

impl super::super::super::event::Event for GlobalRemove {
    fn encode(&self, dst: &mut bytes::BytesMut) -> Result<(), std::io::Error> {
        let total_len = 8 + 4;
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
        NativeEndian::write_u32(&mut dst[encode_offset..], self.name);
        encode_offset += 4;
        let _ = encode_offset;
        Ok(())
    }
}
