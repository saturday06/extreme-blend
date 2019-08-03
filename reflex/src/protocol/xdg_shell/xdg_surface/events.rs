// Copyright © 2008-2013 Kristian Høgsberg
// Copyright © 2013      Rafael Antognolli
// Copyright © 2013      Jasper St. Pierre
// Copyright © 2010-2013 Intel Corporation
// Copyright © 2015-2017 Samsung Electronics Co., Ltd
// Copyright © 2015-2017 Red Hat Inc.
// 
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
// 
// The above copyright notice and this permission notice (including the next
// paragraph) shall be included in all copies or substantial portions of the
// Software.
// 
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

use byteorder::{ByteOrder, NativeEndian};

// suggest a surface change
//
// The configure event marks the end of a configure sequence. A configure
// sequence is a set of one or more events configuring the state of the
// xdg_surface, including the final xdg_surface.configure event.
// 
// Where applicable, xdg_surface surface roles will during a configure
// sequence extend this event as a latched state sent as events before the
// xdg_surface.configure event. Such events should be considered to make up
// a set of atomically applied configuration states, where the
// xdg_surface.configure commits the accumulated state.
// 
// Clients should arrange their surface for the new states, and then send
// an ack_configure request with the serial sent in this configure event at
// some point before committing the new surface.
// 
// If the client receives multiple configure events before it can respond
// to one, it is free to discard all but the last event it received.
pub struct Configure {
    pub sender_object_id: u32,
    pub serial: u32, // uint: serial of the configure event
}

impl super::super::super::event::Event for Configure {
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
