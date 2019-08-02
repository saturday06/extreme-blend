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

use byteorder::{NativeEndian, ReadBytesExt};
use futures::future::Future;
use futures::sink::Sink;
use std::io::{Cursor, Read};
use std::sync::Arc;
use std::cell::RefCell;

pub mod enums {
    // different method to set the surface fullscreen
    //
    // Hints to indicate to the compositor how to deal with a conflict
    // between the dimensions of the surface and the dimensions of the
    // output. The compositor is free to ignore this parameter.
    pub enum FullscreenMethod {
        Default = 0, // no preference, apply default policy
        Scale = 1, // scale, preserve the surface's aspect ratio and center on output
        Driver = 2, // switch output mode to the smallest mode that can fit the surface, add black borders to compensate size mismatch
        Fill = 3, // no upscaling, center on output and add black borders to compensate size mismatch
    }

    // edge values for resizing
    //
    // These values are used to indicate which edge of a surface
    // is being dragged in a resize operation. The server may
    // use this information to adapt its behavior, e.g. choose
    // an appropriate cursor image.
    pub enum Resize {
        None = 0, // no edge
        Top = 1, // top edge
        Bottom = 2, // bottom edge
        Left = 4, // left edge
        TopLeft = 5, // top and left edges
        BottomLeft = 6, // bottom and left edges
        Right = 8, // right edge
        TopRight = 9, // top and right edges
        BottomRight = 10, // bottom and right edges
    }

    // details of transient behaviour
    //
    // These flags specify details of the expected behaviour
    // of transient surfaces. Used in the set_transient request.
    pub enum Transient {
        Inactive = 0x1, // do not set keyboard focus
    }
}

pub mod events {
    use byteorder::{ByteOrder, NativeEndian};

    // suggest resize
    //
    // The configure event asks the client to resize its surface.
    // 
    // The size is a hint, in the sense that the client is free to
    // ignore it if it doesn't resize, pick a smaller size (to
    // satisfy aspect ratio or resize in steps of NxM pixels).
    // 
    // The edges parameter provides a hint about how the surface
    // was resized. The client may use this information to decide
    // how to adjust its content to the new size (e.g. a scrolling
    // area might adjust its content position to leave the viewable
    // content unmoved).
    // 
    // The client is free to dismiss all but the last configure
    // event it received.
    // 
    // The width and height arguments specify the size of the window
    // in surface-local coordinates.
    pub struct Configure {
        pub sender_object_id: u32,
        pub edges: u32, // uint: how the surface was resized
        pub width: i32, // int: new width of the surface
        pub height: i32, // int: new height of the surface
    }

    impl super::super::super::event::Event for Configure {
        fn encode(&self, dst: &mut bytes::BytesMut) -> Result<(), std::io::Error> {
            let total_len = 8 + 4 + 4 + 4;
            if total_len > 0xffff {
                return Err(std::io::Error::new(std::io::ErrorKind::Other, "Oops!"));
            }

            let i = dst.len();
            dst.resize(i + total_len, 0);

            NativeEndian::write_u32(&mut dst[i..], self.sender_object_id);
            NativeEndian::write_u32(&mut dst[i + 4..], ((total_len << 16) | 1) as u32);

            NativeEndian::write_u32(&mut dst[i + 8..], self.edges);
            NativeEndian::write_i32(&mut dst[i + 8 + 4..], self.width);
            NativeEndian::write_i32(&mut dst[i + 8 + 4 + 4..], self.height);
            Ok(())
        }
    }

    // ping client
    //
    // Ping a client to check if it is receiving events and sending
    // requests. A client is expected to reply with a pong request.
    pub struct Ping {
        pub sender_object_id: u32,
        pub serial: u32, // uint: serial number of the ping
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

    // popup interaction is done
    //
    // The popup_done event is sent out when a popup grab is broken,
    // that is, when the user clicks a surface that doesn't belong
    // to the client owning the popup surface.
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
            NativeEndian::write_u32(&mut dst[i + 4..], ((total_len << 16) | 2) as u32);

            Ok(())
        }
    }
}

pub fn dispatch_request(request: Arc<RefCell<WlShellSurface>>, session: &mut super::super::session::Session, tx: tokio::sync::mpsc::Sender<Box<super::super::event::Event + Send>>, sender_object_id: u32, opcode: u16, args: Vec<u8>) -> Box<futures::future::Future<Item = (), Error = ()>> {
    let mut cursor = Cursor::new(&args);
    match opcode {
        0 => {
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
            return WlShellSurface::pong(request, session, tx, sender_object_id, serial)
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
            return WlShellSurface::move_fn(request, session, tx, sender_object_id, seat, serial)
        },
        2 => {
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
            let edges = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
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
            return WlShellSurface::resize(request, session, tx, sender_object_id, seat, serial, edges)
        },
        3 => {
            return WlShellSurface::set_toplevel(request, session, tx, sender_object_id, )
        },
        4 => {
            let parent = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
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
            let x = if let Ok(x) = cursor.read_i32::<NativeEndian>() {
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
            let y = if let Ok(x) = cursor.read_i32::<NativeEndian>() {
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
            let flags = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
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
            return WlShellSurface::set_transient(request, session, tx, sender_object_id, parent, x, y, flags)
        },
        5 => {
            let method = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
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
            let framerate = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
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
            let output = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
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
            return WlShellSurface::set_fullscreen(request, session, tx, sender_object_id, method, framerate, output)
        },
        6 => {
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
            let parent = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
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
            let x = if let Ok(x) = cursor.read_i32::<NativeEndian>() {
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
            let y = if let Ok(x) = cursor.read_i32::<NativeEndian>() {
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
            let flags = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
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
            return WlShellSurface::set_popup(request, session, tx, sender_object_id, seat, serial, parent, x, y, flags)
        },
        7 => {
            let output = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
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
            return WlShellSurface::set_maximized(request, session, tx, sender_object_id, output)
        },
        8 => {
            let title = {
                let buf_len = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
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
                let padded_buf_len = (buf_len + 3) / 4 * 4;
                let mut buf = Vec::new();
                buf.resize(buf_len as usize, 0);
                if let Err(_) = cursor.read_exact(&mut buf) {
                return Box::new(tx.send(Box::new(super::super::wayland::wl_display::events::Error {
                    sender_object_id: 1,
                    object_id: sender_object_id,
                    code: super::super::wayland::wl_display::enums::Error::InvalidMethod as u32,
                    message: format!(
                        "@{} opcode={} args={:?} not found",
                        sender_object_id, opcode, args
                    ),
                })).map_err(|_| ()).map(|_tx| ()));

                }
                let s = if let Ok(x) = String::from_utf8(buf) {
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
                cursor.set_position(cursor.position() + (padded_buf_len - buf_len) as u64);
                s
            };
            return WlShellSurface::set_title(request, session, tx, sender_object_id, title)
        },
        9 => {
            let class_ = {
                let buf_len = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
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
                let padded_buf_len = (buf_len + 3) / 4 * 4;
                let mut buf = Vec::new();
                buf.resize(buf_len as usize, 0);
                if let Err(_) = cursor.read_exact(&mut buf) {
                return Box::new(tx.send(Box::new(super::super::wayland::wl_display::events::Error {
                    sender_object_id: 1,
                    object_id: sender_object_id,
                    code: super::super::wayland::wl_display::enums::Error::InvalidMethod as u32,
                    message: format!(
                        "@{} opcode={} args={:?} not found",
                        sender_object_id, opcode, args
                    ),
                })).map_err(|_| ()).map(|_tx| ()));

                }
                let s = if let Ok(x) = String::from_utf8(buf) {
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
                cursor.set_position(cursor.position() + (padded_buf_len - buf_len) as u64);
                s
            };
            return WlShellSurface::set_class(request, session, tx, sender_object_id, class_)
        },
        _ => {},
    };
    Box::new(futures::future::ok(()))
}

// desktop-style metadata interface
//
// An interface that may be implemented by a wl_surface, for
// implementations that provide a desktop-style user interface.
// 
// It provides requests to treat surfaces like toplevel, fullscreen
// or popup windows, move, resize or maximize them, associate
// metadata like title and class, etc.
// 
// On the server side the object is automatically destroyed when
// the related wl_surface is destroyed. On the client side,
// wl_shell_surface_destroy() must be called before destroying
// the wl_surface object.
pub struct WlShellSurface {
}

impl WlShellSurface {
    // start an interactive move
    //
    // Start a pointer-driven move of the surface.
    // 
    // This request must be used in response to a button press event.
    // The server may ignore move requests depending on the state of
    // the surface (e.g. fullscreen or maximized).
    pub fn move_fn(
        request: Arc<RefCell<WlShellSurface>>,
        session: &mut super::super::session::Session,
        tx: tokio::sync::mpsc::Sender<Box<super::super::event::Event + Send>>,
        sender_object_id: u32,
        seat: u32, // object: seat whose pointer is used
        serial: u32, // uint: serial number of the implicit grab on the pointer
    ) -> Box<futures::future::Future<Item = (), Error = ()>> {
        Box::new(futures::future::ok(()))
    }

    // respond to a ping event
    //
    // A client must respond to a ping event with a pong request or
    // the client may be deemed unresponsive.
    pub fn pong(
        request: Arc<RefCell<WlShellSurface>>,
        session: &mut super::super::session::Session,
        tx: tokio::sync::mpsc::Sender<Box<super::super::event::Event + Send>>,
        sender_object_id: u32,
        serial: u32, // uint: serial number of the ping event
    ) -> Box<futures::future::Future<Item = (), Error = ()>> {
        Box::new(futures::future::ok(()))
    }

    // start an interactive resize
    //
    // Start a pointer-driven resizing of the surface.
    // 
    // This request must be used in response to a button press event.
    // The server may ignore resize requests depending on the state of
    // the surface (e.g. fullscreen or maximized).
    pub fn resize(
        request: Arc<RefCell<WlShellSurface>>,
        session: &mut super::super::session::Session,
        tx: tokio::sync::mpsc::Sender<Box<super::super::event::Event + Send>>,
        sender_object_id: u32,
        seat: u32, // object: seat whose pointer is used
        serial: u32, // uint: serial number of the implicit grab on the pointer
        edges: u32, // uint: which edge or corner is being dragged
    ) -> Box<futures::future::Future<Item = (), Error = ()>> {
        Box::new(futures::future::ok(()))
    }

    // set surface class
    //
    // Set a class for the surface.
    // 
    // The surface class identifies the general class of applications
    // to which the surface belongs. A common convention is to use the
    // file name (or the full path if it is a non-standard location) of
    // the application's .desktop file as the class.
    pub fn set_class(
        request: Arc<RefCell<WlShellSurface>>,
        session: &mut super::super::session::Session,
        tx: tokio::sync::mpsc::Sender<Box<super::super::event::Event + Send>>,
        sender_object_id: u32,
        class_: String, // string: surface class
    ) -> Box<futures::future::Future<Item = (), Error = ()>> {
        Box::new(futures::future::ok(()))
    }

    // make the surface a fullscreen surface
    //
    // Map the surface as a fullscreen surface.
    // 
    // If an output parameter is given then the surface will be made
    // fullscreen on that output. If the client does not specify the
    // output then the compositor will apply its policy - usually
    // choosing the output on which the surface has the biggest surface
    // area.
    // 
    // The client may specify a method to resolve a size conflict
    // between the output size and the surface size - this is provided
    // through the method parameter.
    // 
    // The framerate parameter is used only when the method is set
    // to "driver", to indicate the preferred framerate. A value of 0
    // indicates that the client does not care about framerate.  The
    // framerate is specified in mHz, that is framerate of 60000 is 60Hz.
    // 
    // A method of "scale" or "driver" implies a scaling operation of
    // the surface, either via a direct scaling operation or a change of
    // the output mode. This will override any kind of output scaling, so
    // that mapping a surface with a buffer size equal to the mode can
    // fill the screen independent of buffer_scale.
    // 
    // A method of "fill" means we don't scale up the buffer, however
    // any output scale is applied. This means that you may run into
    // an edge case where the application maps a buffer with the same
    // size of the output mode but buffer_scale 1 (thus making a
    // surface larger than the output). In this case it is allowed to
    // downscale the results to fit the screen.
    // 
    // The compositor must reply to this request with a configure event
    // with the dimensions for the output on which the surface will
    // be made fullscreen.
    pub fn set_fullscreen(
        request: Arc<RefCell<WlShellSurface>>,
        session: &mut super::super::session::Session,
        tx: tokio::sync::mpsc::Sender<Box<super::super::event::Event + Send>>,
        sender_object_id: u32,
        method: u32, // uint: method for resolving size conflict
        framerate: u32, // uint: framerate in mHz
        output: u32, // object: output on which the surface is to be fullscreen
    ) -> Box<futures::future::Future<Item = (), Error = ()>> {
        Box::new(futures::future::ok(()))
    }

    // make the surface a maximized surface
    //
    // Map the surface as a maximized surface.
    // 
    // If an output parameter is given then the surface will be
    // maximized on that output. If the client does not specify the
    // output then the compositor will apply its policy - usually
    // choosing the output on which the surface has the biggest surface
    // area.
    // 
    // The compositor will reply with a configure event telling
    // the expected new surface size. The operation is completed
    // on the next buffer attach to this surface.
    // 
    // A maximized surface typically fills the entire output it is
    // bound to, except for desktop elements such as panels. This is
    // the main difference between a maximized shell surface and a
    // fullscreen shell surface.
    // 
    // The details depend on the compositor implementation.
    pub fn set_maximized(
        request: Arc<RefCell<WlShellSurface>>,
        session: &mut super::super::session::Session,
        tx: tokio::sync::mpsc::Sender<Box<super::super::event::Event + Send>>,
        sender_object_id: u32,
        output: u32, // object: output on which the surface is to be maximized
    ) -> Box<futures::future::Future<Item = (), Error = ()>> {
        Box::new(futures::future::ok(()))
    }

    // make the surface a popup surface
    //
    // Map the surface as a popup.
    // 
    // A popup surface is a transient surface with an added pointer
    // grab.
    // 
    // An existing implicit grab will be changed to owner-events mode,
    // and the popup grab will continue after the implicit grab ends
    // (i.e. releasing the mouse button does not cause the popup to
    // be unmapped).
    // 
    // The popup grab continues until the window is destroyed or a
    // mouse button is pressed in any other client's window. A click
    // in any of the client's surfaces is reported as normal, however,
    // clicks in other clients' surfaces will be discarded and trigger
    // the callback.
    // 
    // The x and y arguments specify the location of the upper left
    // corner of the surface relative to the upper left corner of the
    // parent surface, in surface-local coordinates.
    pub fn set_popup(
        request: Arc<RefCell<WlShellSurface>>,
        session: &mut super::super::session::Session,
        tx: tokio::sync::mpsc::Sender<Box<super::super::event::Event + Send>>,
        sender_object_id: u32,
        seat: u32, // object: seat whose pointer is used
        serial: u32, // uint: serial number of the implicit grab on the pointer
        parent: u32, // object: parent surface
        x: i32, // int: surface-local x coordinate
        y: i32, // int: surface-local y coordinate
        flags: u32, // uint: transient surface behavior
    ) -> Box<futures::future::Future<Item = (), Error = ()>> {
        Box::new(futures::future::ok(()))
    }

    // set surface title
    //
    // Set a short title for the surface.
    // 
    // This string may be used to identify the surface in a task bar,
    // window list, or other user interface elements provided by the
    // compositor.
    // 
    // The string must be encoded in UTF-8.
    pub fn set_title(
        request: Arc<RefCell<WlShellSurface>>,
        session: &mut super::super::session::Session,
        tx: tokio::sync::mpsc::Sender<Box<super::super::event::Event + Send>>,
        sender_object_id: u32,
        title: String, // string: surface title
    ) -> Box<futures::future::Future<Item = (), Error = ()>> {
        Box::new(futures::future::ok(()))
    }

    // make the surface a toplevel surface
    //
    // Map the surface as a toplevel surface.
    // 
    // A toplevel surface is not fullscreen, maximized or transient.
    pub fn set_toplevel(
        request: Arc<RefCell<WlShellSurface>>,
        session: &mut super::super::session::Session,
        tx: tokio::sync::mpsc::Sender<Box<super::super::event::Event + Send>>,
        sender_object_id: u32,
    ) -> Box<futures::future::Future<Item = (), Error = ()>> {
        Box::new(futures::future::ok(()))
    }

    // make the surface a transient surface
    //
    // Map the surface relative to an existing surface.
    // 
    // The x and y arguments specify the location of the upper left
    // corner of the surface relative to the upper left corner of the
    // parent surface, in surface-local coordinates.
    // 
    // The flags argument controls details of the transient behaviour.
    pub fn set_transient(
        request: Arc<RefCell<WlShellSurface>>,
        session: &mut super::super::session::Session,
        tx: tokio::sync::mpsc::Sender<Box<super::super::event::Event + Send>>,
        sender_object_id: u32,
        parent: u32, // object: parent surface
        x: i32, // int: surface-local x coordinate
        y: i32, // int: surface-local y coordinate
        flags: u32, // uint: transient surface behavior
    ) -> Box<futures::future::Future<Item = (), Error = ()>> {
        Box::new(futures::future::ok(()))
    }
}
