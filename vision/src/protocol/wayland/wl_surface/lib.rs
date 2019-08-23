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
) -> Box<dyn futures::future::Future<Item = crate::protocol::session::Session, Error = ()> + Send> {
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
            return Box::new(super::WlSurface::destroy(context).and_then(
                |(session, next_action)| -> Box<
                    dyn futures::future::Future<
                            Item = crate::protocol::session::Session,
                            Error = (),
                        > + Send,
                > { Box::new(futures::future::ok(session)) },
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
            return Box::new(
                super::WlSurface::attach(context, arg_buffer, arg_x, arg_y).and_then(
                    |(session, next_action)| -> Box<
                        dyn futures::future::Future<
                                Item = crate::protocol::session::Session,
                                Error = (),
                            > + Send,
                    > { Box::new(futures::future::ok(session)) },
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
            return Box::new(
                super::WlSurface::damage(context, arg_x, arg_y, arg_width, arg_height).and_then(
                    |(session, next_action)| -> Box<
                        dyn futures::future::Future<
                                Item = crate::protocol::session::Session,
                                Error = (),
                            > + Send,
                    > { Box::new(futures::future::ok(session)) },
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
            return Box::new(super::WlSurface::frame(context, arg_callback).and_then(
                |(session, next_action)| -> Box<
                    dyn futures::future::Future<
                            Item = crate::protocol::session::Session,
                            Error = (),
                        > + Send,
                > { Box::new(futures::future::ok(session)) },
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
            return Box::new(
                super::WlSurface::set_opaque_region(context, arg_region).and_then(
                    |(session, next_action)| -> Box<
                        dyn futures::future::Future<
                                Item = crate::protocol::session::Session,
                                Error = (),
                            > + Send,
                    > { Box::new(futures::future::ok(session)) },
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
            return Box::new(
                super::WlSurface::set_input_region(context, arg_region).and_then(
                    |(session, next_action)| -> Box<
                        dyn futures::future::Future<
                                Item = crate::protocol::session::Session,
                                Error = (),
                            > + Send,
                    > { Box::new(futures::future::ok(session)) },
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
            return Box::new(super::WlSurface::commit(context).and_then(
                |(session, next_action)| -> Box<
                    dyn futures::future::Future<
                            Item = crate::protocol::session::Session,
                            Error = (),
                        > + Send,
                > { Box::new(futures::future::ok(session)) },
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
            return Box::new(
                super::WlSurface::set_buffer_transform(context, arg_transform).and_then(
                    |(session, next_action)| -> Box<
                        dyn futures::future::Future<
                                Item = crate::protocol::session::Session,
                                Error = (),
                            > + Send,
                    > { Box::new(futures::future::ok(session)) },
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
            return Box::new(
                super::WlSurface::set_buffer_scale(context, arg_scale).and_then(
                    |(session, next_action)| -> Box<
                        dyn futures::future::Future<
                                Item = crate::protocol::session::Session,
                                Error = (),
                            > + Send,
                    > { Box::new(futures::future::ok(session)) },
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
            return Box::new(
                super::WlSurface::damage_buffer(context, arg_x, arg_y, arg_width, arg_height)
                    .and_then(
                        |(session, next_action)| -> Box<
                            dyn futures::future::Future<
                                    Item = crate::protocol::session::Session,
                                    Error = (),
                                > + Send,
                        > { Box::new(futures::future::ok(session)) },
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
