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

#[allow(unused_imports)]
use crate::protocol::session::NextAction;
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
pub const VERSION: u32 = 1;

#[allow(unused_variables)]
#[allow(dead_code)]
pub fn dispatch_request(
    context: crate::protocol::session::Context<
        crate::protocol::wayland::wl_subcompositor::WlSubcompositor,
    >,
    opcode: u16,
    args: Vec<u8>,
) -> Box<dyn futures::future::Future<Item = crate::protocol::session::Session, Error = ()> + Send> {
    let sender_object_id = context.sender_object_id;
    #[allow(unused_mut)]
    let mut cursor = Cursor::new(&args);
    match opcode {
        0 => {
            if Ok(cursor.position()) != args.len().try_into() {
                return context.invalid_method_dispatch(format!(
                    "opcode={} args={:?} not found",
                    opcode, args
                ));
            }
            let relay_buf = {
                let total_len = 8;
                if total_len > 0xffff {
                    println!("Oops! total_len={}", total_len);
                    return Box::new(futures::future::err(()));
                }

                let mut dst: Vec<u8> = Vec::new();
                dst.resize(total_len, 0);

                NativeEndian::write_u32(&mut dst[0..], sender_object_id);
                NativeEndian::write_u32(
                    &mut dst[4..],
                    (total_len << 16) as u32 | u32::from(opcode),
                );

                #[allow(unused_mut)]
                let mut encode_offset = 8;

                let _ = encode_offset;
                dst
            };
            return Box::new(super::WlSubcompositor::destroy(context).and_then(
                |(session, next_action)| -> Box<
                    dyn futures::future::Future<
                            Item = crate::protocol::session::Session,
                            Error = (),
                        > + Send,
                > {
                    match next_action {
                        NextAction::Nop => Box::new(futures::future::ok(session)),
                        NextAction::Relay => session.relay(relay_buf),
                        NextAction::RelayWait => session.relay_wait(relay_buf),
                    }
                },
            ));
        }
        1 => {
            let arg_id = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
                x
            } else {
                return context.invalid_method_dispatch(format!(
                    "opcode={} args={:?} not found",
                    opcode, args
                ));
            };
            let arg_surface = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
                x
            } else {
                return context.invalid_method_dispatch(format!(
                    "opcode={} args={:?} not found",
                    opcode, args
                ));
            };
            let arg_parent = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
                x
            } else {
                return context.invalid_method_dispatch(format!(
                    "opcode={} args={:?} not found",
                    opcode, args
                ));
            };

            if Ok(cursor.position()) != args.len().try_into() {
                return context.invalid_method_dispatch(format!(
                    "opcode={} args={:?} not found",
                    opcode, args
                ));
            }
            let relay_buf = {
                let total_len = 8 + 4 + 4 + 4;
                if total_len > 0xffff {
                    println!("Oops! total_len={}", total_len);
                    return Box::new(futures::future::err(()));
                }

                let mut dst: Vec<u8> = Vec::new();
                dst.resize(total_len, 0);

                NativeEndian::write_u32(&mut dst[0..], sender_object_id);
                NativeEndian::write_u32(
                    &mut dst[4..],
                    (total_len << 16) as u32 | u32::from(opcode),
                );

                #[allow(unused_mut)]
                let mut encode_offset = 8;

                NativeEndian::write_u32(&mut dst[encode_offset..], arg_id);
                encode_offset += 4;
                NativeEndian::write_u32(&mut dst[encode_offset..], arg_surface);
                encode_offset += 4;
                NativeEndian::write_u32(&mut dst[encode_offset..], arg_parent);
                encode_offset += 4;
                let _ = encode_offset;
                dst
            };
            return Box::new(
                super::WlSubcompositor::get_subsurface(context, arg_id, arg_surface, arg_parent)
                    .and_then(
                        |(session, next_action)| -> Box<
                            dyn futures::future::Future<
                                    Item = crate::protocol::session::Session,
                                    Error = (),
                                > + Send,
                        > {
                            match next_action {
                                NextAction::Nop => Box::new(futures::future::ok(session)),
                                NextAction::Relay => session.relay(relay_buf),
                                NextAction::RelayWait => session.relay_wait(relay_buf),
                            }
                        },
                    ),
            );
        }
        _ => {}
    };
    return context.invalid_method_dispatch(format!("opcode={} args={:?} not found", opcode, args));
}

impl Into<crate::protocol::resource::Resource>
    for crate::protocol::wayland::wl_subcompositor::WlSubcompositor
{
    fn into(self) -> crate::protocol::resource::Resource {
        crate::protocol::resource::Resource::WlSubcompositor(self)
    }
}
