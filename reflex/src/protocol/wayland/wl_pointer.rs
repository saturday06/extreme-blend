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

pub mod enums;
pub mod events;
mod lib;
pub use lib::*;

// pointer input device
//
// The wl_pointer interface represents one or more input devices,
// such as mice, which control the pointer location and pointer_focus
// of a seat.
//
// The wl_pointer interface generates motion, enter and leave
// events for the surfaces that the pointer is located over,
// and button and axis events for button presses, button releases
// and scrolling.
pub struct WlPointer {}

impl WlPointer {
    // release the pointer object
    //
    // Using this request a client can tell the server that it is not going to
    // use the pointer object anymore.
    //
    // This request destroys the pointer proxy object, so clients must not call
    // wl_pointer_destroy() after using this request.
    pub fn release(context: Context<WlPointer>) -> Box<Future<Item = Session, Error = ()> + Send> {
        Box::new(err(()))
    }

    // set the pointer surface
    //
    // Set the pointer surface, i.e., the surface that contains the
    // pointer image (cursor). This request gives the surface the role
    // of a cursor. If the surface already has another role, it raises
    // a protocol error.
    //
    // The cursor actually changes only if the pointer
    // focus for this device is one of the requesting client's surfaces
    // or the surface parameter is the current pointer surface. If
    // there was a previous surface set with this request it is
    // replaced. If surface is NULL, the pointer image is hidden.
    //
    // The parameters hotspot_x and hotspot_y define the position of
    // the pointer surface relative to the pointer location. Its
    // top-left corner is always at (x, y) - (hotspot_x, hotspot_y),
    // where (x, y) are the coordinates of the pointer location, in
    // surface-local coordinates.
    //
    // On surface.attach requests to the pointer surface, hotspot_x
    // and hotspot_y are decremented by the x and y parameters
    // passed to the request. Attach must be confirmed by
    // wl_surface.commit as usual.
    //
    // The hotspot can also be updated by passing the currently set
    // pointer surface to this request with new values for hotspot_x
    // and hotspot_y.
    //
    // The current and pending input regions of the wl_surface are
    // cleared, and wl_surface.set_input_region is ignored until the
    // wl_surface is no longer used as the cursor. When the use as a
    // cursor ends, the current and pending input regions become
    // undefined, and the wl_surface is unmapped.
    pub fn set_cursor(
        context: Context<WlPointer>,
        serial: u32,    // uint: serial number of the enter event
        surface: u32,   // object: pointer surface
        hotspot_x: i32, // int: surface-local x coordinate
        hotspot_y: i32, // int: surface-local y coordinate
    ) -> Box<Future<Item = Session, Error = ()> + Send> {
        Box::new(err(()))
    }
}
