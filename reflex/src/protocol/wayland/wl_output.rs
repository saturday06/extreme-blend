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

use byteorder::{NativeEndian, ReadBytesExt};
use futures::future::Future;
use futures::sink::Sink;
use std::io::{Cursor, Read};
use std::sync::Arc;
use std::cell::RefCell;

pub mod enums {
    // mode information
    //
    // These flags describe properties of an output mode.
    // They are used in the flags bitfield of the mode event.
    pub enum Mode {
        Current = 0x1, // indicates this is the current mode
        Preferred = 0x2, // indicates this is the preferred mode
    }

    // subpixel geometry information
    //
    // This enumeration describes how the physical
    // pixels on an output are laid out.
    pub enum Subpixel {
        Unknown = 0, // unknown geometry
        None = 1, // no geometry
        HorizontalRgb = 2, // horizontal RGB
        HorizontalBgr = 3, // horizontal BGR
        VerticalRgb = 4, // vertical RGB
        VerticalBgr = 5, // vertical BGR
    }

    // transform from framebuffer to output
    //
    // This describes the transform that a compositor will apply to a
    // surface to compensate for the rotation or mirroring of an
    // output device.
    // 
    // The flipped values correspond to an initial flip around a
    // vertical axis followed by rotation.
    // 
    // The purpose is mainly to allow clients to render accordingly and
    // tell the compositor, so that for fullscreen surfaces, the
    // compositor will still be able to scan out directly from client
    // surfaces.
    pub enum Transform {
        TransformNormal = 0, // no transform
        Transform90 = 1, // 90 degrees counter-clockwise
        Transform180 = 2, // 180 degrees counter-clockwise
        Transform270 = 3, // 270 degrees counter-clockwise
        TransformFlipped = 4, // 180 degree flip around a vertical axis
        TransformFlipped90 = 5, // flip and rotate 90 degrees counter-clockwise
        TransformFlipped180 = 6, // flip and rotate 180 degrees counter-clockwise
        TransformFlipped270 = 7, // flip and rotate 270 degrees counter-clockwise
    }
}

pub mod events {
    use byteorder::{ByteOrder, NativeEndian};

    // sent all information about output
    //
    // This event is sent after all other properties have been
    // sent after binding to the output object and after any
    // other property changes done after that. This allows
    // changes to the output properties to be seen as
    // atomic, even if they happen via multiple events.
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
    pub struct Geometry {
        pub sender_object_id: u32,
        pub x: i32, // int: x position within the global compositor space
        pub y: i32, // int: y position within the global compositor space
        pub physical_width: i32, // int: width in millimeters of the output
        pub physical_height: i32, // int: height in millimeters of the output
        pub subpixel: i32, // int: subpixel orientation of the output
        pub make: String, // string: textual description of the manufacturer
        pub model: String, // string: textual description of the model
        pub transform: i32, // int: transform that maps framebuffer to output
    }

    impl super::super::super::event::Event for Geometry {
        fn encode(&self, dst: &mut bytes::BytesMut) -> Result<(), std::io::Error> {
            let total_len = 8 + 4 + 4 + 4 + 4 + 4 + (4 + (self.make.len() + 1 + 3) / 4 * 4) + (4 + (self.model.len() + 1 + 3) / 4 * 4) + 4;
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
            
            NativeEndian::write_u32(&mut dst[i + 8 + 4 + 4 + 4 + 4 + 4..], self.make.len() as u32);
            let mut aligned_make = self.make.clone();
            aligned_make.push(0u8.into());
            while aligned_make.len() % 4 != 0 {
                aligned_make.push(0u8.into());
            }
            dst[(i + 8 + 4 + 4 + 4 + 4 + 4 + 4)..(i + 8 + 4 + 4 + 4 + 4 + 4 + 4 + aligned_make.len())].copy_from_slice(aligned_make.as_bytes());

            
            NativeEndian::write_u32(&mut dst[i + 8 + 4 + 4 + 4 + 4 + 4 + (4 + (self.make.len() + 1 + 3) / 4 * 4)..], self.model.len() as u32);
            let mut aligned_model = self.model.clone();
            aligned_model.push(0u8.into());
            while aligned_model.len() % 4 != 0 {
                aligned_model.push(0u8.into());
            }
            dst[(i + 8 + 4 + 4 + 4 + 4 + 4 + (4 + (self.make.len() + 1 + 3) / 4 * 4) + 4)..(i + 8 + 4 + 4 + 4 + 4 + 4 + (4 + (self.make.len() + 1 + 3) / 4 * 4) + 4 + aligned_model.len())].copy_from_slice(aligned_model.as_bytes());

            NativeEndian::write_i32(&mut dst[i + 8 + 4 + 4 + 4 + 4 + 4 + (4 + (self.make.len() + 1 + 3) / 4 * 4) + (4 + (self.model.len() + 1 + 3) / 4 * 4)..], self.transform);
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
    pub struct Mode {
        pub sender_object_id: u32,
        pub flags: u32, // uint: bitfield of mode flags
        pub width: i32, // int: width of the mode in hardware units
        pub height: i32, // int: height of the mode in hardware units
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
}

pub fn dispatch_request(request: Arc<RefCell<WlOutput>>, session: &mut super::super::session::Session, tx: tokio::sync::mpsc::Sender<Box<super::super::event::Event + Send>>, sender_object_id: u32, opcode: u16, args: Vec<u8>) -> Box<futures::future::Future<Item = (), Error = ()>> {
    let mut cursor = Cursor::new(&args);
    match opcode {
        0 => {
            return WlOutput::release(request, session, tx, sender_object_id, )
        },
        _ => {},
    };
    Box::new(futures::future::ok(()))
}

// compositor output region
//
// An output describes part of the compositor geometry.  The
// compositor works in the 'compositor coordinate system' and an
// output corresponds to a rectangular area in that space that is
// actually visible.  This typically corresponds to a monitor that
// displays part of the compositor space.  This object is published
// as global during start up, or when a monitor is hotplugged.
pub struct WlOutput {
}

impl WlOutput {
    // release the output object
    //
    // Using this request a client can tell the server that it is not going to
    // use the output object anymore.
    pub fn release(
        request: Arc<RefCell<WlOutput>>,
        session: &mut super::super::session::Session,
        tx: tokio::sync::mpsc::Sender<Box<super::super::event::Event + Send>>,
        sender_object_id: u32,
    ) -> Box<futures::future::Future<Item = (), Error = ()>> {
        Box::new(futures::future::ok(()))
    }
}
