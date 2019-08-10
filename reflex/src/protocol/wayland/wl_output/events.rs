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

// sent all information about output
//
// This event is sent after all other properties have been
// sent after binding to the output object and after any
// other property changes done after that. This allows
// changes to the output properties to be seen as
// atomic, even if they happen via multiple events.
#[allow(dead_code)]
pub struct Done {
    pub sender_object_id: u32,
}

impl super::super::super::event::Event for Done {
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

// properties of the output
//
// The geometry event describes geometric properties of the output.
// The event is sent when binding to the output object and whenever
// any of the properties change.
//
// The physical size can be set to zero if it doesn't make sense for this
// output (e.g. for projectors or virtual outputs).
#[allow(dead_code)]
pub struct Geometry {
    pub sender_object_id: u32,
    pub x: i32,               // int: x position within the global compositor space
    pub y: i32,               // int: y position within the global compositor space
    pub physical_width: i32,  // int: width in millimeters of the output
    pub physical_height: i32, // int: height in millimeters of the output
    pub subpixel: i32,        // int: subpixel orientation of the output
    pub make: String,         // string: textual description of the manufacturer
    pub model: String,        // string: textual description of the model
    pub transform: i32,       // int: transform that maps framebuffer to output
}

impl super::super::super::event::Event for Geometry {
    fn encode(&self, dst: &mut bytes::BytesMut) -> Result<(), std::io::Error> {
        let total_len = 8
            + 4
            + 4
            + 4
            + 4
            + 4
            + (4 + (self.make.len() + 1 + 3) / 4 * 4)
            + (4 + (self.model.len() + 1 + 3) / 4 * 4)
            + 4;
        if total_len > 0xffff {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Oops!"));
        }

        let i = dst.len();
        dst.resize(i + total_len, 0);

        NativeEndian::write_u32(&mut dst[i..], self.sender_object_id);
        NativeEndian::write_u32(&mut dst[i + 4..], ((total_len << 16) | 0) as u32);

        NativeEndian::write_i32(&mut dst[i + 8..], self.x);
        NativeEndian::write_i32(&mut dst[i + 8 + 4..], self.y);
        NativeEndian::write_i32(&mut dst[i + 8 + 4 + 4..], self.physical_width);
        NativeEndian::write_i32(&mut dst[i + 8 + 4 + 4 + 4..], self.physical_height);
        NativeEndian::write_i32(&mut dst[i + 8 + 4 + 4 + 4 + 4..], self.subpixel);

        NativeEndian::write_u32(
            &mut dst[i + 8 + 4 + 4 + 4 + 4 + 4..],
            (self.make.len() + 1) as u32,
        );
        {
            let mut aligned = self.make.clone();
            aligned.push(0u8.into());
            while aligned.len() % 4 != 0 {
                aligned.push(0u8.into());
            }
            dst[(i + 8 + 4 + 4 + 4 + 4 + 4 + 4)..(i + 8 + 4 + 4 + 4 + 4 + 4 + 4 + aligned.len())]
                .copy_from_slice(aligned.as_bytes());
        }

        NativeEndian::write_u32(
            &mut dst[i + 8 + 4 + 4 + 4 + 4 + 4 + (4 + (self.make.len() + 1 + 3) / 4 * 4)..],
            (self.model.len() + 1) as u32,
        );
        {
            let mut aligned = self.model.clone();
            aligned.push(0u8.into());
            while aligned.len() % 4 != 0 {
                aligned.push(0u8.into());
            }
            dst[(i + 8 + 4 + 4 + 4 + 4 + 4 + (4 + (self.make.len() + 1 + 3) / 4 * 4) + 4)
                ..(i + 8
                    + 4
                    + 4
                    + 4
                    + 4
                    + 4
                    + (4 + (self.make.len() + 1 + 3) / 4 * 4)
                    + 4
                    + aligned.len())]
                .copy_from_slice(aligned.as_bytes());
        }

        NativeEndian::write_i32(
            &mut dst[i
                + 8
                + 4
                + 4
                + 4
                + 4
                + 4
                + (4 + (self.make.len() + 1 + 3) / 4 * 4)
                + (4 + (self.model.len() + 1 + 3) / 4 * 4)..],
            self.transform,
        );
        Ok(())
    }
}

// advertise available modes for the output
//
// The mode event describes an available mode for the output.
//
// The event is sent when binding to the output object and there
// will always be one mode, the current mode.  The event is sent
// again if an output changes mode, for the mode that is now
// current.  In other words, the current mode is always the last
// mode that was received with the current flag set.
//
// The size of a mode is given in physical hardware units of
// the output device. This is not necessarily the same as
// the output size in the global compositor space. For instance,
// the output may be scaled, as described in wl_output.scale,
// or transformed, as described in wl_output.transform.
#[allow(dead_code)]
pub struct Mode {
    pub sender_object_id: u32,
    pub flags: u32,   // uint: bitfield of mode flags
    pub width: i32,   // int: width of the mode in hardware units
    pub height: i32,  // int: height of the mode in hardware units
    pub refresh: i32, // int: vertical refresh rate in mHz
}

impl super::super::super::event::Event for Mode {
    fn encode(&self, dst: &mut bytes::BytesMut) -> Result<(), std::io::Error> {
        let total_len = 8 + 4 + 4 + 4 + 4;
        if total_len > 0xffff {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Oops!"));
        }

        let i = dst.len();
        dst.resize(i + total_len, 0);

        NativeEndian::write_u32(&mut dst[i..], self.sender_object_id);
        NativeEndian::write_u32(&mut dst[i + 4..], ((total_len << 16) | 1) as u32);

        NativeEndian::write_u32(&mut dst[i + 8..], self.flags);
        NativeEndian::write_i32(&mut dst[i + 8 + 4..], self.width);
        NativeEndian::write_i32(&mut dst[i + 8 + 4 + 4..], self.height);
        NativeEndian::write_i32(&mut dst[i + 8 + 4 + 4 + 4..], self.refresh);
        Ok(())
    }
}

// output scaling properties
//
// This event contains scaling geometry information
// that is not in the geometry event. It may be sent after
// binding the output object or if the output scale changes
// later. If it is not sent, the client should assume a
// scale of 1.
//
// A scale larger than 1 means that the compositor will
// automatically scale surface buffers by this amount
// when rendering. This is used for very high resolution
// displays where applications rendering at the native
// resolution would be too small to be legible.
//
// It is intended that scaling aware clients track the
// current output of a surface, and if it is on a scaled
// output it should use wl_surface.set_buffer_scale with
// the scale of the output. That way the compositor can
// avoid scaling the surface, and the client can supply
// a higher detail image.
#[allow(dead_code)]
pub struct Scale {
    pub sender_object_id: u32,
    pub factor: i32, // int: scaling factor of output
}

impl super::super::super::event::Event for Scale {
    fn encode(&self, dst: &mut bytes::BytesMut) -> Result<(), std::io::Error> {
        let total_len = 8 + 4;
        if total_len > 0xffff {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Oops!"));
        }

        let i = dst.len();
        dst.resize(i + total_len, 0);

        NativeEndian::write_u32(&mut dst[i..], self.sender_object_id);
        NativeEndian::write_u32(&mut dst[i + 4..], ((total_len << 16) | 3) as u32);

        NativeEndian::write_i32(&mut dst[i + 8..], self.factor);
        Ok(())
    }
}
