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

mod lib;
pub use lib::*;

// a shared memory pool
//
// The wl_shm_pool object encapsulates a piece of memory shared
// between the compositor and client.  Through the wl_shm_pool
// object, the client can allocate shared memory wl_buffer objects.
// All objects created through the same pool share the same
// underlying mapped memory. Reusing the mapped memory avoids the
// setup/teardown overhead and is useful when interactively resizing
// a surface or for many small buffers.
pub struct WlShmPool {}

impl WlShmPool {
    // create a buffer from the pool
    //
    // Create a wl_buffer object from the pool.
    //
    // The buffer is created offset bytes into the pool and has
    // width and height as specified.  The stride argument specifies
    // the number of bytes from the beginning of one row to the beginning
    // of the next.  The format is the pixel format of the buffer and
    // must be one of those advertised through the wl_shm.format event.
    //
    // A buffer will keep a reference to the pool it was created from
    // so it is valid to destroy the pool immediately after creating
    // a buffer from it.
    pub fn create_buffer(
        context: Context<WlShmPool>,
        _id: u32,     // new_id: buffer to create
        _offset: i32, // int: buffer byte offset within the pool
        _width: i32,  // int: buffer width, in pixels
        _height: i32, // int: buffer height, in pixels
        _stride: i32, // int: number of bytes from the beginning of one row to the beginning of the next row
        _format: u32, // uint: buffer pixel format
    ) -> Box<Future<Item = Session, Error = ()> + Send> {
        context.invalid_method("wl_shm_pool::create_buffer is not implemented yet".to_string())
    }

    // destroy the pool
    //
    // Destroy the shared memory pool.
    //
    // The mmapped memory will be released when all
    // buffers that have been created from this pool
    // are gone.
    pub fn destroy(context: Context<WlShmPool>) -> Box<Future<Item = Session, Error = ()> + Send> {
        context.invalid_method("wl_shm_pool::destroy is not implemented yet".to_string())
    }

    // change the size of the pool mapping
    //
    // This request will cause the server to remap the backing memory
    // for the pool from the file descriptor passed when the pool was
    // created, but using the new size.  This request can only be
    // used to make the pool bigger.
    pub fn resize(
        context: Context<WlShmPool>,
        _size: i32, // int: new size of the pool, in bytes
    ) -> Box<Future<Item = Session, Error = ()> + Send> {
        context.invalid_method("wl_shm_pool::resize is not implemented yet".to_string())
    }
}
