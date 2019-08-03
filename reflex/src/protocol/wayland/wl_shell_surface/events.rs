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

// suggest resize
//
// The configure event asks the client to resize its surface.
//
// The size is a hint, in the sense that the client is free to
// ignore it if it doesn't resize, pick a smaller size (to
// satisfy aspect ratio or resize in steps of NxM pixels).
//
// The edges parameter provides a hint about how the surface
// was resized. The client may use this information to decide
// how to adjust its content to the new size (e.g. a scrolling
// area might adjust its content position to leave the viewable
// content unmoved).
//
// The client is free to dismiss all but the last configure
// event it received.
//
// The width and height arguments specify the size of the window
// in surface-local coordinates.
pub struct Configure {
    pub sender_object_id: u32,
    pub edges: u32,  // uint: how the surface was resized
    pub width: i32,  // int: new width of the surface
    pub height: i32, // int: new height of the surface
}

impl super::super::super::event::Event for Configure {
    fn encode(&self, dst: &mut bytes::BytesMut) -> Result<(), std::io::Error> {
        let total_len = 8 + 4 + 4 + 4;
        if total_len > 0xffff {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Oops!"));
        }

        let i = dst.len();
        dst.resize(i + total_len, 0);

        NativeEndian::write_u32(&mut dst[i..], self.sender_object_id);
        NativeEndian::write_u32(&mut dst[i + 4..], ((total_len << 16) | 1) as u32);

        NativeEndian::write_u32(&mut dst[i + 8..], self.edges);
        NativeEndian::write_i32(&mut dst[i + 8 + 4..], self.width);
        NativeEndian::write_i32(&mut dst[i + 8 + 4 + 4..], self.height);
        Ok(())
    }
}

// ping client
//
// Ping a client to check if it is receiving events and sending
// requests. A client is expected to reply with a pong request.
pub struct Ping {
    pub sender_object_id: u32,
    pub serial: u32, // uint: serial number of the ping
}

impl super::super::super::event::Event for Ping {
    fn encode(&self, dst: &mut bytes::BytesMut) -> Result<(), std::io::Error> {
        let total_len = 8 + 4;
        if total_len > 0xffff {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Oops!"));
        }

        let i = dst.len();
        dst.resize(i + total_len, 0);

        NativeEndian::write_u32(&mut dst[i..], self.sender_object_id);
        NativeEndian::write_u32(&mut dst[i + 4..], ((total_len << 16) | 0) as u32);

        NativeEndian::write_u32(&mut dst[i + 8..], self.serial);
        Ok(())
    }
}

// popup interaction is done
//
// The popup_done event is sent out when a popup grab is broken,
// that is, when the user clicks a surface that doesn't belong
// to the client owning the popup surface.
pub struct PopupDone {
    pub sender_object_id: u32,
}

impl super::super::super::event::Event for PopupDone {
    fn encode(&self, dst: &mut bytes::BytesMut) -> Result<(), std::io::Error> {
        let total_len = 8;
        if total_len > 0xffff {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Oops!"));
        }

        let i = dst.len();
        dst.resize(i + total_len, 0);

        NativeEndian::write_u32(&mut dst[i..], self.sender_object_id);
        NativeEndian::write_u32(&mut dst[i + 4..], ((total_len << 16) | 2) as u32);

        Ok(())
    }
}
