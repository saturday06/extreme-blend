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
pub const VERSION: u32 = 3;

#[allow(unused_variables)]
#[allow(dead_code)]
pub fn dispatch_request(
    mut context: crate::protocol::session::Context<
        crate::protocol::wayland::wl_data_offer::WlDataOffer,
    >,
    opcode: u16,
    args: Vec<u8>,
) -> Box<futures::future::Future<Item = crate::protocol::session::Session, Error = ()> + Send> {
    let sender_object_id = context.sender_object_id;
    #[allow(unused_mut)]
    let mut cursor = Cursor::new(&args);
    match opcode {
        0 => {
            let arg_serial = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
                x
            } else {
                return context.invalid_method_dispatch(format!(
                    "opcode={} args={:?} not found",
                    opcode, args
                ));
            };
            let arg_mime_type = {
                let buf_len = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
                    x
                } else {
                    return context.invalid_method_dispatch(format!(
                        "opcode={} args={:?} not found",
                        opcode, args
                    ));
                };
                let padded_buf_len = (buf_len + 3) / 4 * 4;
                let mut buf = Vec::new();
                buf.resize(buf_len as usize, 0);
                if cursor.read_exact(&mut buf).is_err() {
                    return context.invalid_method_dispatch(format!(
                        "opcode={} args={:?} not found",
                        opcode, args
                    ));
                }
                let s = if let Ok(x) = String::from_utf8(buf) {
                    x
                } else {
                    return context.invalid_method_dispatch(format!(
                        "opcode={} args={:?} not found",
                        opcode, args
                    ));
                };
                cursor.set_position(cursor.position() + u64::from(padded_buf_len - buf_len));
                s
            };

            if Ok(cursor.position()) != args.len().try_into() {
                return context.invalid_method_dispatch(format!(
                    "opcode={} args={:?} not found",
                    opcode, args
                ));
            }
            let relay_buf = {
                let total_len = 8 + 4 + { 4 + (arg_mime_type.len() + 1 + 3) / 4 * 4 };
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

                NativeEndian::write_u32(&mut dst[encode_offset..], arg_serial);
                encode_offset += 4;
                NativeEndian::write_u32(
                    &mut dst[encode_offset..],
                    (arg_mime_type.len() + 1) as u32,
                );
                {
                    let mut aligned = arg_mime_type.clone();
                    aligned.push(0u8.into());
                    while aligned.len() % 4 != 0 {
                        aligned.push(0u8.into());
                    }
                    dst[(encode_offset + 4)..(encode_offset + 4 + aligned.len())]
                        .copy_from_slice(aligned.as_bytes());
                }

                encode_offset += { 4 + (arg_mime_type.len() + 1 + 3) / 4 * 4 };
                let _ = encode_offset;
                dst
            };
            return Box::new(
                super::WlDataOffer::accept(context, arg_serial, arg_mime_type).and_then(
                    |(session, next_action)| -> Box<
                        futures::future::Future<
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
        1 => {
            let arg_mime_type = {
                let buf_len = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
                    x
                } else {
                    return context.invalid_method_dispatch(format!(
                        "opcode={} args={:?} not found",
                        opcode, args
                    ));
                };
                let padded_buf_len = (buf_len + 3) / 4 * 4;
                let mut buf = Vec::new();
                buf.resize(buf_len as usize, 0);
                if cursor.read_exact(&mut buf).is_err() {
                    return context.invalid_method_dispatch(format!(
                        "opcode={} args={:?} not found",
                        opcode, args
                    ));
                }
                let s = if let Ok(x) = String::from_utf8(buf) {
                    x
                } else {
                    return context.invalid_method_dispatch(format!(
                        "opcode={} args={:?} not found",
                        opcode, args
                    ));
                };
                cursor.set_position(cursor.position() + u64::from(padded_buf_len - buf_len));
                s
            };
            if context.fds.len() == 0 {
                return context.invalid_method_dispatch(format!(
                    "opcode={} args={:?} not found",
                    opcode, args
                ));
            }
            let arg_fd = {
                let rest = context.fds.split_off(1);
                let first = context.fds.pop().expect("fds");
                context.fds = rest;
                first
            };

            if Ok(cursor.position()) != args.len().try_into() {
                return context.invalid_method_dispatch(format!(
                    "opcode={} args={:?} not found",
                    opcode, args
                ));
            }
            let relay_buf = {
                let total_len = 8 + { 4 + (arg_mime_type.len() + 1 + 3) / 4 * 4 } + 4;
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

                NativeEndian::write_u32(
                    &mut dst[encode_offset..],
                    (arg_mime_type.len() + 1) as u32,
                );
                {
                    let mut aligned = arg_mime_type.clone();
                    aligned.push(0u8.into());
                    while aligned.len() % 4 != 0 {
                        aligned.push(0u8.into());
                    }
                    dst[(encode_offset + 4)..(encode_offset + 4 + aligned.len())]
                        .copy_from_slice(aligned.as_bytes());
                }

                encode_offset += { 4 + (arg_mime_type.len() + 1 + 3) / 4 * 4 };
                NativeEndian::write_i32(&mut dst[encode_offset..], arg_fd);
                encode_offset += 4;
                let _ = encode_offset;
                dst
            };
            return Box::new(
                super::WlDataOffer::receive(context, arg_mime_type, arg_fd).and_then(
                    |(session, next_action)| -> Box<
                        futures::future::Future<
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
        2 => {
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
            return Box::new(super::WlDataOffer::destroy(context).and_then(
                |(session, next_action)| -> Box<
                    futures::future::Future<Item = crate::protocol::session::Session, Error = ()>
                        + Send,
                > {
                    match next_action {
                        NextAction::Nop => Box::new(futures::future::ok(session)),
                        NextAction::Relay => session.relay(relay_buf),
                        NextAction::RelayWait => session.relay_wait(relay_buf),
                    }
                },
            ));
        }
        3 => {
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
            return Box::new(super::WlDataOffer::finish(context).and_then(
                |(session, next_action)| -> Box<
                    futures::future::Future<Item = crate::protocol::session::Session, Error = ()>
                        + Send,
                > {
                    match next_action {
                        NextAction::Nop => Box::new(futures::future::ok(session)),
                        NextAction::Relay => session.relay(relay_buf),
                        NextAction::RelayWait => session.relay_wait(relay_buf),
                    }
                },
            ));
        }
        4 => {
            let arg_dnd_actions = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
                x
            } else {
                return context.invalid_method_dispatch(format!(
                    "opcode={} args={:?} not found",
                    opcode, args
                ));
            };
            let arg_preferred_action = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
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
                let total_len = 8 + 4 + 4;
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

                NativeEndian::write_u32(&mut dst[encode_offset..], arg_dnd_actions);
                encode_offset += 4;
                NativeEndian::write_u32(&mut dst[encode_offset..], arg_preferred_action);
                encode_offset += 4;
                let _ = encode_offset;
                dst
            };
            return Box::new(
                super::WlDataOffer::set_actions(context, arg_dnd_actions, arg_preferred_action)
                    .and_then(
                        |(session, next_action)| -> Box<
                            futures::future::Future<
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
    for crate::protocol::wayland::wl_data_offer::WlDataOffer
{
    fn into(self) -> crate::protocol::resource::Resource {
        crate::protocol::resource::Resource::WlDataOffer(self)
    }
}
