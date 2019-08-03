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

#[allow(unused_imports)] use byteorder::{NativeEndian, ReadBytesExt};
#[allow(unused_imports)] use futures::future::Future;
#[allow(unused_imports)] use futures::sink::Sink;
#[allow(unused_imports)] use std::io::{Cursor, Read};
#[allow(unused_imports)] use std::sync::{Arc, RwLock};

pub mod enums {
    // drag and drop actions
    //
    // This is a bitmask of the available/preferred actions in a
    // drag-and-drop operation.
    // 
    // In the compositor, the selected action is a result of matching the
    // actions offered by the source and destination sides.  "action" events
    // with a "none" action will be sent to both source and destination if
    // there is no match. All further checks will effectively happen on
    // (source actions ∩ destination actions).
    // 
    // In addition, compositors may also pick different actions in
    // reaction to key modifiers being pressed. One common design that
    // is used in major toolkits (and the behavior recommended for
    // compositors) is:
    // 
    // - If no modifiers are pressed, the first match (in bit order)
    //   will be used.
    // - Pressing Shift selects "move", if enabled in the mask.
    // - Pressing Control selects "copy", if enabled in the mask.
    // 
    // Behavior beyond that is considered implementation-dependent.
    // Compositors may for example bind other modifiers (like Alt/Meta)
    // or drags initiated with other buttons than BTN_LEFT to specific
    // actions (e.g. "ask").
    pub enum DndAction {
        None = 0, // no action
        Copy = 1, // copy action
        Move = 2, // move action
        Ask = 4, // ask action
    }
}

pub fn dispatch_request(request: Arc<RwLock<WlDataDeviceManager>>, session: RwLock<super::super::session::Session>, tx: tokio::sync::mpsc::Sender<Box<super::super::event::Event + Send>>, sender_object_id: u32, opcode: u16, args: Vec<u8>) -> Box<futures::future::Future<Item = (), Error = ()>> {
    let mut cursor = Cursor::new(&args);
    match opcode {
        0 => {
            let id = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
                x 
            } else {
                return Box::new(tx.send(Box::new(super::super::wayland::wl_display::events::Error {
                    sender_object_id: 1,
                    object_id: sender_object_id,
                    code: super::super::wayland::wl_display::enums::Error::InvalidMethod as u32,
                    message: format!(
                        "@{} opcode={} args={:?} not found",
                        sender_object_id, opcode, args
                    ),
                })).map_err(|_| ()).map(|_tx| ()));

            };
            return WlDataDeviceManager::create_data_source(request, session, tx, sender_object_id, id)
        },
        1 => {
            let id = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
                x 
            } else {
                return Box::new(tx.send(Box::new(super::super::wayland::wl_display::events::Error {
                    sender_object_id: 1,
                    object_id: sender_object_id,
                    code: super::super::wayland::wl_display::enums::Error::InvalidMethod as u32,
                    message: format!(
                        "@{} opcode={} args={:?} not found",
                        sender_object_id, opcode, args
                    ),
                })).map_err(|_| ()).map(|_tx| ()));

            };
            let seat = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
                x 
            } else {
                return Box::new(tx.send(Box::new(super::super::wayland::wl_display::events::Error {
                    sender_object_id: 1,
                    object_id: sender_object_id,
                    code: super::super::wayland::wl_display::enums::Error::InvalidMethod as u32,
                    message: format!(
                        "@{} opcode={} args={:?} not found",
                        sender_object_id, opcode, args
                    ),
                })).map_err(|_| ()).map(|_tx| ()));

            };
            return WlDataDeviceManager::get_data_device(request, session, tx, sender_object_id, id, seat)
        },
        _ => {},
    };
    Box::new(futures::future::ok(()))
}

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
pub struct WlDataDeviceManager {
}

impl WlDataDeviceManager {
    // create a new data source
    //
    // Create a new data source.
    pub fn create_data_source(
        request: Arc<RwLock<WlDataDeviceManager>>,
        session: RwLock<super::super::session::Session>,
        tx: tokio::sync::mpsc::Sender<Box<super::super::event::Event + Send>>,
        sender_object_id: u32,
        id: u32, // new_id: data source to create
    ) -> Box<futures::future::Future<Item = (), Error = ()>> {
        Box::new(futures::future::ok(()))
    }

    // create a new data device
    //
    // Create a new data device for a given seat.
    pub fn get_data_device(
        request: Arc<RwLock<WlDataDeviceManager>>,
        session: RwLock<super::super::session::Session>,
        tx: tokio::sync::mpsc::Sender<Box<super::super::event::Event + Send>>,
        sender_object_id: u32,
        id: u32, // new_id: data device to create
        seat: u32, // object: seat associated with the data device
    ) -> Box<futures::future::Future<Item = (), Error = ()>> {
        Box::new(futures::future::ok(()))
    }
}
