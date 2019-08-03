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
use byteorder::{NativeEndian, ReadBytesExt};
#[allow(unused_imports)]
use futures::future::Future;
#[allow(unused_imports)]
use futures::sink::Sink;
#[allow(unused_imports)]
use std::io::{Cursor, Read};
#[allow(unused_imports)]
use std::sync::{Arc, RwLock};

pub fn dispatch_request(
    request: Arc<RwLock<WlRegion>>,
    session: RwLock<super::super::session::Session>,
    tx: tokio::sync::mpsc::Sender<Box<super::super::event::Event + Send>>,
    sender_object_id: u32,
    opcode: u16,
    args: Vec<u8>,
) -> Box<futures::future::Future<Item = (), Error = ()> + Send> {
    let mut cursor = Cursor::new(&args);
    match opcode {
        0 => return WlRegion::destroy(request, session, tx, sender_object_id),
        1 => {
            let x = if let Ok(x) = cursor.read_i32::<NativeEndian>() {
                x
            } else {
                return Box::new(
                    tx.send(Box::new(super::super::wayland::wl_display::events::Error {
                        sender_object_id: 1,
                        object_id: sender_object_id,
                        code: super::super::wayland::wl_display::enums::Error::InvalidMethod as u32,
                        message: format!(
                            "@{} opcode={} args={:?} not found",
                            sender_object_id, opcode, args
                        ),
                    }))
                    .map_err(|_| ())
                    .map(|_tx| ()),
                );
            };
            let y = if let Ok(x) = cursor.read_i32::<NativeEndian>() {
                x
            } else {
                return Box::new(
                    tx.send(Box::new(super::super::wayland::wl_display::events::Error {
                        sender_object_id: 1,
                        object_id: sender_object_id,
                        code: super::super::wayland::wl_display::enums::Error::InvalidMethod as u32,
                        message: format!(
                            "@{} opcode={} args={:?} not found",
                            sender_object_id, opcode, args
                        ),
                    }))
                    .map_err(|_| ())
                    .map(|_tx| ()),
                );
            };
            let width = if let Ok(x) = cursor.read_i32::<NativeEndian>() {
                x
            } else {
                return Box::new(
                    tx.send(Box::new(super::super::wayland::wl_display::events::Error {
                        sender_object_id: 1,
                        object_id: sender_object_id,
                        code: super::super::wayland::wl_display::enums::Error::InvalidMethod as u32,
                        message: format!(
                            "@{} opcode={} args={:?} not found",
                            sender_object_id, opcode, args
                        ),
                    }))
                    .map_err(|_| ())
                    .map(|_tx| ()),
                );
            };
            let height = if let Ok(x) = cursor.read_i32::<NativeEndian>() {
                x
            } else {
                return Box::new(
                    tx.send(Box::new(super::super::wayland::wl_display::events::Error {
                        sender_object_id: 1,
                        object_id: sender_object_id,
                        code: super::super::wayland::wl_display::enums::Error::InvalidMethod as u32,
                        message: format!(
                            "@{} opcode={} args={:?} not found",
                            sender_object_id, opcode, args
                        ),
                    }))
                    .map_err(|_| ())
                    .map(|_tx| ()),
                );
            };
            return WlRegion::add(request, session, tx, sender_object_id, x, y, width, height);
        }
        2 => {
            let x = if let Ok(x) = cursor.read_i32::<NativeEndian>() {
                x
            } else {
                return Box::new(
                    tx.send(Box::new(super::super::wayland::wl_display::events::Error {
                        sender_object_id: 1,
                        object_id: sender_object_id,
                        code: super::super::wayland::wl_display::enums::Error::InvalidMethod as u32,
                        message: format!(
                            "@{} opcode={} args={:?} not found",
                            sender_object_id, opcode, args
                        ),
                    }))
                    .map_err(|_| ())
                    .map(|_tx| ()),
                );
            };
            let y = if let Ok(x) = cursor.read_i32::<NativeEndian>() {
                x
            } else {
                return Box::new(
                    tx.send(Box::new(super::super::wayland::wl_display::events::Error {
                        sender_object_id: 1,
                        object_id: sender_object_id,
                        code: super::super::wayland::wl_display::enums::Error::InvalidMethod as u32,
                        message: format!(
                            "@{} opcode={} args={:?} not found",
                            sender_object_id, opcode, args
                        ),
                    }))
                    .map_err(|_| ())
                    .map(|_tx| ()),
                );
            };
            let width = if let Ok(x) = cursor.read_i32::<NativeEndian>() {
                x
            } else {
                return Box::new(
                    tx.send(Box::new(super::super::wayland::wl_display::events::Error {
                        sender_object_id: 1,
                        object_id: sender_object_id,
                        code: super::super::wayland::wl_display::enums::Error::InvalidMethod as u32,
                        message: format!(
                            "@{} opcode={} args={:?} not found",
                            sender_object_id, opcode, args
                        ),
                    }))
                    .map_err(|_| ())
                    .map(|_tx| ()),
                );
            };
            let height = if let Ok(x) = cursor.read_i32::<NativeEndian>() {
                x
            } else {
                return Box::new(
                    tx.send(Box::new(super::super::wayland::wl_display::events::Error {
                        sender_object_id: 1,
                        object_id: sender_object_id,
                        code: super::super::wayland::wl_display::enums::Error::InvalidMethod as u32,
                        message: format!(
                            "@{} opcode={} args={:?} not found",
                            sender_object_id, opcode, args
                        ),
                    }))
                    .map_err(|_| ())
                    .map(|_tx| ()),
                );
            };
            return WlRegion::subtract(request, session, tx, sender_object_id, x, y, width, height);
        }
        _ => {}
    };
    Box::new(futures::future::ok(()))
}

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
        request: Arc<RwLock<WlRegion>>,
        session: RwLock<super::super::session::Session>,
        tx: tokio::sync::mpsc::Sender<Box<super::super::event::Event + Send>>,
        sender_object_id: u32,
        x: i32,      // int: region-local x coordinate
        y: i32,      // int: region-local y coordinate
        width: i32,  // int: rectangle width
        height: i32, // int: rectangle height
    ) -> Box<futures::future::Future<Item = (), Error = ()> + Send> {
        Box::new(futures::future::ok(()))
    }

    // destroy region
    //
    // Destroy the region.  This will invalidate the object ID.
    pub fn destroy(
        request: Arc<RwLock<WlRegion>>,
        session: RwLock<super::super::session::Session>,
        tx: tokio::sync::mpsc::Sender<Box<super::super::event::Event + Send>>,
        sender_object_id: u32,
    ) -> Box<futures::future::Future<Item = (), Error = ()> + Send> {
        Box::new(futures::future::ok(()))
    }

    // subtract rectangle from region
    //
    // Subtract the specified rectangle from the region.
    pub fn subtract(
        request: Arc<RwLock<WlRegion>>,
        session: RwLock<super::super::session::Session>,
        tx: tokio::sync::mpsc::Sender<Box<super::super::event::Event + Send>>,
        sender_object_id: u32,
        x: i32,      // int: region-local x coordinate
        y: i32,      // int: region-local y coordinate
        width: i32,  // int: rectangle width
        height: i32, // int: rectangle height
    ) -> Box<futures::future::Future<Item = (), Error = ()> + Send> {
        Box::new(futures::future::ok(()))
    }
}
