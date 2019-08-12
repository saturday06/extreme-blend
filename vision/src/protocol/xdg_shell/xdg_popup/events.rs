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

// configure the popup surface
//
// This event asks the popup surface to configure itself given the
// configuration. The configured state should not be applied immediately.
// See xdg_surface.configure for details.
// 
// The x and y arguments represent the position the popup was placed at
// given the xdg_positioner rule, relative to the upper left corner of the
// window geometry of the parent surface.
#[allow(dead_code)]
pub struct Configure {
    pub sender_object_id: u32,
    pub x: i32, // int: x position relative to parent surface window geometry
    pub y: i32, // int: y position relative to parent surface window geometry
    pub width: i32, // int: window geometry width
    pub height: i32, // int: window geometry height
}

impl super::super::super::event::Event for Configure {
    fn encode(&self, dst: &mut bytes::BytesMut) -> Result<(), std::io::Error> {
        let total_len = 8
 + 4 + 4 + 4 + 4;
        if total_len > 0xffff {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Oops!"));
        }

        let i = dst.len();
        dst.resize(i + total_len, 0);

        NativeEndian::write_u32(&mut dst[i..], self.sender_object_id);
        NativeEndian::write_u32(&mut dst[i + 4..], ((total_len << 16) | 0) as u32);

        NativeEndian::write_i32(&mut dst[i + 8..], self.x);
        NativeEndian::write_i32(&mut dst[i + 8 + 4..], self.y);
        NativeEndian::write_i32(&mut dst[i + 8 + 4 + 4..], self.width);
        NativeEndian::write_i32(&mut dst[i + 8 + 4 + 4 + 4..], self.height);
        Ok(())
    }
}

// popup interaction is done
//
// The popup_done event is sent out when a popup is dismissed by the
// compositor. The client should destroy the xdg_popup object at this
// point.
#[allow(dead_code)]
pub struct PopupDone {
    pub sender_object_id: u32,
}

impl super::super::super::event::Event for PopupDone {
    fn encode(&self, dst: &mut bytes::BytesMut) -> Result<(), std::io::Error> {
        let total_len = 8
;
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
