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

#[allow(unused_imports)]
use byteorder::{ByteOrder, NativeEndian, ReadBytesExt};
#[allow(unused_imports)]
use futures::future::Future;
#[allow(unused_imports)]
use futures::sink::Sink;
#[allow(unused_imports)]
use std::convert::TryInto;
#[allow(unused_imports)]
use std::io::{Cursor, Read};
#[allow(unused_imports)]
use std::sync::{Arc, RwLock};

#[allow(dead_code)]
pub const VERSION: u32 = 2;

#[allow(unused_variables)]
#[allow(dead_code)]
pub fn dispatch_request(
    context: crate::protocol::session::Context<crate::protocol::xdg_shell::xdg_surface::XdgSurface>,
    opcode: u16,
    args: Vec<u8>,
) -> Box<futures::future::Future<Item = crate::protocol::session::Session, Error = ()> + Send> {
    #[allow(unused_mut)]
    let mut cursor = Cursor::new(&args);
    match opcode {
        0 => {
            if Ok(cursor.position()) != args.len().try_into() {
                return context
                    .invalid_method(format!("opcode={} args={:?} not found", opcode, args));
            }
            return super::XdgSurface::destroy(context);
        }
        1 => {
            let id = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
                x
            } else {
                return context
                    .invalid_method(format!("opcode={} args={:?} not found", opcode, args));
            };

            if Ok(cursor.position()) != args.len().try_into() {
                return context
                    .invalid_method(format!("opcode={} args={:?} not found", opcode, args));
            }
            return super::XdgSurface::get_toplevel(context, id);
        }
        2 => {
            let id = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
                x
            } else {
                return context
                    .invalid_method(format!("opcode={} args={:?} not found", opcode, args));
            };
            let parent = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
                x
            } else {
                return context
                    .invalid_method(format!("opcode={} args={:?} not found", opcode, args));
            };
            let positioner = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
                x
            } else {
                return context
                    .invalid_method(format!("opcode={} args={:?} not found", opcode, args));
            };

            if Ok(cursor.position()) != args.len().try_into() {
                return context
                    .invalid_method(format!("opcode={} args={:?} not found", opcode, args));
            }
            return super::XdgSurface::get_popup(context, id, parent, positioner);
        }
        3 => {
            let x = if let Ok(x) = cursor.read_i32::<NativeEndian>() {
                x
            } else {
                return context
                    .invalid_method(format!("opcode={} args={:?} not found", opcode, args));
            };
            let y = if let Ok(x) = cursor.read_i32::<NativeEndian>() {
                x
            } else {
                return context
                    .invalid_method(format!("opcode={} args={:?} not found", opcode, args));
            };
            let width = if let Ok(x) = cursor.read_i32::<NativeEndian>() {
                x
            } else {
                return context
                    .invalid_method(format!("opcode={} args={:?} not found", opcode, args));
            };
            let height = if let Ok(x) = cursor.read_i32::<NativeEndian>() {
                x
            } else {
                return context
                    .invalid_method(format!("opcode={} args={:?} not found", opcode, args));
            };

            if Ok(cursor.position()) != args.len().try_into() {
                return context
                    .invalid_method(format!("opcode={} args={:?} not found", opcode, args));
            }
            return super::XdgSurface::set_window_geometry(context, x, y, width, height);
        }
        4 => {
            let serial = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
                x
            } else {
                return context
                    .invalid_method(format!("opcode={} args={:?} not found", opcode, args));
            };

            if Ok(cursor.position()) != args.len().try_into() {
                return context
                    .invalid_method(format!("opcode={} args={:?} not found", opcode, args));
            }
            return super::XdgSurface::ack_configure(context, serial);
        }
        _ => {}
    };
    Box::new(futures::future::ok(context.into()))
}

impl Into<crate::protocol::resource::Resource>
    for crate::protocol::xdg_shell::xdg_surface::XdgSurface
{
    fn into(self) -> crate::protocol::resource::Resource {
        crate::protocol::resource::Resource::XdgSurface(self)
    }
}
