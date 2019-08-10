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
use crate::protocol::session::{Context, Session};
#[allow(unused_imports)]
use futures::future::{err, ok, Future};
#[allow(unused_imports)]
use futures::sink::Sink;
#[allow(unused_imports)]
use std::sync::{Arc, RwLock};

mod lib;
pub use lib::*;

// region interface
//
// A region object describes an area.
//
// Region objects are used to describe the opaque and input
// regions of a surface.
pub struct WlRegion {}

impl WlRegion {
    // add rectangle to region
    //
    // Add the specified rectangle to the region.
    pub fn add(
        context: Context<WlRegion>,
        _x: i32,      // int: region-local x coordinate
        _y: i32,      // int: region-local y coordinate
        _width: i32,  // int: rectangle width
        _height: i32, // int: rectangle height
    ) -> Box<Future<Item = Session, Error = ()> + Send> {
        context.invalid_method("wl_region::add is not implemented yet".to_string())
    }

    // destroy region
    //
    // Destroy the region.  This will invalidate the object ID.
    pub fn destroy(context: Context<WlRegion>) -> Box<Future<Item = Session, Error = ()> + Send> {
        context.invalid_method("wl_region::destroy is not implemented yet".to_string())
    }

    // subtract rectangle from region
    //
    // Subtract the specified rectangle from the region.
    pub fn subtract(
        context: Context<WlRegion>,
        _x: i32,      // int: region-local x coordinate
        _y: i32,      // int: region-local y coordinate
        _width: i32,  // int: rectangle width
        _height: i32, // int: rectangle height
    ) -> Box<Future<Item = Session, Error = ()> + Send> {
        context.invalid_method("wl_region::subtract is not implemented yet".to_string())
    }
}
