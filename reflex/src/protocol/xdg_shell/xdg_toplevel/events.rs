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

// surface wants to be closed
//
// The close event is sent by the compositor when the user
// wants the surface to be closed. This should be equivalent to
// the user clicking the close button in client-side decorations,
// if your application has any.
//
// This is only a request that the user intends to close the
// window. The client may choose to ignore this request, or show
// a dialog to ask the user to save their data, etc.
pub struct Close {
    pub sender_object_id: u32,
}

impl super::super::super::event::Event for Close {
    fn encode(&self, dst: &mut bytes::BytesMut) -> Result<(), std::io::Error> {
        let total_len = 8;
        if total_len > 0xffff {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Oops!"));
        }

        let i = dst.len();
        dst.resize(i + total_len, 0);

        NativeEndian::write_u32(&mut dst[i..], self.sender_object_id);
        NativeEndian::write_u32(&mut dst[i + 4..], ((total_len << 16) | 1) as u32);

        Ok(())
    }
}

// suggest a surface change
//
// This configure event asks the client to resize its toplevel surface or
// to change its state. The configured state should not be applied
// immediately. See xdg_surface.configure for details.
//
// The width and height arguments specify a hint to the window
// about how its surface should be resized in window geometry
// coordinates. See set_window_geometry.
//
// If the width or height arguments are zero, it means the client
// should decide its own window dimension. This may happen when the
// compositor needs to configure the state of the surface but doesn't
// have any information about any previous or expected dimension.
//
// The states listed in the event specify how the width/height
// arguments should be interpreted, and possibly how it should be
// drawn.
//
// Clients must send an ack_configure in response to this event. See
// xdg_surface.configure and xdg_surface.ack_configure for details.
pub struct Configure {
    pub sender_object_id: u32,
    pub width: i32,      // int:
    pub height: i32,     // int:
    pub states: Vec<u8>, // array:
}

impl super::super::super::event::Event for Configure {
    fn encode(&self, dst: &mut bytes::BytesMut) -> Result<(), std::io::Error> {
        let total_len = 8 + 4 + 4 + (4 + (self.states.len() + 1 + 3) / 4 * 4);
        if total_len > 0xffff {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Oops!"));
        }

        let i = dst.len();
        dst.resize(i + total_len, 0);

        NativeEndian::write_u32(&mut dst[i..], self.sender_object_id);
        NativeEndian::write_u32(&mut dst[i + 4..], ((total_len << 16) | 0) as u32);

        NativeEndian::write_i32(&mut dst[i + 8..], self.width);
        NativeEndian::write_i32(&mut dst[i + 8 + 4..], self.height);

        NativeEndian::write_u32(&mut dst[i + 8 + 4 + 4..], self.states.len() as u32);
        let mut aligned_states = self.states.clone();
        while aligned_states.len() % 4 != 0 {
            aligned_states.push(0u8);
        }
        dst[(i + 8 + 4 + 4 + 4)..(i + 8 + 4 + 4 + 4 + aligned_states.len())]
            .copy_from_slice(&aligned_states[..]);

        Ok(())
    }
}
