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

use crate::protocol::session::{Context, Session};
use futures::future::{err, ok, Future};
use futures::sink::Sink;
use std::sync::{Arc, RwLock};

pub mod enums;
pub mod events;
mod lib;
pub use lib::*;

// desktop user interface surface base interface
//
//    An interface that may be implemented by a wl_surface, for
//    implementations that provide a desktop-style user interface.
//
//    It provides a base set of functionality required to construct user
//    interface elements requiring management by the compositor, such as
//    toplevel windows, menus, etc. The types of functionality are split into
//    xdg_surface roles.
//
//    Creating an xdg_surface does not set the role for a wl_surface. In order
//    to map an xdg_surface, the client must create a role-specific object
//    using, e.g., get_toplevel, get_popup. The wl_surface for any given
//    xdg_surface can have at most one role, and may not be assigned any role
//    not based on xdg_surface.
//
//    A role must be assigned before any other requests are made to the
//    xdg_surface object.
//
//    The client must call wl_surface.commit on the corresponding wl_surface
//    for the xdg_surface state to take effect.
//
//    Creating an xdg_surface from a wl_surface which has a buffer attached or
//    committed is a client error, and any attempts by a client to attach or
//    manipulate a buffer prior to the first xdg_surface.configure call must
//    also be treated as errors.
//
//    Mapping an xdg_surface-based role surface is defined as making it
//    possible for the surface to be shown by the compositor. Note that
//    a mapped surface is not guaranteed to be visible once it is mapped.
//
//    For an xdg_surface to be mapped by the compositor, the following
//    conditions must be met:
//    (1) the client has assigned an xdg_surface-based role to the surface
//    (2) the client has set and committed the xdg_surface state and the
// role-dependent state to the surface
//    (3) the client has committed a buffer to the surface
//
//    A newly-unmapped surface is considered to have met condition (1) out
//    of the 3 required conditions for mapping a surface if its role surface
//    has not been destroyed.
pub struct XdgSurface {
    pub wl_surface_id: u32,
}

impl XdgSurface {
    // ack a configure event
    //
    // When a configure event is received, if a client commits the
    // surface in response to the configure event, then the client
    // must make an ack_configure request sometime before the commit
    // request, passing along the serial of the configure event.
    //
    // For instance, for toplevel surfaces the compositor might use this
    // information to move a surface to the top left only when the client has
    // drawn itself for the maximized or fullscreen state.
    //
    // If the client receives multiple configure events before it
    // can respond to one, it only has to ack the last configure event.
    //
    // A client is not required to commit immediately after sending
    // an ack_configure request - it may even ack_configure several times
    // before its next surface commit.
    //
    // A client may send multiple ack_configure requests before committing, but
    // only the last request sent before a commit indicates which configure
    // event the client really is responding to.
    pub fn ack_configure(
        context: Context<XdgSurface>,
        _serial: u32, // uint: the serial from the configure event
    ) -> Box<Future<Item = Session, Error = ()> + Send> {
        context.invalid_method("xdg_surface::ack_configure is not implemented yet".to_string())
    }

    // destroy the xdg_surface
    //
    // Destroy the xdg_surface object. An xdg_surface must only be destroyed
    // after its role object has been destroyed.
    pub fn destroy(context: Context<XdgSurface>) -> Box<Future<Item = Session, Error = ()> + Send> {
        context.invalid_method("xdg_surface::destroy is not implemented yet".to_string())
    }

    // assign the xdg_popup surface role
    //
    // This creates an xdg_popup object for the given xdg_surface and gives
    // the associated wl_surface the xdg_popup role.
    //
    // If null is passed as a parent, a parent surface must be specified using
    // some other protocol, before committing the initial state.
    //
    // See the documentation of xdg_popup for more details about what an
    // xdg_popup is and how it is used.
    pub fn get_popup(
        context: Context<XdgSurface>,
        _id: u32,         // new_id:
        _parent: u32,     // object:
        _positioner: u32, // object:
    ) -> Box<Future<Item = Session, Error = ()> + Send> {
        context.invalid_method("xdg_surface::get_popup is not implemented yet".to_string())
    }

    // assign the xdg_toplevel surface role
    //
    // This creates an xdg_toplevel object for the given xdg_surface and gives
    // the associated wl_surface the xdg_toplevel role.
    //
    // See the documentation of xdg_toplevel for more details about what an
    // xdg_toplevel is and how it is used.
    pub fn get_toplevel(
        mut context: Context<XdgSurface>,
        id: u32, // new_id:
    ) -> Box<Future<Item = Session, Error = ()> + Send> {
        println!("XdgSurface::get_toplevel(id={})", id);
        context.resources.insert(
            id,
            crate::protocol::xdg_shell::xdg_toplevel::XdgToplevel {
                xdg_surface_id: context.sender_object_id,
            }
            .into(),
        );
        Box::new(ok(context.into()))
    }

    // set the new window geometry
    //
    // The window geometry of a surface is its "visible bounds" from the
    // user's perspective. Client-side decorations often have invisible
    // portions like drop-shadows which should be ignored for the
    // purposes of aligning, placing and constraining windows.
    //
    // The window geometry is double buffered, and will be applied at the
    // time wl_surface.commit of the corresponding wl_surface is called.
    //
    // When maintaining a position, the compositor should treat the (x, y)
    // coordinate of the window geometry as the top left corner of the window.
    // A client changing the (x, y) window geometry coordinate should in
    // general not alter the position of the window.
    //
    // Once the window geometry of the surface is set, it is not possible to
    // unset it, and it will remain the same until set_window_geometry is
    // called again, even if a new subsurface or buffer is attached.
    //
    // If never set, the value is the full bounds of the surface,
    // including any subsurfaces. This updates dynamically on every
    // commit. This unset is meant for extremely simple clients.
    //
    // The arguments are given in the surface-local coordinate space of
    // the wl_surface associated with this xdg_surface.
    //
    // The width and height must be greater than zero. Setting an invalid size
    // will raise an error. When applied, the effective window geometry will be
    // the set window geometry clamped to the bounding rectangle of the
    // combined geometry of the surface of the xdg_surface and the associated
    // subsurfaces.
    pub fn set_window_geometry(
        context: Context<XdgSurface>,
        _x: i32,      // int:
        _y: i32,      // int:
        _width: i32,  // int:
        _height: i32, // int:
    ) -> Box<Future<Item = Session, Error = ()> + Send> {
        context.invalid_method("xdg_surface::set_window_geometry is not implemented yet".to_string())
    }
}
