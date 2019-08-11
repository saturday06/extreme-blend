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
    context: crate::protocol::session::Context<
        crate::protocol::xdg_shell::xdg_toplevel::XdgToplevel,
    >,
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
            return super::XdgToplevel::destroy(context);
        }
        1 => {
            let parent = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
                x
            } else {
                return context
                    .invalid_method(format!("opcode={} args={:?} not found", opcode, args));
            };

            if Ok(cursor.position()) != args.len().try_into() {
                return context
                    .invalid_method(format!("opcode={} args={:?} not found", opcode, args));
            }
            return super::XdgToplevel::set_parent(context, parent);
        }
        2 => {
            let title = {
                let buf_len = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
                    x
                } else {
                    return context
                        .invalid_method(format!("opcode={} args={:?} not found", opcode, args));
                };
                let padded_buf_len = (buf_len + 3) / 4 * 4;
                let mut buf = Vec::new();
                buf.resize(buf_len as usize, 0);
                if let Err(_) = cursor.read_exact(&mut buf) {
                    return context
                        .invalid_method(format!("opcode={} args={:?} not found", opcode, args));
                }
                let s = if let Ok(x) = String::from_utf8(buf) {
                    x
                } else {
                    return context
                        .invalid_method(format!("opcode={} args={:?} not found", opcode, args));
                };
                cursor.set_position(cursor.position() + (padded_buf_len - buf_len) as u64);
                s
            };

            if Ok(cursor.position()) != args.len().try_into() {
                return context
                    .invalid_method(format!("opcode={} args={:?} not found", opcode, args));
            }
            return super::XdgToplevel::set_title(context, title);
        }
        3 => {
            let app_id = {
                let buf_len = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
                    x
                } else {
                    return context
                        .invalid_method(format!("opcode={} args={:?} not found", opcode, args));
                };
                let padded_buf_len = (buf_len + 3) / 4 * 4;
                let mut buf = Vec::new();
                buf.resize(buf_len as usize, 0);
                if let Err(_) = cursor.read_exact(&mut buf) {
                    return context
                        .invalid_method(format!("opcode={} args={:?} not found", opcode, args));
                }
                let s = if let Ok(x) = String::from_utf8(buf) {
                    x
                } else {
                    return context
                        .invalid_method(format!("opcode={} args={:?} not found", opcode, args));
                };
                cursor.set_position(cursor.position() + (padded_buf_len - buf_len) as u64);
                s
            };

            if Ok(cursor.position()) != args.len().try_into() {
                return context
                    .invalid_method(format!("opcode={} args={:?} not found", opcode, args));
            }
            return super::XdgToplevel::set_app_id(context, app_id);
        }
        4 => {
            let seat = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
                x
            } else {
                return context
                    .invalid_method(format!("opcode={} args={:?} not found", opcode, args));
            };
            let serial = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
                x
            } else {
                return context
                    .invalid_method(format!("opcode={} args={:?} not found", opcode, args));
            };
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

            if Ok(cursor.position()) != args.len().try_into() {
                return context
                    .invalid_method(format!("opcode={} args={:?} not found", opcode, args));
            }
            return super::XdgToplevel::show_window_menu(context, seat, serial, x, y);
        }
        5 => {
            let seat = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
                x
            } else {
                return context
                    .invalid_method(format!("opcode={} args={:?} not found", opcode, args));
            };
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
            return super::XdgToplevel::move_fn(context, seat, serial);
        }
        6 => {
            let seat = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
                x
            } else {
                return context
                    .invalid_method(format!("opcode={} args={:?} not found", opcode, args));
            };
            let serial = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
                x
            } else {
                return context
                    .invalid_method(format!("opcode={} args={:?} not found", opcode, args));
            };
            let edges = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
                x
            } else {
                return context
                    .invalid_method(format!("opcode={} args={:?} not found", opcode, args));
            };

            if Ok(cursor.position()) != args.len().try_into() {
                return context
                    .invalid_method(format!("opcode={} args={:?} not found", opcode, args));
            }
            return super::XdgToplevel::resize(context, seat, serial, edges);
        }
        7 => {
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
            return super::XdgToplevel::set_max_size(context, width, height);
        }
        8 => {
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
            return super::XdgToplevel::set_min_size(context, width, height);
        }
        9 => {
            if Ok(cursor.position()) != args.len().try_into() {
                return context
                    .invalid_method(format!("opcode={} args={:?} not found", opcode, args));
            }
            return super::XdgToplevel::set_maximized(context);
        }
        10 => {
            if Ok(cursor.position()) != args.len().try_into() {
                return context
                    .invalid_method(format!("opcode={} args={:?} not found", opcode, args));
            }
            return super::XdgToplevel::unset_maximized(context);
        }
        11 => {
            let output = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
                x
            } else {
                return context
                    .invalid_method(format!("opcode={} args={:?} not found", opcode, args));
            };

            if Ok(cursor.position()) != args.len().try_into() {
                return context
                    .invalid_method(format!("opcode={} args={:?} not found", opcode, args));
            }
            return super::XdgToplevel::set_fullscreen(context, output);
        }
        12 => {
            if Ok(cursor.position()) != args.len().try_into() {
                return context
                    .invalid_method(format!("opcode={} args={:?} not found", opcode, args));
            }
            return super::XdgToplevel::unset_fullscreen(context);
        }
        13 => {
            if Ok(cursor.position()) != args.len().try_into() {
                return context
                    .invalid_method(format!("opcode={} args={:?} not found", opcode, args));
            }
            return super::XdgToplevel::set_minimized(context);
        }
        _ => {}
    };
    Box::new(futures::future::ok(context.into()))
}

impl Into<crate::protocol::resource::Resource>
    for crate::protocol::xdg_shell::xdg_toplevel::XdgToplevel
{
    fn into(self) -> crate::protocol::resource::Resource {
        crate::protocol::resource::Resource::XdgToplevel(self)
    }
}
