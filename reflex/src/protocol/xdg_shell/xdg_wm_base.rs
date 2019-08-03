// Copyright © 2008-2013 Kristian Høgsberg
// Copyright © 2013      Rafael Antognolli
// Copyright © 2013      Jasper St. Pierre
// Copyright © 2010-2013 Intel Corporation
// Copyright © 2015-2017 Samsung Electronics Co., Ltd
// Copyright © 2015-2017 Red Hat Inc.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice (including the next
// paragraph) shall be included in all copies or substantial portions of the
// Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

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

pub mod enums {
    pub enum Error {
        Role = 0,                // given wl_surface has another role
        DefunctSurfaces = 1,     // xdg_wm_base was destroyed before children
        NotTheTopmostPopup = 2,  // the client tried to map or destroy a non-topmost popup
        InvalidPopupParent = 3,  // the client specified an invalid popup parent surface
        InvalidSurfaceState = 4, // the client provided an invalid surface state
        InvalidPositioner = 5,   // the client provided an invalid positioner
    }
}

pub mod events {
    use byteorder::{ByteOrder, NativeEndian};

    // check if the client is alive
    //
    // The ping event asks the client if it's still alive. Pass the
    // serial specified in the event back to the compositor by sending
    // a "pong" request back with the specified serial. See xdg_wm_base.ping.
    //
    // Compositors can use this to determine if the client is still
    // alive. It's unspecified what will happen if the client doesn't
    // respond to the ping request, or in what timeframe. Clients should
    // try to respond in a reasonable amount of time.
    //
    // A compositor is free to ping in any way it wants, but a client must
    // always respond to any xdg_wm_base object it created.
    pub struct Ping {
        pub sender_object_id: u32,
        pub serial: u32, // uint: pass this to the pong request
    }

    impl super::super::super::event::Event for Ping {
        fn encode(&self, dst: &mut bytes::BytesMut) -> Result<(), std::io::Error> {
            let total_len = 8 + 4;
            if total_len > 0xffff {
                return Err(std::io::Error::new(std::io::ErrorKind::Other, "Oops!"));
            }

            let i = dst.len();
            dst.resize(i + total_len, 0);

            NativeEndian::write_u32(&mut dst[i..], self.sender_object_id);
            NativeEndian::write_u32(&mut dst[i + 4..], ((total_len << 16) | 0) as u32);

            NativeEndian::write_u32(&mut dst[i + 8..], self.serial);
            Ok(())
        }
    }
}

pub fn dispatch_request(
    request: Arc<RwLock<XdgWmBase>>,
    session: RwLock<super::super::session::Session>,
    tx: tokio::sync::mpsc::Sender<Box<super::super::event::Event + Send>>,
    sender_object_id: u32,
    opcode: u16,
    args: Vec<u8>,
) -> Box<futures::future::Future<Item = (), Error = ()> + Send> {
    let mut cursor = Cursor::new(&args);
    match opcode {
        0 => return XdgWmBase::destroy(request, session, tx, sender_object_id),
        1 => {
            let id = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
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
            return XdgWmBase::create_positioner(request, session, tx, sender_object_id, id);
        }
        2 => {
            let id = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
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
            let surface = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
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
            return XdgWmBase::get_xdg_surface(request, session, tx, sender_object_id, id, surface);
        }
        3 => {
            let serial = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
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
            return XdgWmBase::pong(request, session, tx, sender_object_id, serial);
        }
        _ => {}
    };
    Box::new(futures::future::ok(()))
}

// create desktop-style surfaces
//
// The xdg_wm_base interface is exposed as a global object enabling clients
// to turn their wl_surfaces into windows in a desktop environment. It
// defines the basic functionality needed for clients and the compositor to
// create windows that can be dragged, resized, maximized, etc, as well as
// creating transient windows such as popup menus.
pub struct XdgWmBase {}

impl XdgWmBase {
    // create a positioner object
    //
    // Create a positioner object. A positioner object is used to position
    // surfaces relative to some parent surface. See the interface description
    // and xdg_surface.get_popup for details.
    pub fn create_positioner(
        request: Arc<RwLock<XdgWmBase>>,
        session: RwLock<super::super::session::Session>,
        tx: tokio::sync::mpsc::Sender<Box<super::super::event::Event + Send>>,
        sender_object_id: u32,
        id: u32, // new_id:
    ) -> Box<futures::future::Future<Item = (), Error = ()> + Send> {
        Box::new(futures::future::ok(()))
    }

    // destroy xdg_wm_base
    //
    // Destroy this xdg_wm_base object.
    //
    // Destroying a bound xdg_wm_base object while there are surfaces
    // still alive created by this xdg_wm_base object instance is illegal
    // and will result in a protocol error.
    pub fn destroy(
        request: Arc<RwLock<XdgWmBase>>,
        session: RwLock<super::super::session::Session>,
        tx: tokio::sync::mpsc::Sender<Box<super::super::event::Event + Send>>,
        sender_object_id: u32,
    ) -> Box<futures::future::Future<Item = (), Error = ()> + Send> {
        Box::new(futures::future::ok(()))
    }

    // create a shell surface from a surface
    //
    // This creates an xdg_surface for the given surface. While xdg_surface
    // itself is not a role, the corresponding surface may only be assigned
    // a role extending xdg_surface, such as xdg_toplevel or xdg_popup.
    //
    // This creates an xdg_surface for the given surface. An xdg_surface is
    // used as basis to define a role to a given surface, such as xdg_toplevel
    // or xdg_popup. It also manages functionality shared between xdg_surface
    // based surface roles.
    //
    // See the documentation of xdg_surface for more details about what an
    // xdg_surface is and how it is used.
    pub fn get_xdg_surface(
        request: Arc<RwLock<XdgWmBase>>,
        session: RwLock<super::super::session::Session>,
        tx: tokio::sync::mpsc::Sender<Box<super::super::event::Event + Send>>,
        sender_object_id: u32,
        id: u32,      // new_id:
        surface: u32, // object:
    ) -> Box<futures::future::Future<Item = (), Error = ()> + Send> {
        Box::new(futures::future::ok(()))
    }

    // respond to a ping event
    //
    // A client must respond to a ping event with a pong request or
    // the client may be deemed unresponsive. See xdg_wm_base.ping.
    pub fn pong(
        request: Arc<RwLock<XdgWmBase>>,
        session: RwLock<super::super::session::Session>,
        tx: tokio::sync::mpsc::Sender<Box<super::super::event::Event + Send>>,
        sender_object_id: u32,
        serial: u32, // uint: serial of the ping event
    ) -> Box<futures::future::Future<Item = (), Error = ()> + Send> {
        Box::new(futures::future::ok(()))
    }
}
