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
use std::sync::{Arc, RwLock};

mod lib;
pub use lib::*;

// the compositor singleton
//
// A compositor.  This object is a singleton global.  The
// compositor is in charge of combining the contents of multiple
// surfaces into one displayable output.
pub struct WlCompositor {}

impl WlCompositor {
    // create new region
    //
    // Ask the compositor to create a new region.
    pub fn create_region(
        context: Context<Arc<RwLock<WlCompositor>>>,
        id: u32, // new_id: the new region
    ) -> Box<Future<Item = Session, Error = ()> + Send> {
        Box::new(err(()))
    }

    // create new surface
    //
    // Ask the compositor to create a new surface.
    pub fn create_surface(
        mut context: Context<Arc<RwLock<WlCompositor>>>,
        id: u32, // new_id: the new surface
    ) -> Box<Future<Item = Session, Error = ()> + Send> {
        println!("WlCompositor::create_surface(id: {})", id);
        context.resources.insert(id, crate::protocol::wayland::wl_surface::WlSurface{}.into());
        Box::new(ok(context.into()))
    }
}
