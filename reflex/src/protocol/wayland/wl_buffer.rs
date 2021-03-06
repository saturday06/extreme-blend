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
use crate::protocol::session::{Context, NextAction, Session};
#[allow(unused_imports)]
use futures::future::{err, ok, Future};
#[allow(unused_imports)]
use futures::sink::Sink;
#[allow(unused_imports)]
use std::sync::{Arc, RwLock};

pub mod events;
mod lib;
pub use lib::*;

// content for a wl_surface
//
// A buffer provides the content for a wl_surface. Buffers are
// created through factory interfaces such as wl_drm, wl_shm or
// similar. It has a width and a height and can be attached to a
// wl_surface, but the mechanism by which a client provides and
// updates the contents is defined by the buffer factory interface.
pub struct WlBuffer {
    pub offset: i32, // int: buffer byte offset within the pool
    pub width: i32,  // int: buffer width, in pixels
    pub height: i32, // int: buffer height, in pixels
    pub stride: i32, // int: number of bytes from the beginning of one row to the beginning of the next row
    pub format: u32, // uint: buffer pixel format
}

impl WlBuffer {
    // destroy a buffer
    //
    // Destroy a buffer. If and how you need to release the backing
    // storage is defined by the buffer factory interface.
    //
    // For possible side-effects to a surface, see wl_surface.attach.
    pub fn destroy(
        context: Context<WlBuffer>,
    ) -> Box<dyn Future<Item = (Session, NextAction), Error = ()> + Send> {
        context.invalid_method("wl_buffer::destroy is not implemented yet".to_string())
    }
}
