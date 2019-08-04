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

pub mod enums;
pub mod events;
mod lib;
pub use lib::*;

// data transfer device
//
// There is one wl_data_device per seat which can be obtained
// from the global wl_data_device_manager singleton.
//
// A wl_data_device provides access to inter-client data transfer
// mechanisms such as copy-and-paste and drag-and-drop.
pub struct WlDataDevice {}

impl WlDataDevice {
    // destroy data device
    //
    // This request destroys the data device.
    pub fn release(
        context: Context<WlDataDevice>,
    ) -> Box<Future<Item = Session, Error = ()> + Send> {
        let tx = context.tx.clone();
        return Box::new(
            tx.send(Box::new(
                crate::protocol::wayland::wl_display::events::Error {
                    sender_object_id: 1,
                    object_id: context.sender_object_id,
                    code: crate::protocol::wayland::wl_display::enums::Error::InvalidMethod as u32,
                    message: format!(
                        "wl_data_device@{}::release is not implemented yet",
                        context.sender_object_id
                    ),
                },
            ))
            .map_err(|_| ())
            .map(|_| context.into()),
        );
    }

    // copy data to the selection
    //
    // This request asks the compositor to set the selection
    // to the data from the source on behalf of the client.
    //
    // To unset the selection, set the source to NULL.
    pub fn set_selection(
        context: Context<WlDataDevice>,
        _source: u32, // object: data source for the selection
        _serial: u32, // uint: serial number of the event that triggered this request
    ) -> Box<Future<Item = Session, Error = ()> + Send> {
        let tx = context.tx.clone();
        return Box::new(
            tx.send(Box::new(
                crate::protocol::wayland::wl_display::events::Error {
                    sender_object_id: 1,
                    object_id: context.sender_object_id,
                    code: crate::protocol::wayland::wl_display::enums::Error::InvalidMethod as u32,
                    message: format!(
                        "wl_data_device@{}::set_selection is not implemented yet",
                        context.sender_object_id
                    ),
                },
            ))
            .map_err(|_| ())
            .map(|_| context.into()),
        );
    }

    // start drag-and-drop operation
    //
    // This request asks the compositor to start a drag-and-drop
    // operation on behalf of the client.
    //
    // The source argument is the data source that provides the data
    // for the eventual data transfer. If source is NULL, enter, leave
    // and motion events are sent only to the client that initiated the
    // drag and the client is expected to handle the data passing
    // internally.
    //
    // The origin surface is the surface where the drag originates and
    // the client must have an active implicit grab that matches the
    // serial.
    //
    // The icon surface is an optional (can be NULL) surface that
    // provides an icon to be moved around with the cursor.  Initially,
    // the top-left corner of the icon surface is placed at the cursor
    // hotspot, but subsequent wl_surface.attach request can move the
    // relative position. Attach requests must be confirmed with
    // wl_surface.commit as usual. The icon surface is given the role of
    // a drag-and-drop icon. If the icon surface already has another role,
    // it raises a protocol error.
    //
    // The current and pending input regions of the icon wl_surface are
    // cleared, and wl_surface.set_input_region is ignored until the
    // wl_surface is no longer used as the icon surface. When the use
    // as an icon ends, the current and pending input regions become
    // undefined, and the wl_surface is unmapped.
    pub fn start_drag(
        context: Context<WlDataDevice>,
        _source: u32, // object: data source for the eventual transfer
        _origin: u32, // object: surface where the drag originates
        _icon: u32,   // object: drag-and-drop icon surface
        _serial: u32, // uint: serial number of the implicit grab on the origin
    ) -> Box<Future<Item = Session, Error = ()> + Send> {
        let tx = context.tx.clone();
        return Box::new(
            tx.send(Box::new(
                crate::protocol::wayland::wl_display::events::Error {
                    sender_object_id: 1,
                    object_id: context.sender_object_id,
                    code: crate::protocol::wayland::wl_display::enums::Error::InvalidMethod as u32,
                    message: format!(
                        "wl_data_device@{}::start_drag is not implemented yet",
                        context.sender_object_id
                    ),
                },
            ))
            .map_err(|_| ())
            .map(|_| context.into()),
        );
    }
}
