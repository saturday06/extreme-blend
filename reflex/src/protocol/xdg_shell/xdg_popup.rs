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

#[allow(unused_imports)] use byteorder::{NativeEndian, ReadBytesExt};
#[allow(unused_imports)] use futures::future::Future;
#[allow(unused_imports)] use futures::sink::Sink;
#[allow(unused_imports)] use std::io::{Cursor, Read};
#[allow(unused_imports)] use std::sync::{Arc, RwLock};

pub mod enums {
    pub enum Error {
        InvalidGrab = 0, // tried to grab after being mapped
    }
}

pub mod events {
    use byteorder::{ByteOrder, NativeEndian};

    // configure the popup surface
    //
    // This event asks the popup surface to configure itself given the
    // configuration. The configured state should not be applied immediately.
    // See xdg_surface.configure for details.
    // 
    // The x and y arguments represent the position the popup was placed at
    // given the xdg_positioner rule, relative to the upper left corner of the
    // window geometry of the parent surface.
    pub struct Configure {
        pub sender_object_id: u32,
        pub x: i32, // int: x position relative to parent surface window geometry
        pub y: i32, // int: y position relative to parent surface window geometry
        pub width: i32, // int: window geometry width
        pub height: i32, // int: window geometry height
    }

    impl super::super::super::event::Event for Configure {
        fn encode(&self, dst: &mut bytes::BytesMut) -> Result<(), std::io::Error> {
            let total_len = 8 + 4 + 4 + 4 + 4;
            if total_len > 0xffff {
                return Err(std::io::Error::new(std::io::ErrorKind::Other, "Oops!"));
            }

            let i = dst.len();
            dst.resize(i + total_len, 0);

            NativeEndian::write_u32(&mut dst[i..], self.sender_object_id);
            NativeEndian::write_u32(&mut dst[i + 4..], ((total_len << 16) | 0) as u32);

            NativeEndian::write_i32(&mut dst[i + 8..], self.x);
            NativeEndian::write_i32(&mut dst[i + 8 + 4..], self.y);
            NativeEndian::write_i32(&mut dst[i + 8 + 4 + 4..], self.width);
            NativeEndian::write_i32(&mut dst[i + 8 + 4 + 4 + 4..], self.height);
            Ok(())
        }
    }

    // popup interaction is done
    //
    // The popup_done event is sent out when a popup is dismissed by the
    // compositor. The client should destroy the xdg_popup object at this
    // point.
    pub struct PopupDone {
        pub sender_object_id: u32,
    }

    impl super::super::super::event::Event for PopupDone {
        fn encode(&self, dst: &mut bytes::BytesMut) -> Result<(), std::io::Error> {
            let total_len = 8;
            if total_len > 0xffff {
                return Err(std::io::Error::new(std::io::ErrorKind::Other, "Oops!"));
            }

            let i = dst.len();
            dst.resize(i + total_len, 0);

            NativeEndian::write_u32(&mut dst[i..], self.sender_object_id);
            NativeEndian::write_u32(&mut dst[i + 4..], ((total_len << 16) | 1) as u32);

            Ok(())
        }
    }
}

pub fn dispatch_request(request: Arc<RwLock<XdgPopup>>, session: RwLock<super::super::session::Session>, tx: tokio::sync::mpsc::Sender<Box<super::super::event::Event + Send>>, sender_object_id: u32, opcode: u16, args: Vec<u8>) -> Box<futures::future::Future<Item = (), Error = ()> + Send> {
    let mut cursor = Cursor::new(&args);
    match opcode {
        0 => {
            return XdgPopup::destroy(request, session, tx, sender_object_id, )
        },
        1 => {
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
            let serial = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
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
            return XdgPopup::grab(request, session, tx, sender_object_id, seat, serial)
        },
        _ => {},
    };
    Box::new(futures::future::ok(()))
}

// short-lived, popup surfaces for menus
//
// A popup surface is a short-lived, temporary surface. It can be used to
// implement for example menus, popovers, tooltips and other similar user
// interface concepts.
// 
// A popup can be made to take an explicit grab. See xdg_popup.grab for
// details.
// 
// When the popup is dismissed, a popup_done event will be sent out, and at
// the same time the surface will be unmapped. See the xdg_popup.popup_done
// event for details.
// 
// Explicitly destroying the xdg_popup object will also dismiss the popup and
// unmap the surface. Clients that want to dismiss the popup when another
// surface of their own is clicked should dismiss the popup using the destroy
// request.
// 
// A newly created xdg_popup will be stacked on top of all previously created
// xdg_popup surfaces associated with the same xdg_toplevel.
// 
// The parent of an xdg_popup must be mapped (see the xdg_surface
// description) before the xdg_popup itself.
// 
// The x and y arguments passed when creating the popup object specify
// where the top left of the popup should be placed, relative to the
// local surface coordinates of the parent surface. See
// xdg_surface.get_popup. An xdg_popup must intersect with or be at least
// partially adjacent to its parent surface.
// 
// The client must call wl_surface.commit on the corresponding wl_surface
// for the xdg_popup state to take effect.
pub struct XdgPopup {
}

impl XdgPopup {
    // remove xdg_popup interface
    //
    // This destroys the popup. Explicitly destroying the xdg_popup
    // object will also dismiss the popup, and unmap the surface.
    // 
    // If this xdg_popup is not the "topmost" popup, a protocol error
    // will be sent.
    pub fn destroy(
        request: Arc<RwLock<XdgPopup>>,
        session: RwLock<super::super::session::Session>,
        tx: tokio::sync::mpsc::Sender<Box<super::super::event::Event + Send>>,
        sender_object_id: u32,
    ) -> Box<futures::future::Future<Item = (), Error = ()> + Send> {
        Box::new(futures::future::ok(()))
    }

    // make the popup take an explicit grab
    //
    // This request makes the created popup take an explicit grab. An explicit
    // grab will be dismissed when the user dismisses the popup, or when the
    // client destroys the xdg_popup. This can be done by the user clicking
    // outside the surface, using the keyboard, or even locking the screen
    // through closing the lid or a timeout.
    // 
    // If the compositor denies the grab, the popup will be immediately
    // dismissed.
    // 
    // This request must be used in response to some sort of user action like a
    // button press, key press, or touch down event. The serial number of the
    // event should be passed as 'serial'.
    // 
    // The parent of a grabbing popup must either be an xdg_toplevel surface or
    // another xdg_popup with an explicit grab. If the parent is another
    // xdg_popup it means that the popups are nested, with this popup now being
    // the topmost popup.
    // 
    // Nested popups must be destroyed in the reverse order they were created
    // in, e.g. the only popup you are allowed to destroy at all times is the
    // topmost one.
    // 
    // When compositors choose to dismiss a popup, they may dismiss every
    // nested grabbing popup as well. When a compositor dismisses popups, it
    // will follow the same dismissing order as required from the client.
    // 
    // The parent of a grabbing popup must either be another xdg_popup with an
    // active explicit grab, or an xdg_popup or xdg_toplevel, if there are no
    // explicit grabs already taken.
    // 
    // If the topmost grabbing popup is destroyed, the grab will be returned to
    // the parent of the popup, if that parent previously had an explicit grab.
    // 
    // If the parent is a grabbing popup which has already been dismissed, this
    // popup will be immediately dismissed. If the parent is a popup that did
    // not take an explicit grab, an error will be raised.
    // 
    // During a popup grab, the client owning the grab will receive pointer
    // and touch events for all their surfaces as normal (similar to an
    // "owner-events" grab in X11 parlance), while the top most grabbing popup
    // will always have keyboard focus.
    pub fn grab(
        request: Arc<RwLock<XdgPopup>>,
        session: RwLock<super::super::session::Session>,
        tx: tokio::sync::mpsc::Sender<Box<super::super::event::Event + Send>>,
        sender_object_id: u32,
        seat: u32, // object: the wl_seat of the user event
        serial: u32, // uint: the serial of the user event
    ) -> Box<futures::future::Future<Item = (), Error = ()> + Send> {
        Box::new(futures::future::ok(()))
    }
}
