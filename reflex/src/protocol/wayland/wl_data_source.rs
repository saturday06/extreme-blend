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
use futures::future::{Future, ok};

pub mod enums;
pub mod events;
mod lib;
pub use lib::*;

// offer to transfer data
//
// The wl_data_source object is the source side of a wl_data_offer.
// It is created by the source client in a data transfer and
// provides a way to describe the offered data and a way to respond
// to requests to transfer the data.
pub struct WlDataSource {
}

impl WlDataSource {
    // destroy the data source
    //
    // Destroy the data source.
    pub fn destroy(
        context: Context<WlDataSource>,
    ) -> Box<Future<Item = Session, Error = ()> + Send> {
        Box::new(ok(context.into()))
    }

    // add an offered mime type
    //
    // This request adds a mime type to the set of mime types
    // advertised to targets.  Can be called several times to offer
    // multiple types.
    pub fn offer(
        context: Context<WlDataSource>,
        mime_type: String, // string: mime type offered by the data source
    ) -> Box<Future<Item = Session, Error = ()> + Send> {
        Box::new(ok(context.into()))
    }

    // set the available drag-and-drop actions
    //
    // Sets the actions that the source side client supports for this
    // operation. This request may trigger wl_data_source.action and
    // wl_data_offer.action events if the compositor needs to change the
    // selected action.
    // 
    // The dnd_actions argument must contain only values expressed in the
    // wl_data_device_manager.dnd_actions enum, otherwise it will result
    // in a protocol error.
    // 
    // This request must be made once only, and can only be made on sources
    // used in drag-and-drop, so it must be performed before
    // wl_data_device.start_drag. Attempting to use the source other than
    // for drag-and-drop will raise a protocol error.
    pub fn set_actions(
        context: Context<WlDataSource>,
        dnd_actions: u32, // uint: actions supported by the data source
    ) -> Box<Future<Item = Session, Error = ()> + Send> {
        Box::new(ok(context.into()))
    }
}
