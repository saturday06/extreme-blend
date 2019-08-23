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
pub const VERSION: u32 = 4;

#[allow(unused_variables)]
#[allow(dead_code)]
pub fn dispatch_request(
    context: crate::protocol::session::Context<crate::protocol::wayland::wl_surface::WlSurface>,
    opcode: u16,
    args: Vec<u8>,
) -> Box<futures::future::Future<Item = crate::protocol::session::Session, Error = ()> + Send> {
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
            return Box::new(super::WlSurface::destroy(context).and_then(
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
        1 => {
            let arg_buffer = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
                x
            } else {
                return context.invalid_method_dispatch(format!(
                    "opcode={} args={:?} not found",
                    opcode, args
                ));
            };
            let arg_x = if let Ok(x) = cursor.read_i32::<NativeEndian>() {
                x
            } else {
                return context.invalid_method_dispatch(format!(
                    "opcode={} args={:?} not found",
                    opcode, args
                ));
            };
            let arg_y = if let Ok(x) = cursor.read_i32::<NativeEndian>() {
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

                NativeEndian::write_u32(&mut dst[encode_offset..], arg_buffer);
                encode_offset += 4;
                NativeEndian::write_i32(&mut dst[encode_offset..], arg_x);
                encode_offset += 4;
                NativeEndian::write_i32(&mut dst[encode_offset..], arg_y);
                encode_offset += 4;
                let _ = encode_offset;
                dst
            };
            return Box::new(
                super::WlSurface::attach(context, arg_buffer, arg_x, arg_y).and_then(
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
            let arg_x = if let Ok(x) = cursor.read_i32::<NativeEndian>() {
                x
            } else {
                return context.invalid_method_dispatch(format!(
                    "opcode={} args={:?} not found",
                    opcode, args
                ));
            };
            let arg_y = if let Ok(x) = cursor.read_i32::<NativeEndian>() {
                x
            } else {
                return context.invalid_method_dispatch(format!(
                    "opcode={} args={:?} not found",
                    opcode, args
                ));
            };
            let arg_width = if let Ok(x) = cursor.read_i32::<NativeEndian>() {
                x
            } else {
                return context.invalid_method_dispatch(format!(
                    "opcode={} args={:?} not found",
                    opcode, args
                ));
            };
            let arg_height = if let Ok(x) = cursor.read_i32::<NativeEndian>() {
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
                let total_len = 8 + 4 + 4 + 4 + 4;
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

                NativeEndian::write_i32(&mut dst[encode_offset..], arg_x);
                encode_offset += 4;
                NativeEndian::write_i32(&mut dst[encode_offset..], arg_y);
                encode_offset += 4;
                NativeEndian::write_i32(&mut dst[encode_offset..], arg_width);
                encode_offset += 4;
                NativeEndian::write_i32(&mut dst[encode_offset..], arg_height);
                encode_offset += 4;
                let _ = encode_offset;
                dst
            };
            return Box::new(
                super::WlSurface::damage(context, arg_x, arg_y, arg_width, arg_height).and_then(
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
        3 => {
            let arg_callback = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
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
                let total_len = 8 + 4;
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

                NativeEndian::write_u32(&mut dst[encode_offset..], arg_callback);
                encode_offset += 4;
                let _ = encode_offset;
                dst
            };
            return Box::new(super::WlSurface::frame(context, arg_callback).and_then(
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
            let arg_region = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
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
                let total_len = 8 + 4;
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

                NativeEndian::write_u32(&mut dst[encode_offset..], arg_region);
                encode_offset += 4;
                let _ = encode_offset;
                dst
            };
            return Box::new(
                super::WlSurface::set_opaque_region(context, arg_region).and_then(
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
        5 => {
            let arg_region = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
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
                let total_len = 8 + 4;
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

                NativeEndian::write_u32(&mut dst[encode_offset..], arg_region);
                encode_offset += 4;
                let _ = encode_offset;
                dst
            };
            return Box::new(
                super::WlSurface::set_input_region(context, arg_region).and_then(
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
        6 => {
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
            return Box::new(super::WlSurface::commit(context).and_then(
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
        7 => {
            let arg_transform = if let Ok(x) = cursor.read_i32::<NativeEndian>() {
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
                let total_len = 8 + 4;
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

                NativeEndian::write_i32(&mut dst[encode_offset..], arg_transform);
                encode_offset += 4;
                let _ = encode_offset;
                dst
            };
            return Box::new(
                super::WlSurface::set_buffer_transform(context, arg_transform).and_then(
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
        8 => {
            let arg_scale = if let Ok(x) = cursor.read_i32::<NativeEndian>() {
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
                let total_len = 8 + 4;
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

                NativeEndian::write_i32(&mut dst[encode_offset..], arg_scale);
                encode_offset += 4;
                let _ = encode_offset;
                dst
            };
            return Box::new(
                super::WlSurface::set_buffer_scale(context, arg_scale).and_then(
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
        9 => {
            let arg_x = if let Ok(x) = cursor.read_i32::<NativeEndian>() {
                x
            } else {
                return context.invalid_method_dispatch(format!(
                    "opcode={} args={:?} not found",
                    opcode, args
                ));
            };
            let arg_y = if let Ok(x) = cursor.read_i32::<NativeEndian>() {
                x
            } else {
                return context.invalid_method_dispatch(format!(
                    "opcode={} args={:?} not found",
                    opcode, args
                ));
            };
            let arg_width = if let Ok(x) = cursor.read_i32::<NativeEndian>() {
                x
            } else {
                return context.invalid_method_dispatch(format!(
                    "opcode={} args={:?} not found",
                    opcode, args
                ));
            };
            let arg_height = if let Ok(x) = cursor.read_i32::<NativeEndian>() {
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
                let total_len = 8 + 4 + 4 + 4 + 4;
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

                NativeEndian::write_i32(&mut dst[encode_offset..], arg_x);
                encode_offset += 4;
                NativeEndian::write_i32(&mut dst[encode_offset..], arg_y);
                encode_offset += 4;
                NativeEndian::write_i32(&mut dst[encode_offset..], arg_width);
                encode_offset += 4;
                NativeEndian::write_i32(&mut dst[encode_offset..], arg_height);
                encode_offset += 4;
                let _ = encode_offset;
                dst
            };
            return Box::new(
                super::WlSurface::damage_buffer(context, arg_x, arg_y, arg_width, arg_height)
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

impl Into<crate::protocol::resource::Resource> for crate::protocol::wayland::wl_surface::WlSurface {
    fn into(self) -> crate::protocol::resource::Resource {
        crate::protocol::resource::Resource::WlSurface(self)
    }
}
