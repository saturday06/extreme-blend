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
use byteorder::{NativeEndian, ReadBytesExt};
#[allow(unused_imports)]
use futures::future::Future;
#[allow(unused_imports)]
use futures::sink::Sink;
#[allow(unused_imports)]
use std::io::{Cursor, Read};
#[allow(unused_imports)]
use std::sync::{Arc, RwLock};

pub mod enums {
    pub enum Anchor {
        None = 0,        //
        Top = 1,         //
        Bottom = 2,      //
        Left = 3,        //
        Right = 4,       //
        TopLeft = 5,     //
        BottomLeft = 6,  //
        TopRight = 7,    //
        BottomRight = 8, //
    }

    // constraint adjustments
    //
    // The constraint adjustment value define ways the compositor will adjust
    // the position of the surface, if the unadjusted position would result
    // in the surface being partly constrained.
    //
    // Whether a surface is considered 'constrained' is left to the compositor
    // to determine. For example, the surface may be partly outside the
    // compositor's defined 'work area', thus necessitating the child surface's
    // position be adjusted until it is entirely inside the work area.
    //
    // The adjustments can be combined, according to a defined precedence: 1)
    // Flip, 2) Slide, 3) Resize.
    pub enum ConstraintAdjustment {
        None = 0,     //
        SlideX = 1,   //
        SlideY = 2,   //
        FlipX = 4,    //
        FlipY = 8,    //
        ResizeX = 16, //
        ResizeY = 32, //
    }

    pub enum Error {
        InvalidInput = 0, // invalid input provided
    }

    pub enum Gravity {
        None = 0,        //
        Top = 1,         //
        Bottom = 2,      //
        Left = 3,        //
        Right = 4,       //
        TopLeft = 5,     //
        BottomLeft = 6,  //
        TopRight = 7,    //
        BottomRight = 8, //
    }
}

pub fn dispatch_request(
    request: Arc<RwLock<XdgPositioner>>,
    session: RwLock<super::super::session::Session>,
    tx: tokio::sync::mpsc::Sender<Box<super::super::event::Event + Send>>,
    sender_object_id: u32,
    opcode: u16,
    args: Vec<u8>,
) -> Box<futures::future::Future<Item = (), Error = ()> + Send> {
    let mut cursor = Cursor::new(&args);
    match opcode {
        0 => return XdgPositioner::destroy(request, session, tx, sender_object_id),
        1 => {
            let width = if let Ok(x) = cursor.read_i32::<NativeEndian>() {
                x
            } else {
                return Box::new(
                    tx.send(Box::new(super::super::wayland::wl_display::events::Error {
                        sender_object_id: 1,
                        object_id: sender_object_id,
                        code: super::super::wayland::wl_display::enums::Error::InvalidMethod as u32,
                        message: format!(
                            "@{} opcode={} args={:?} not found",
                            sender_object_id, opcode, args
                        ),
                    }))
                    .map_err(|_| ())
                    .map(|_tx| ()),
                );
            };
            let height = if let Ok(x) = cursor.read_i32::<NativeEndian>() {
                x
            } else {
                return Box::new(
                    tx.send(Box::new(super::super::wayland::wl_display::events::Error {
                        sender_object_id: 1,
                        object_id: sender_object_id,
                        code: super::super::wayland::wl_display::enums::Error::InvalidMethod as u32,
                        message: format!(
                            "@{} opcode={} args={:?} not found",
                            sender_object_id, opcode, args
                        ),
                    }))
                    .map_err(|_| ())
                    .map(|_tx| ()),
                );
            };
            return XdgPositioner::set_size(request, session, tx, sender_object_id, width, height);
        }
        2 => {
            let x = if let Ok(x) = cursor.read_i32::<NativeEndian>() {
                x
            } else {
                return Box::new(
                    tx.send(Box::new(super::super::wayland::wl_display::events::Error {
                        sender_object_id: 1,
                        object_id: sender_object_id,
                        code: super::super::wayland::wl_display::enums::Error::InvalidMethod as u32,
                        message: format!(
                            "@{} opcode={} args={:?} not found",
                            sender_object_id, opcode, args
                        ),
                    }))
                    .map_err(|_| ())
                    .map(|_tx| ()),
                );
            };
            let y = if let Ok(x) = cursor.read_i32::<NativeEndian>() {
                x
            } else {
                return Box::new(
                    tx.send(Box::new(super::super::wayland::wl_display::events::Error {
                        sender_object_id: 1,
                        object_id: sender_object_id,
                        code: super::super::wayland::wl_display::enums::Error::InvalidMethod as u32,
                        message: format!(
                            "@{} opcode={} args={:?} not found",
                            sender_object_id, opcode, args
                        ),
                    }))
                    .map_err(|_| ())
                    .map(|_tx| ()),
                );
            };
            let width = if let Ok(x) = cursor.read_i32::<NativeEndian>() {
                x
            } else {
                return Box::new(
                    tx.send(Box::new(super::super::wayland::wl_display::events::Error {
                        sender_object_id: 1,
                        object_id: sender_object_id,
                        code: super::super::wayland::wl_display::enums::Error::InvalidMethod as u32,
                        message: format!(
                            "@{} opcode={} args={:?} not found",
                            sender_object_id, opcode, args
                        ),
                    }))
                    .map_err(|_| ())
                    .map(|_tx| ()),
                );
            };
            let height = if let Ok(x) = cursor.read_i32::<NativeEndian>() {
                x
            } else {
                return Box::new(
                    tx.send(Box::new(super::super::wayland::wl_display::events::Error {
                        sender_object_id: 1,
                        object_id: sender_object_id,
                        code: super::super::wayland::wl_display::enums::Error::InvalidMethod as u32,
                        message: format!(
                            "@{} opcode={} args={:?} not found",
                            sender_object_id, opcode, args
                        ),
                    }))
                    .map_err(|_| ())
                    .map(|_tx| ()),
                );
            };
            return XdgPositioner::set_anchor_rect(
                request,
                session,
                tx,
                sender_object_id,
                x,
                y,
                width,
                height,
            );
        }
        3 => {
            let anchor = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
                x
            } else {
                return Box::new(
                    tx.send(Box::new(super::super::wayland::wl_display::events::Error {
                        sender_object_id: 1,
                        object_id: sender_object_id,
                        code: super::super::wayland::wl_display::enums::Error::InvalidMethod as u32,
                        message: format!(
                            "@{} opcode={} args={:?} not found",
                            sender_object_id, opcode, args
                        ),
                    }))
                    .map_err(|_| ())
                    .map(|_tx| ()),
                );
            };
            return XdgPositioner::set_anchor(request, session, tx, sender_object_id, anchor);
        }
        4 => {
            let gravity = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
                x
            } else {
                return Box::new(
                    tx.send(Box::new(super::super::wayland::wl_display::events::Error {
                        sender_object_id: 1,
                        object_id: sender_object_id,
                        code: super::super::wayland::wl_display::enums::Error::InvalidMethod as u32,
                        message: format!(
                            "@{} opcode={} args={:?} not found",
                            sender_object_id, opcode, args
                        ),
                    }))
                    .map_err(|_| ())
                    .map(|_tx| ()),
                );
            };
            return XdgPositioner::set_gravity(request, session, tx, sender_object_id, gravity);
        }
        5 => {
            let constraint_adjustment = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
                x
            } else {
                return Box::new(
                    tx.send(Box::new(super::super::wayland::wl_display::events::Error {
                        sender_object_id: 1,
                        object_id: sender_object_id,
                        code: super::super::wayland::wl_display::enums::Error::InvalidMethod as u32,
                        message: format!(
                            "@{} opcode={} args={:?} not found",
                            sender_object_id, opcode, args
                        ),
                    }))
                    .map_err(|_| ())
                    .map(|_tx| ()),
                );
            };
            return XdgPositioner::set_constraint_adjustment(
                request,
                session,
                tx,
                sender_object_id,
                constraint_adjustment,
            );
        }
        6 => {
            let x = if let Ok(x) = cursor.read_i32::<NativeEndian>() {
                x
            } else {
                return Box::new(
                    tx.send(Box::new(super::super::wayland::wl_display::events::Error {
                        sender_object_id: 1,
                        object_id: sender_object_id,
                        code: super::super::wayland::wl_display::enums::Error::InvalidMethod as u32,
                        message: format!(
                            "@{} opcode={} args={:?} not found",
                            sender_object_id, opcode, args
                        ),
                    }))
                    .map_err(|_| ())
                    .map(|_tx| ()),
                );
            };
            let y = if let Ok(x) = cursor.read_i32::<NativeEndian>() {
                x
            } else {
                return Box::new(
                    tx.send(Box::new(super::super::wayland::wl_display::events::Error {
                        sender_object_id: 1,
                        object_id: sender_object_id,
                        code: super::super::wayland::wl_display::enums::Error::InvalidMethod as u32,
                        message: format!(
                            "@{} opcode={} args={:?} not found",
                            sender_object_id, opcode, args
                        ),
                    }))
                    .map_err(|_| ())
                    .map(|_tx| ()),
                );
            };
            return XdgPositioner::set_offset(request, session, tx, sender_object_id, x, y);
        }
        _ => {}
    };
    Box::new(futures::future::ok(()))
}

// child surface positioner
//
// The xdg_positioner provides a collection of rules for the placement of a
// child surface relative to a parent surface. Rules can be defined to ensure
// the child surface remains within the visible area's borders, and to
// specify how the child surface changes its position, such as sliding along
// an axis, or flipping around a rectangle. These positioner-created rules are
// constrained by the requirement that a child surface must intersect with or
// be at least partially adjacent to its parent surface.
//
// See the various requests for details about possible rules.
//
// At the time of the request, the compositor makes a copy of the rules
// specified by the xdg_positioner. Thus, after the request is complete the
// xdg_positioner object can be destroyed or reused; further changes to the
// object will have no effect on previous usages.
//
// For an xdg_positioner object to be considered complete, it must have a
// non-zero size set by set_size, and a non-zero anchor rectangle set by
// set_anchor_rect. Passing an incomplete xdg_positioner object when
// positioning a surface raises an error.
pub struct XdgPositioner {}

impl XdgPositioner {
    // destroy the xdg_positioner object
    //
    // Notify the compositor that the xdg_positioner will no longer be used.
    pub fn destroy(
        request: Arc<RwLock<XdgPositioner>>,
        session: RwLock<super::super::session::Session>,
        tx: tokio::sync::mpsc::Sender<Box<super::super::event::Event + Send>>,
        sender_object_id: u32,
    ) -> Box<futures::future::Future<Item = (), Error = ()> + Send> {
        Box::new(futures::future::ok(()))
    }

    // set anchor rectangle anchor
    //
    // Defines the anchor point for the anchor rectangle. The specified anchor
    // is used derive an anchor point that the child surface will be
    // positioned relative to. If a corner anchor is set (e.g. 'top_left' or
    // 'bottom_right'), the anchor point will be at the specified corner;
    // otherwise, the derived anchor point will be centered on the specified
    // edge, or in the center of the anchor rectangle if no edge is specified.
    pub fn set_anchor(
        request: Arc<RwLock<XdgPositioner>>,
        session: RwLock<super::super::session::Session>,
        tx: tokio::sync::mpsc::Sender<Box<super::super::event::Event + Send>>,
        sender_object_id: u32,
        anchor: u32, // uint: anchor
    ) -> Box<futures::future::Future<Item = (), Error = ()> + Send> {
        Box::new(futures::future::ok(()))
    }

    // set the anchor rectangle within the parent surface
    //
    // Specify the anchor rectangle within the parent surface that the child
    // surface will be placed relative to. The rectangle is relative to the
    // window geometry as defined by xdg_surface.set_window_geometry of the
    // parent surface.
    //
    // When the xdg_positioner object is used to position a child surface, the
    // anchor rectangle may not extend outside the window geometry of the
    // positioned child's parent surface.
    //
    // If a negative size is set the invalid_input error is raised.
    pub fn set_anchor_rect(
        request: Arc<RwLock<XdgPositioner>>,
        session: RwLock<super::super::session::Session>,
        tx: tokio::sync::mpsc::Sender<Box<super::super::event::Event + Send>>,
        sender_object_id: u32,
        x: i32,      // int: x position of anchor rectangle
        y: i32,      // int: y position of anchor rectangle
        width: i32,  // int: width of anchor rectangle
        height: i32, // int: height of anchor rectangle
    ) -> Box<futures::future::Future<Item = (), Error = ()> + Send> {
        Box::new(futures::future::ok(()))
    }

    // set the adjustment to be done when constrained
    //
    // Specify how the window should be positioned if the originally intended
    // position caused the surface to be constrained, meaning at least
    // partially outside positioning boundaries set by the compositor. The
    // adjustment is set by constructing a bitmask describing the adjustment to
    // be made when the surface is constrained on that axis.
    //
    // If no bit for one axis is set, the compositor will assume that the child
    // surface should not change its position on that axis when constrained.
    //
    // If more than one bit for one axis is set, the order of how adjustments
    // are applied is specified in the corresponding adjustment descriptions.
    //
    // The default adjustment is none.
    pub fn set_constraint_adjustment(
        request: Arc<RwLock<XdgPositioner>>,
        session: RwLock<super::super::session::Session>,
        tx: tokio::sync::mpsc::Sender<Box<super::super::event::Event + Send>>,
        sender_object_id: u32,
        constraint_adjustment: u32, // uint: bit mask of constraint adjustments
    ) -> Box<futures::future::Future<Item = (), Error = ()> + Send> {
        Box::new(futures::future::ok(()))
    }

    // set child surface gravity
    //
    // Defines in what direction a surface should be positioned, relative to
    // the anchor point of the parent surface. If a corner gravity is
    // specified (e.g. 'bottom_right' or 'top_left'), then the child surface
    // will be placed towards the specified gravity; otherwise, the child
    // surface will be centered over the anchor point on any axis that had no
    // gravity specified.
    pub fn set_gravity(
        request: Arc<RwLock<XdgPositioner>>,
        session: RwLock<super::super::session::Session>,
        tx: tokio::sync::mpsc::Sender<Box<super::super::event::Event + Send>>,
        sender_object_id: u32,
        gravity: u32, // uint: gravity direction
    ) -> Box<futures::future::Future<Item = (), Error = ()> + Send> {
        Box::new(futures::future::ok(()))
    }

    // set surface position offset
    //
    // Specify the surface position offset relative to the position of the
    // anchor on the anchor rectangle and the anchor on the surface. For
    // example if the anchor of the anchor rectangle is at (x, y), the surface
    // has the gravity bottom|right, and the offset is (ox, oy), the calculated
    // surface position will be (x + ox, y + oy). The offset position of the
    // surface is the one used for constraint testing. See
    // set_constraint_adjustment.
    //
    // An example use case is placing a popup menu on top of a user interface
    // element, while aligning the user interface element of the parent surface
    // with some user interface element placed somewhere in the popup surface.
    pub fn set_offset(
        request: Arc<RwLock<XdgPositioner>>,
        session: RwLock<super::super::session::Session>,
        tx: tokio::sync::mpsc::Sender<Box<super::super::event::Event + Send>>,
        sender_object_id: u32,
        x: i32, // int: surface position x offset
        y: i32, // int: surface position y offset
    ) -> Box<futures::future::Future<Item = (), Error = ()> + Send> {
        Box::new(futures::future::ok(()))
    }

    // set the size of the to-be positioned rectangle
    //
    // Set the size of the surface that is to be positioned with the positioner
    // object. The size is in surface-local coordinates and corresponds to the
    // window geometry. See xdg_surface.set_window_geometry.
    //
    // If a zero or negative size is set the invalid_input error is raised.
    pub fn set_size(
        request: Arc<RwLock<XdgPositioner>>,
        session: RwLock<super::super::session::Session>,
        tx: tokio::sync::mpsc::Sender<Box<super::super::event::Event + Send>>,
        sender_object_id: u32,
        width: i32,  // int: width of positioned rectangle
        height: i32, // int: height of positioned rectangle
    ) -> Box<futures::future::Future<Item = (), Error = ()> + Send> {
        Box::new(futures::future::ok(()))
    }
}
