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

// desktop-style metadata interface
//
// An interface that may be implemented by a wl_surface, for
// implementations that provide a desktop-style user interface.
//
// It provides requests to treat surfaces like toplevel, fullscreen
// or popup windows, move, resize or maximize them, associate
// metadata like title and class, etc.
//
// On the server side the object is automatically destroyed when
// the related wl_surface is destroyed. On the client side,
// wl_shell_surface_destroy() must be called before destroying
// the wl_surface object.
pub struct WlShellSurface {}

impl WlShellSurface {
    // start an interactive move
    //
    // Start a pointer-driven move of the surface.
    //
    // This request must be used in response to a button press event.
    // The server may ignore move requests depending on the state of
    // the surface (e.g. fullscreen or maximized).
    pub fn move_fn(
        context: Context<WlShellSurface>,
        seat: u32,   // object: seat whose pointer is used
        serial: u32, // uint: serial number of the implicit grab on the pointer
    ) -> Box<Future<Item = Session, Error = ()> + Send> {
        Box::new(err(()))
    }

    // respond to a ping event
    //
    // A client must respond to a ping event with a pong request or
    // the client may be deemed unresponsive.
    pub fn pong(
        context: Context<WlShellSurface>,
        serial: u32, // uint: serial number of the ping event
    ) -> Box<Future<Item = Session, Error = ()> + Send> {
        Box::new(err(()))
    }

    // start an interactive resize
    //
    // Start a pointer-driven resizing of the surface.
    //
    // This request must be used in response to a button press event.
    // The server may ignore resize requests depending on the state of
    // the surface (e.g. fullscreen or maximized).
    pub fn resize(
        context: Context<WlShellSurface>,
        seat: u32,   // object: seat whose pointer is used
        serial: u32, // uint: serial number of the implicit grab on the pointer
        edges: u32,  // uint: which edge or corner is being dragged
    ) -> Box<Future<Item = Session, Error = ()> + Send> {
        Box::new(err(()))
    }

    // set surface class
    //
    // Set a class for the surface.
    //
    // The surface class identifies the general class of applications
    // to which the surface belongs. A common convention is to use the
    // file name (or the full path if it is a non-standard location) of
    // the application's .desktop file as the class.
    pub fn set_class(
        context: Context<WlShellSurface>,
        class_: String, // string: surface class
    ) -> Box<Future<Item = Session, Error = ()> + Send> {
        Box::new(err(()))
    }

    // make the surface a fullscreen surface
    //
    // Map the surface as a fullscreen surface.
    //
    // If an output parameter is given then the surface will be made
    // fullscreen on that output. If the client does not specify the
    // output then the compositor will apply its policy - usually
    // choosing the output on which the surface has the biggest surface
    // area.
    //
    // The client may specify a method to resolve a size conflict
    // between the output size and the surface size - this is provided
    // through the method parameter.
    //
    // The framerate parameter is used only when the method is set
    // to "driver", to indicate the preferred framerate. A value of 0
    // indicates that the client does not care about framerate.  The
    // framerate is specified in mHz, that is framerate of 60000 is 60Hz.
    //
    // A method of "scale" or "driver" implies a scaling operation of
    // the surface, either via a direct scaling operation or a change of
    // the output mode. This will override any kind of output scaling, so
    // that mapping a surface with a buffer size equal to the mode can
    // fill the screen independent of buffer_scale.
    //
    // A method of "fill" means we don't scale up the buffer, however
    // any output scale is applied. This means that you may run into
    // an edge case where the application maps a buffer with the same
    // size of the output mode but buffer_scale 1 (thus making a
    // surface larger than the output). In this case it is allowed to
    // downscale the results to fit the screen.
    //
    // The compositor must reply to this request with a configure event
    // with the dimensions for the output on which the surface will
    // be made fullscreen.
    pub fn set_fullscreen(
        context: Context<WlShellSurface>,
        method: u32,    // uint: method for resolving size conflict
        framerate: u32, // uint: framerate in mHz
        output: u32,    // object: output on which the surface is to be fullscreen
    ) -> Box<Future<Item = Session, Error = ()> + Send> {
        Box::new(err(()))
    }

    // make the surface a maximized surface
    //
    // Map the surface as a maximized surface.
    //
    // If an output parameter is given then the surface will be
    // maximized on that output. If the client does not specify the
    // output then the compositor will apply its policy - usually
    // choosing the output on which the surface has the biggest surface
    // area.
    //
    // The compositor will reply with a configure event telling
    // the expected new surface size. The operation is completed
    // on the next buffer attach to this surface.
    //
    // A maximized surface typically fills the entire output it is
    // bound to, except for desktop elements such as panels. This is
    // the main difference between a maximized shell surface and a
    // fullscreen shell surface.
    //
    // The details depend on the compositor implementation.
    pub fn set_maximized(
        context: Context<WlShellSurface>,
        output: u32, // object: output on which the surface is to be maximized
    ) -> Box<Future<Item = Session, Error = ()> + Send> {
        Box::new(err(()))
    }

    // make the surface a popup surface
    //
    // Map the surface as a popup.
    //
    // A popup surface is a transient surface with an added pointer
    // grab.
    //
    // An existing implicit grab will be changed to owner-events mode,
    // and the popup grab will continue after the implicit grab ends
    // (i.e. releasing the mouse button does not cause the popup to
    // be unmapped).
    //
    // The popup grab continues until the window is destroyed or a
    // mouse button is pressed in any other client's window. A click
    // in any of the client's surfaces is reported as normal, however,
    // clicks in other clients' surfaces will be discarded and trigger
    // the callback.
    //
    // The x and y arguments specify the location of the upper left
    // corner of the surface relative to the upper left corner of the
    // parent surface, in surface-local coordinates.
    pub fn set_popup(
        context: Context<WlShellSurface>,
        seat: u32,   // object: seat whose pointer is used
        serial: u32, // uint: serial number of the implicit grab on the pointer
        parent: u32, // object: parent surface
        x: i32,      // int: surface-local x coordinate
        y: i32,      // int: surface-local y coordinate
        flags: u32,  // uint: transient surface behavior
    ) -> Box<Future<Item = Session, Error = ()> + Send> {
        Box::new(err(()))
    }

    // set surface title
    //
    // Set a short title for the surface.
    //
    // This string may be used to identify the surface in a task bar,
    // window list, or other user interface elements provided by the
    // compositor.
    //
    // The string must be encoded in UTF-8.
    pub fn set_title(
        context: Context<WlShellSurface>,
        title: String, // string: surface title
    ) -> Box<Future<Item = Session, Error = ()> + Send> {
        Box::new(err(()))
    }

    // make the surface a toplevel surface
    //
    // Map the surface as a toplevel surface.
    //
    // A toplevel surface is not fullscreen, maximized or transient.
    pub fn set_toplevel(
        context: Context<WlShellSurface>,
    ) -> Box<Future<Item = Session, Error = ()> + Send> {
        Box::new(err(()))
    }

    // make the surface a transient surface
    //
    // Map the surface relative to an existing surface.
    //
    // The x and y arguments specify the location of the upper left
    // corner of the surface relative to the upper left corner of the
    // parent surface, in surface-local coordinates.
    //
    // The flags argument controls details of the transient behaviour.
    pub fn set_transient(
        context: Context<WlShellSurface>,
        parent: u32, // object: parent surface
        x: i32,      // int: surface-local x coordinate
        y: i32,      // int: surface-local y coordinate
        flags: u32,  // uint: transient surface behavior
    ) -> Box<Future<Item = Session, Error = ()> + Send> {
        Box::new(err(()))
    }
}
