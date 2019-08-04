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

use crate::protocol::session::{Context, Session};
use futures::future::{err, ok, Future};
use futures::sink::Sink;
use std::sync::{Arc, RwLock};

pub mod enums;
pub mod events;
mod lib;
pub use lib::*;

// group of input devices
//
// A seat is a group of keyboards, pointer and touch devices. This
// object is published as a global during start up, or when such a
// device is hot plugged.  A seat typically has a pointer and
// maintains a keyboard focus and a pointer focus.
pub struct WlSeat {}

impl WlSeat {
    // return keyboard object
    //
    // The ID provided will be initialized to the wl_keyboard interface
    // for this seat.
    //
    // This request only takes effect if the seat has the keyboard
    // capability, or has had the keyboard capability in the past.
    // It is a protocol violation to issue this request on a seat that has
    // never had the keyboard capability.
    pub fn get_keyboard(
        context: Context<WlSeat>,
        _id: u32, // new_id: seat keyboard
    ) -> Box<Future<Item = Session, Error = ()> + Send> {
        context.invalid_method(format!(
            "wl_seat@{}::get_keyboard is not implemented yet",
            context.sender_object_id
        ))
    }

    // return pointer object
    //
    // The ID provided will be initialized to the wl_pointer interface
    // for this seat.
    //
    // This request only takes effect if the seat has the pointer
    // capability, or has had the pointer capability in the past.
    // It is a protocol violation to issue this request on a seat that has
    // never had the pointer capability.
    pub fn get_pointer(
        context: Context<WlSeat>,
        _id: u32, // new_id: seat pointer
    ) -> Box<Future<Item = Session, Error = ()> + Send> {
        context.invalid_method(format!(
            "wl_seat@{}::get_pointer is not implemented yet",
            context.sender_object_id
        ))
    }

    // return touch object
    //
    // The ID provided will be initialized to the wl_touch interface
    // for this seat.
    //
    // This request only takes effect if the seat has the touch
    // capability, or has had the touch capability in the past.
    // It is a protocol violation to issue this request on a seat that has
    // never had the touch capability.
    pub fn get_touch(
        context: Context<WlSeat>,
        _id: u32, // new_id: seat touch interface
    ) -> Box<Future<Item = Session, Error = ()> + Send> {
        context.invalid_method(format!(
            "wl_seat@{}::get_touch is not implemented yet",
            context.sender_object_id
        ))
    }

    // release the seat object
    //
    // Using this request a client can tell the server that it is not going to
    // use the seat object anymore.
    pub fn release(context: Context<WlSeat>) -> Box<Future<Item = Session, Error = ()> + Send> {
        context.invalid_method(format!(
            "wl_seat@{}::release is not implemented yet",
            context.sender_object_id
        ))
    }
}
