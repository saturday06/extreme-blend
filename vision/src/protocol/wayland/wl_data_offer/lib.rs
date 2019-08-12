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

#[allow(unused_imports)] use byteorder::{ByteOrder, NativeEndian, ReadBytesExt};
#[allow(unused_imports)] use futures::future::Future;
#[allow(unused_imports)] use futures::sink::Sink;
#[allow(unused_imports)] use std::convert::TryInto;
#[allow(unused_imports)] use std::io::{Cursor, Read};
#[allow(unused_imports)] use std::sync::{Arc, RwLock};

#[allow(dead_code)]
pub const VERSION: u32 = 3;

#[allow(unused_variables)]
#[allow(dead_code)]
pub fn dispatch_request(mut context: crate::protocol::session::Context<crate::protocol::wayland::wl_data_offer::WlDataOffer>, opcode: u16, args: Vec<u8>) -> Box<futures::future::Future<Item = crate::protocol::session::Session, Error = ()> + Send> {
    #[allow(unused_mut)] let mut cursor = Cursor::new(&args);
    match opcode {
        0 => {
            let serial = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
                x 
            } else {
                return context.invalid_method(format!(
                    "opcode={} args={:?} not found", opcode, args
                ))
            };
            let mime_type = {
                let buf_len = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
                    x
                } else {
                return context.invalid_method(format!(
                    "opcode={} args={:?} not found", opcode, args
                ))
                };
                let padded_buf_len = (buf_len + 3) / 4 * 4;
                let mut buf = Vec::new();
                buf.resize(buf_len as usize, 0);
                if let Err(_) = cursor.read_exact(&mut buf) {
                return context.invalid_method(format!(
                    "opcode={} args={:?} not found", opcode, args
                ))
                }
                let s = if let Ok(x) = String::from_utf8(buf) {
                    x
                } else {
                return context.invalid_method(format!(
                    "opcode={} args={:?} not found", opcode, args
                ))
                };
                cursor.set_position(cursor.position() + (padded_buf_len - buf_len) as u64);
                s
            };

            if Ok(cursor.position()) != args.len().try_into() {
                return context.invalid_method(format!(
                    "opcode={} args={:?} not found", opcode, args
                ));
            }
            return super::WlDataOffer::accept(context, serial, mime_type);
        },
        1 => {
            let mime_type = {
                let buf_len = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
                    x
                } else {
                return context.invalid_method(format!(
                    "opcode={} args={:?} not found", opcode, args
                ))
                };
                let padded_buf_len = (buf_len + 3) / 4 * 4;
                let mut buf = Vec::new();
                buf.resize(buf_len as usize, 0);
                if let Err(_) = cursor.read_exact(&mut buf) {
                return context.invalid_method(format!(
                    "opcode={} args={:?} not found", opcode, args
                ))
                }
                let s = if let Ok(x) = String::from_utf8(buf) {
                    x
                } else {
                return context.invalid_method(format!(
                    "opcode={} args={:?} not found", opcode, args
                ))
                };
                cursor.set_position(cursor.position() + (padded_buf_len - buf_len) as u64);
                s
            };
            if context.fds.len() == 0 {
                return context.invalid_method(format!(
                    "opcode={} args={:?} not found", opcode, args
                ))
            }
            let fd = {
                let rest = context.fds.split_off(1);
                let first = context.fds.pop().expect("fds");
                context.fds = rest;
                first
            };

            if Ok(cursor.position()) != args.len().try_into() {
                return context.invalid_method(format!(
                    "opcode={} args={:?} not found", opcode, args
                ));
            }
            return super::WlDataOffer::receive(context, mime_type, fd);
        },
        2 => {

            if Ok(cursor.position()) != args.len().try_into() {
                return context.invalid_method(format!(
                    "opcode={} args={:?} not found", opcode, args
                ));
            }
            return super::WlDataOffer::destroy(context);
        },
        3 => {

            if Ok(cursor.position()) != args.len().try_into() {
                return context.invalid_method(format!(
                    "opcode={} args={:?} not found", opcode, args
                ));
            }
            return super::WlDataOffer::finish(context);
        },
        4 => {
            let dnd_actions = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
                x 
            } else {
                return context.invalid_method(format!(
                    "opcode={} args={:?} not found", opcode, args
                ))
            };
            let preferred_action = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
                x 
            } else {
                return context.invalid_method(format!(
                    "opcode={} args={:?} not found", opcode, args
                ))
            };

            if Ok(cursor.position()) != args.len().try_into() {
                return context.invalid_method(format!(
                    "opcode={} args={:?} not found", opcode, args
                ));
            }
            return super::WlDataOffer::set_actions(context, dnd_actions, preferred_action);
        },
        _ => {},
    };
    Box::new(futures::future::ok(context.into()))
}

impl Into<crate::protocol::resource::Resource> for crate::protocol::wayland::wl_data_offer::WlDataOffer {
    fn into(self) -> crate::protocol::resource::Resource {
        crate::protocol::resource::Resource::WlDataOffer(self)
    }
}
