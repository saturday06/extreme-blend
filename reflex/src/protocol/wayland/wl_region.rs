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
use futures::future::{Future, ok, err};
use std::sync::{Arc, RwLock};

mod lib;
pub use lib::*;

// region interface
//
// A region object describes an area.
// 
// Region objects are used to describe the opaque and input
// regions of a surface.
pub struct WlRegion {
}

impl WlRegion {
    // add rectangle to region
    //
    // Add the specified rectangle to the region.
    pub fn add(
        context: Context<WlRegion>,
        x: i32, // int: region-local x coordinate
        y: i32, // int: region-local y coordinate
        width: i32, // int: rectangle width
        height: i32, // int: rectangle height
    ) -> Box<Future<Item = Session, Error = ()> + Send> {
        Box::new(err(()))
    }

    // destroy region
    //
    // Destroy the region.  This will invalidate the object ID.
    pub fn destroy(
        context: Context<WlRegion>,
    ) -> Box<Future<Item = Session, Error = ()> + Send> {
        Box::new(err(()))
    }

    // subtract rectangle from region
    //
    // Subtract the specified rectangle from the region.
    pub fn subtract(
        context: Context<WlRegion>,
        x: i32, // int: region-local x coordinate
        y: i32, // int: region-local y coordinate
        width: i32, // int: rectangle width
        height: i32, // int: rectangle height
    ) -> Box<Future<Item = Session, Error = ()> + Send> {
        Box::new(err(()))
    }
}
