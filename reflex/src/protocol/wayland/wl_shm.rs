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

// shared memory support
//
// A singleton global object that provides support for shared
// memory.
//
// Clients can create wl_shm_pool objects using the create_pool
// request.
//
// At connection setup time, the wl_shm object emits one or more
// format events to inform clients about the valid pixel formats
// that can be used for buffers.
pub struct WlShm {}

impl WlShm {
    // create a shm pool
    //
    // Create a new wl_shm_pool object.
    //
    // The pool can be used to create shared memory based buffer
    // objects.  The server will mmap size bytes of the passed file
    // descriptor, to use as backing memory for the pool.
    pub fn create_pool(
        context: Context<WlShm>,
        id: u32,   // new_id: pool to create
        fd: i32,   // fd: file descriptor for the pool
        size: i32, // int: pool size, in bytes
    ) -> Box<Future<Item = Session, Error = ()> + Send> {
        Box::new(err(()))
    }
}
