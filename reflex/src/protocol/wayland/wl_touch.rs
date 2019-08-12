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

// touchscreen input device
//
// The wl_touch interface represents a touchscreen
// associated with a seat.
//
// Touch interactions can consist of one or more contacts.
// For each contact, a series of events is generated, starting
// with a down event, followed by zero or more motion events,
// and ending with an up event. Events relating to the same
// contact point can be identified by the ID of the sequence.
pub struct WlTouch {}

impl WlTouch {
    // release the touch object
    pub fn release(
        context: Context<WlTouch>,
    ) -> Box<Future<Item = (Session, NextAction), Error = ()> + Send> {
        context.invalid_method("wl_touch::release is not implemented yet".to_string())
    }
}
