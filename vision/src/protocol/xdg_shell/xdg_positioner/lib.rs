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
        crate::protocol::xdg_shell::xdg_positioner::XdgPositioner,
    >,
    opcode: u16,
    args: Vec<u8>,
) -> Box<futures::future::Future<Item = crate::protocol::session::Session, Error = ()> + Send> {
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
            return Box::new(super::XdgPositioner::destroy(context).and_then(
                |(session, next_action)| -> Box<
                    futures::future::Future<Item = crate::protocol::session::Session, Error = ()>
                        + Send,
                > {
                    match next_action {
                        NextAction::Nop => Box::new(futures::future::ok(session)),
                        NextAction::Relay => Box::new(
                            futures::future::ok(()).and_then(|_| futures::future::ok(session)),
                        ),
                        NextAction::RelayWait => Box::new(
                            futures::future::ok(())
                                .and_then(|_| futures::future::ok(()))
                                .and_then(|_| futures::future::ok(session)),
                        ),
                    }
                },
            ));
        }
        1 => {
            let width = if let Ok(x) = cursor.read_i32::<NativeEndian>() {
                x
            } else {
                return context.invalid_method_dispatch(format!(
                    "opcode={} args={:?} not found",
                    opcode, args
                ));
            };
            let height = if let Ok(x) = cursor.read_i32::<NativeEndian>() {
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
                super::XdgPositioner::set_size(context, width, height).and_then(
                    |(session, next_action)| -> Box<
                        futures::future::Future<
                                Item = crate::protocol::session::Session,
                                Error = (),
                            > + Send,
                    > {
                        match next_action {
                            NextAction::Nop => Box::new(futures::future::ok(session)),
                            NextAction::Relay => Box::new(
                                futures::future::ok(()).and_then(|_| futures::future::ok(session)),
                            ),
                            NextAction::RelayWait => Box::new(
                                futures::future::ok(())
                                    .and_then(|_| futures::future::ok(()))
                                    .and_then(|_| futures::future::ok(session)),
                            ),
                        }
                    },
                ),
            );
        }
        2 => {
            let x = if let Ok(x) = cursor.read_i32::<NativeEndian>() {
                x
            } else {
                return context.invalid_method_dispatch(format!(
                    "opcode={} args={:?} not found",
                    opcode, args
                ));
            };
            let y = if let Ok(x) = cursor.read_i32::<NativeEndian>() {
                x
            } else {
                return context.invalid_method_dispatch(format!(
                    "opcode={} args={:?} not found",
                    opcode, args
                ));
            };
            let width = if let Ok(x) = cursor.read_i32::<NativeEndian>() {
                x
            } else {
                return context.invalid_method_dispatch(format!(
                    "opcode={} args={:?} not found",
                    opcode, args
                ));
            };
            let height = if let Ok(x) = cursor.read_i32::<NativeEndian>() {
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
                super::XdgPositioner::set_anchor_rect(context, x, y, width, height).and_then(
                    |(session, next_action)| -> Box<
                        futures::future::Future<
                                Item = crate::protocol::session::Session,
                                Error = (),
                            > + Send,
                    > {
                        match next_action {
                            NextAction::Nop => Box::new(futures::future::ok(session)),
                            NextAction::Relay => Box::new(
                                futures::future::ok(()).and_then(|_| futures::future::ok(session)),
                            ),
                            NextAction::RelayWait => Box::new(
                                futures::future::ok(())
                                    .and_then(|_| futures::future::ok(()))
                                    .and_then(|_| futures::future::ok(session)),
                            ),
                        }
                    },
                ),
            );
        }
        3 => {
            let anchor = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
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
            return Box::new(super::XdgPositioner::set_anchor(context, anchor).and_then(
                |(session, next_action)| -> Box<
                    futures::future::Future<Item = crate::protocol::session::Session, Error = ()>
                        + Send,
                > {
                    match next_action {
                        NextAction::Nop => Box::new(futures::future::ok(session)),
                        NextAction::Relay => Box::new(
                            futures::future::ok(()).and_then(|_| futures::future::ok(session)),
                        ),
                        NextAction::RelayWait => Box::new(
                            futures::future::ok(())
                                .and_then(|_| futures::future::ok(()))
                                .and_then(|_| futures::future::ok(session)),
                        ),
                    }
                },
            ));
        }
        4 => {
            let gravity = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
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
                super::XdgPositioner::set_gravity(context, gravity).and_then(
                    |(session, next_action)| -> Box<
                        futures::future::Future<
                                Item = crate::protocol::session::Session,
                                Error = (),
                            > + Send,
                    > {
                        match next_action {
                            NextAction::Nop => Box::new(futures::future::ok(session)),
                            NextAction::Relay => Box::new(
                                futures::future::ok(()).and_then(|_| futures::future::ok(session)),
                            ),
                            NextAction::RelayWait => Box::new(
                                futures::future::ok(())
                                    .and_then(|_| futures::future::ok(()))
                                    .and_then(|_| futures::future::ok(session)),
                            ),
                        }
                    },
                ),
            );
        }
        5 => {
            let constraint_adjustment = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
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
                super::XdgPositioner::set_constraint_adjustment(context, constraint_adjustment)
                    .and_then(
                        |(session, next_action)| -> Box<
                            futures::future::Future<
                                    Item = crate::protocol::session::Session,
                                    Error = (),
                                > + Send,
                        > {
                            match next_action {
                                NextAction::Nop => Box::new(futures::future::ok(session)),
                                NextAction::Relay => Box::new(
                                    futures::future::ok(())
                                        .and_then(|_| futures::future::ok(session)),
                                ),
                                NextAction::RelayWait => Box::new(
                                    futures::future::ok(())
                                        .and_then(|_| futures::future::ok(()))
                                        .and_then(|_| futures::future::ok(session)),
                                ),
                            }
                        },
                    ),
            );
        }
        6 => {
            let x = if let Ok(x) = cursor.read_i32::<NativeEndian>() {
                x
            } else {
                return context.invalid_method_dispatch(format!(
                    "opcode={} args={:?} not found",
                    opcode, args
                ));
            };
            let y = if let Ok(x) = cursor.read_i32::<NativeEndian>() {
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
            return Box::new(super::XdgPositioner::set_offset(context, x, y).and_then(
                |(session, next_action)| -> Box<
                    futures::future::Future<Item = crate::protocol::session::Session, Error = ()>
                        + Send,
                > {
                    match next_action {
                        NextAction::Nop => Box::new(futures::future::ok(session)),
                        NextAction::Relay => Box::new(
                            futures::future::ok(()).and_then(|_| futures::future::ok(session)),
                        ),
                        NextAction::RelayWait => Box::new(
                            futures::future::ok(())
                                .and_then(|_| futures::future::ok(()))
                                .and_then(|_| futures::future::ok(session)),
                        ),
                    }
                },
            ));
        }
        _ => {}
    };
    return context.invalid_method_dispatch(format!("opcode={} args={:?} not found", opcode, args));
}

impl Into<crate::protocol::resource::Resource>
    for crate::protocol::xdg_shell::xdg_positioner::XdgPositioner
{
    fn into(self) -> crate::protocol::resource::Resource {
        crate::protocol::resource::Resource::XdgPositioner(self)
    }
}
