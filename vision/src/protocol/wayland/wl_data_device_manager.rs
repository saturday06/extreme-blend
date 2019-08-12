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

pub mod enums;
mod lib;
pub use lib::*;

// data transfer interface
//
// The wl_data_device_manager is a singleton global object that
// provides access to inter-client data transfer mechanisms such as
// copy-and-paste and drag-and-drop.  These mechanisms are tied to
// a wl_seat and this interface lets a client get a wl_data_device
// corresponding to a wl_seat.
//
// Depending on the version bound, the objects created from the bound
// wl_data_device_manager object will have different requirements for
// functioning properly. See wl_data_source.set_actions,
// wl_data_offer.accept and wl_data_offer.finish for details.
pub struct WlDataDeviceManager {}

impl WlDataDeviceManager {
    // create a new data source
    //
    // Create a new data source.
    pub fn create_data_source(
        context: Context<Arc<RwLock<WlDataDeviceManager>>>,
        _id: u32, // new_id: data source to create
    ) -> Box<Future<Item = (Session, NextAction), Error = ()> + Send> {
        context.invalid_method(
            "wl_data_device_manager::create_data_source is not implemented yet".to_string(),
        )
    }

    // create a new data device
    //
    // Create a new data device for a given seat.
    pub fn get_data_device(
        context: Context<Arc<RwLock<WlDataDeviceManager>>>,
        _id: u32,   // new_id: data device to create
        _seat: u32, // object: seat associated with the data device
    ) -> Box<Future<Item = (Session, NextAction), Error = ()> + Send> {
        context.invalid_method(
            "wl_data_device_manager::get_data_device is not implemented yet".to_string(),
        )
    }
}
