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
    // edge values for resizing
    //
    // These values are used to indicate which edge of a surface
    // is being dragged in a resize operation.
    pub enum ResizeEdge {
        None = 0, // 
        Top = 1, // 
        Bottom = 2, // 
        Left = 4, // 
        TopLeft = 5, // 
        BottomLeft = 6, // 
        Right = 8, // 
        TopRight = 9, // 
        BottomRight = 10, // 
    }

    // types of state on the surface
    //
    // The different state values used on the surface. This is designed for
    // state values like maximized, fullscreen. It is paired with the
    // configure event to ensure that both the client and the compositor
    // setting the state can be synchronized.
    // 
    // States set in this way are double-buffered. They will get applied on
    // the next commit.
    pub enum State {
        Maximized = 1, // the surface is maximized
        Fullscreen = 2, // the surface is fullscreen
        Resizing = 3, // the surface is being resized
        Activated = 4, // the surface is now activated
        TiledLeft = 5, // 
        TiledRight = 6, // 
        TiledTop = 7, // 
        TiledBottom = 8, // 
    }
}

pub mod events {
    use byteorder::{ByteOrder, NativeEndian};

    // surface wants to be closed
    //
    // The close event is sent by the compositor when the user
    // wants the surface to be closed. This should be equivalent to
    // the user clicking the close button in client-side decorations,
    // if your application has any.
    // 
    // This is only a request that the user intends to close the
    // window. The client may choose to ignore this request, or show
    // a dialog to ask the user to save their data, etc.
    pub struct Close {
        pub sender_object_id: u32,
    }

    impl super::super::super::event::Event for Close {
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

    // suggest a surface change
    //
    // This configure event asks the client to resize its toplevel surface or
    // to change its state. The configured state should not be applied
    // immediately. See xdg_surface.configure for details.
    // 
    // The width and height arguments specify a hint to the window
    // about how its surface should be resized in window geometry
    // coordinates. See set_window_geometry.
    // 
    // If the width or height arguments are zero, it means the client
    // should decide its own window dimension. This may happen when the
    // compositor needs to configure the state of the surface but doesn't
    // have any information about any previous or expected dimension.
    // 
    // The states listed in the event specify how the width/height
    // arguments should be interpreted, and possibly how it should be
    // drawn.
    // 
    // Clients must send an ack_configure in response to this event. See
    // xdg_surface.configure and xdg_surface.ack_configure for details.
    pub struct Configure {
        pub sender_object_id: u32,
        pub width: i32, // int: 
        pub height: i32, // int: 
        pub states: Vec<u8>, // array: 
    }

    impl super::super::super::event::Event for Configure {
        fn encode(&self, dst: &mut bytes::BytesMut) -> Result<(), std::io::Error> {
            let total_len = 8 + 4 + 4 + (4 + (self.states.len() + 1 + 3) / 4 * 4);
            if total_len > 0xffff {
                return Err(std::io::Error::new(std::io::ErrorKind::Other, "Oops!"));
            }

            let i = dst.len();
            dst.resize(i + total_len, 0);

            NativeEndian::write_u32(&mut dst[i..], self.sender_object_id);
            NativeEndian::write_u32(&mut dst[i + 4..], ((total_len << 16) | 0) as u32);

            NativeEndian::write_i32(&mut dst[i + 8..], self.width);
            NativeEndian::write_i32(&mut dst[i + 8 + 4..], self.height);
            
            NativeEndian::write_u32(&mut dst[i + 8 + 4 + 4..], self.states.len() as u32);
            let mut aligned_states = self.states.clone();
            while aligned_states.len() % 4 != 0 {
                aligned_states.push(0u8);
            }
            dst[(i + 8 + 4 + 4 + 4)..(i + 8 + 4 + 4 + 4 + aligned_states.len())].copy_from_slice(&aligned_states[..]);

            Ok(())
        }
    }
}

pub fn dispatch_request(request: Arc<RwLock<XdgToplevel>>, session: crate::protocol::session::Session, sender_object_id: u32, opcode: u16, args: Vec<u8>) -> Box<futures::future::Future<Item = crate::protocol::session::Session, Error = ()> + Send> {
    let mut cursor = Cursor::new(&args);
    match opcode {
        0 => {
            return XdgToplevel::destroy(request, session, sender_object_id, )
        },
        1 => {
            let parent = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
                x 
            } else {
                let tx = session.tx.clone();
                return Box::new(tx.send(Box::new(super::super::wayland::wl_display::events::Error {
                    sender_object_id: 1,
                    object_id: sender_object_id,
                    code: super::super::wayland::wl_display::enums::Error::InvalidMethod as u32,
                    message: format!(
                        "@{} opcode={} args={:?} not found",
                        sender_object_id, opcode, args
                    ),
                })).map_err(|_| ()).map(|_tx| session));

            };
            return XdgToplevel::set_parent(request, session, sender_object_id, parent)
        },
        2 => {
            let title = {
                let buf_len = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
                    x
                } else {
                let tx = session.tx.clone();
                return Box::new(tx.send(Box::new(super::super::wayland::wl_display::events::Error {
                    sender_object_id: 1,
                    object_id: sender_object_id,
                    code: super::super::wayland::wl_display::enums::Error::InvalidMethod as u32,
                    message: format!(
                        "@{} opcode={} args={:?} not found",
                        sender_object_id, opcode, args
                    ),
                })).map_err(|_| ()).map(|_tx| session));

                };
                let padded_buf_len = (buf_len + 3) / 4 * 4;
                let mut buf = Vec::new();
                buf.resize(buf_len as usize, 0);
                if let Err(_) = cursor.read_exact(&mut buf) {
                let tx = session.tx.clone();
                return Box::new(tx.send(Box::new(super::super::wayland::wl_display::events::Error {
                    sender_object_id: 1,
                    object_id: sender_object_id,
                    code: super::super::wayland::wl_display::enums::Error::InvalidMethod as u32,
                    message: format!(
                        "@{} opcode={} args={:?} not found",
                        sender_object_id, opcode, args
                    ),
                })).map_err(|_| ()).map(|_tx| session));

                }
                let s = if let Ok(x) = String::from_utf8(buf) {
                    x
                } else {
                let tx = session.tx.clone();
                return Box::new(tx.send(Box::new(super::super::wayland::wl_display::events::Error {
                    sender_object_id: 1,
                    object_id: sender_object_id,
                    code: super::super::wayland::wl_display::enums::Error::InvalidMethod as u32,
                    message: format!(
                        "@{} opcode={} args={:?} not found",
                        sender_object_id, opcode, args
                    ),
                })).map_err(|_| ()).map(|_tx| session));

                };
                cursor.set_position(cursor.position() + (padded_buf_len - buf_len) as u64);
                s
            };
            return XdgToplevel::set_title(request, session, sender_object_id, title)
        },
        3 => {
            let app_id = {
                let buf_len = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
                    x
                } else {
                let tx = session.tx.clone();
                return Box::new(tx.send(Box::new(super::super::wayland::wl_display::events::Error {
                    sender_object_id: 1,
                    object_id: sender_object_id,
                    code: super::super::wayland::wl_display::enums::Error::InvalidMethod as u32,
                    message: format!(
                        "@{} opcode={} args={:?} not found",
                        sender_object_id, opcode, args
                    ),
                })).map_err(|_| ()).map(|_tx| session));

                };
                let padded_buf_len = (buf_len + 3) / 4 * 4;
                let mut buf = Vec::new();
                buf.resize(buf_len as usize, 0);
                if let Err(_) = cursor.read_exact(&mut buf) {
                let tx = session.tx.clone();
                return Box::new(tx.send(Box::new(super::super::wayland::wl_display::events::Error {
                    sender_object_id: 1,
                    object_id: sender_object_id,
                    code: super::super::wayland::wl_display::enums::Error::InvalidMethod as u32,
                    message: format!(
                        "@{} opcode={} args={:?} not found",
                        sender_object_id, opcode, args
                    ),
                })).map_err(|_| ()).map(|_tx| session));

                }
                let s = if let Ok(x) = String::from_utf8(buf) {
                    x
                } else {
                let tx = session.tx.clone();
                return Box::new(tx.send(Box::new(super::super::wayland::wl_display::events::Error {
                    sender_object_id: 1,
                    object_id: sender_object_id,
                    code: super::super::wayland::wl_display::enums::Error::InvalidMethod as u32,
                    message: format!(
                        "@{} opcode={} args={:?} not found",
                        sender_object_id, opcode, args
                    ),
                })).map_err(|_| ()).map(|_tx| session));

                };
                cursor.set_position(cursor.position() + (padded_buf_len - buf_len) as u64);
                s
            };
            return XdgToplevel::set_app_id(request, session, sender_object_id, app_id)
        },
        4 => {
            let seat = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
                x 
            } else {
                let tx = session.tx.clone();
                return Box::new(tx.send(Box::new(super::super::wayland::wl_display::events::Error {
                    sender_object_id: 1,
                    object_id: sender_object_id,
                    code: super::super::wayland::wl_display::enums::Error::InvalidMethod as u32,
                    message: format!(
                        "@{} opcode={} args={:?} not found",
                        sender_object_id, opcode, args
                    ),
                })).map_err(|_| ()).map(|_tx| session));

            };
            let serial = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
                x 
            } else {
                let tx = session.tx.clone();
                return Box::new(tx.send(Box::new(super::super::wayland::wl_display::events::Error {
                    sender_object_id: 1,
                    object_id: sender_object_id,
                    code: super::super::wayland::wl_display::enums::Error::InvalidMethod as u32,
                    message: format!(
                        "@{} opcode={} args={:?} not found",
                        sender_object_id, opcode, args
                    ),
                })).map_err(|_| ()).map(|_tx| session));

            };
            let x = if let Ok(x) = cursor.read_i32::<NativeEndian>() {
                x
            } else {
                let tx = session.tx.clone();
                return Box::new(tx.send(Box::new(super::super::wayland::wl_display::events::Error {
                    sender_object_id: 1,
                    object_id: sender_object_id,
                    code: super::super::wayland::wl_display::enums::Error::InvalidMethod as u32,
                    message: format!(
                        "@{} opcode={} args={:?} not found",
                        sender_object_id, opcode, args
                    ),
                })).map_err(|_| ()).map(|_tx| session));

            };
            let y = if let Ok(x) = cursor.read_i32::<NativeEndian>() {
                x
            } else {
                let tx = session.tx.clone();
                return Box::new(tx.send(Box::new(super::super::wayland::wl_display::events::Error {
                    sender_object_id: 1,
                    object_id: sender_object_id,
                    code: super::super::wayland::wl_display::enums::Error::InvalidMethod as u32,
                    message: format!(
                        "@{} opcode={} args={:?} not found",
                        sender_object_id, opcode, args
                    ),
                })).map_err(|_| ()).map(|_tx| session));

            };
            return XdgToplevel::show_window_menu(request, session, sender_object_id, seat, serial, x, y)
        },
        5 => {
            let seat = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
                x 
            } else {
                let tx = session.tx.clone();
                return Box::new(tx.send(Box::new(super::super::wayland::wl_display::events::Error {
                    sender_object_id: 1,
                    object_id: sender_object_id,
                    code: super::super::wayland::wl_display::enums::Error::InvalidMethod as u32,
                    message: format!(
                        "@{} opcode={} args={:?} not found",
                        sender_object_id, opcode, args
                    ),
                })).map_err(|_| ()).map(|_tx| session));

            };
            let serial = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
                x 
            } else {
                let tx = session.tx.clone();
                return Box::new(tx.send(Box::new(super::super::wayland::wl_display::events::Error {
                    sender_object_id: 1,
                    object_id: sender_object_id,
                    code: super::super::wayland::wl_display::enums::Error::InvalidMethod as u32,
                    message: format!(
                        "@{} opcode={} args={:?} not found",
                        sender_object_id, opcode, args
                    ),
                })).map_err(|_| ()).map(|_tx| session));

            };
            return XdgToplevel::move_fn(request, session, sender_object_id, seat, serial)
        },
        6 => {
            let seat = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
                x 
            } else {
                let tx = session.tx.clone();
                return Box::new(tx.send(Box::new(super::super::wayland::wl_display::events::Error {
                    sender_object_id: 1,
                    object_id: sender_object_id,
                    code: super::super::wayland::wl_display::enums::Error::InvalidMethod as u32,
                    message: format!(
                        "@{} opcode={} args={:?} not found",
                        sender_object_id, opcode, args
                    ),
                })).map_err(|_| ()).map(|_tx| session));

            };
            let serial = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
                x 
            } else {
                let tx = session.tx.clone();
                return Box::new(tx.send(Box::new(super::super::wayland::wl_display::events::Error {
                    sender_object_id: 1,
                    object_id: sender_object_id,
                    code: super::super::wayland::wl_display::enums::Error::InvalidMethod as u32,
                    message: format!(
                        "@{} opcode={} args={:?} not found",
                        sender_object_id, opcode, args
                    ),
                })).map_err(|_| ()).map(|_tx| session));

            };
            let edges = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
                x 
            } else {
                let tx = session.tx.clone();
                return Box::new(tx.send(Box::new(super::super::wayland::wl_display::events::Error {
                    sender_object_id: 1,
                    object_id: sender_object_id,
                    code: super::super::wayland::wl_display::enums::Error::InvalidMethod as u32,
                    message: format!(
                        "@{} opcode={} args={:?} not found",
                        sender_object_id, opcode, args
                    ),
                })).map_err(|_| ()).map(|_tx| session));

            };
            return XdgToplevel::resize(request, session, sender_object_id, seat, serial, edges)
        },
        7 => {
            let width = if let Ok(x) = cursor.read_i32::<NativeEndian>() {
                x
            } else {
                let tx = session.tx.clone();
                return Box::new(tx.send(Box::new(super::super::wayland::wl_display::events::Error {
                    sender_object_id: 1,
                    object_id: sender_object_id,
                    code: super::super::wayland::wl_display::enums::Error::InvalidMethod as u32,
                    message: format!(
                        "@{} opcode={} args={:?} not found",
                        sender_object_id, opcode, args
                    ),
                })).map_err(|_| ()).map(|_tx| session));

            };
            let height = if let Ok(x) = cursor.read_i32::<NativeEndian>() {
                x
            } else {
                let tx = session.tx.clone();
                return Box::new(tx.send(Box::new(super::super::wayland::wl_display::events::Error {
                    sender_object_id: 1,
                    object_id: sender_object_id,
                    code: super::super::wayland::wl_display::enums::Error::InvalidMethod as u32,
                    message: format!(
                        "@{} opcode={} args={:?} not found",
                        sender_object_id, opcode, args
                    ),
                })).map_err(|_| ()).map(|_tx| session));

            };
            return XdgToplevel::set_max_size(request, session, sender_object_id, width, height)
        },
        8 => {
            let width = if let Ok(x) = cursor.read_i32::<NativeEndian>() {
                x
            } else {
                let tx = session.tx.clone();
                return Box::new(tx.send(Box::new(super::super::wayland::wl_display::events::Error {
                    sender_object_id: 1,
                    object_id: sender_object_id,
                    code: super::super::wayland::wl_display::enums::Error::InvalidMethod as u32,
                    message: format!(
                        "@{} opcode={} args={:?} not found",
                        sender_object_id, opcode, args
                    ),
                })).map_err(|_| ()).map(|_tx| session));

            };
            let height = if let Ok(x) = cursor.read_i32::<NativeEndian>() {
                x
            } else {
                let tx = session.tx.clone();
                return Box::new(tx.send(Box::new(super::super::wayland::wl_display::events::Error {
                    sender_object_id: 1,
                    object_id: sender_object_id,
                    code: super::super::wayland::wl_display::enums::Error::InvalidMethod as u32,
                    message: format!(
                        "@{} opcode={} args={:?} not found",
                        sender_object_id, opcode, args
                    ),
                })).map_err(|_| ()).map(|_tx| session));

            };
            return XdgToplevel::set_min_size(request, session, sender_object_id, width, height)
        },
        9 => {
            return XdgToplevel::set_maximized(request, session, sender_object_id, )
        },
        10 => {
            return XdgToplevel::unset_maximized(request, session, sender_object_id, )
        },
        11 => {
            let output = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
                x 
            } else {
                let tx = session.tx.clone();
                return Box::new(tx.send(Box::new(super::super::wayland::wl_display::events::Error {
                    sender_object_id: 1,
                    object_id: sender_object_id,
                    code: super::super::wayland::wl_display::enums::Error::InvalidMethod as u32,
                    message: format!(
                        "@{} opcode={} args={:?} not found",
                        sender_object_id, opcode, args
                    ),
                })).map_err(|_| ()).map(|_tx| session));

            };
            return XdgToplevel::set_fullscreen(request, session, sender_object_id, output)
        },
        12 => {
            return XdgToplevel::unset_fullscreen(request, session, sender_object_id, )
        },
        13 => {
            return XdgToplevel::set_minimized(request, session, sender_object_id, )
        },
        _ => {},
    };
    Box::new(futures::future::ok(session))
}

// toplevel surface
//
// This interface defines an xdg_surface role which allows a surface to,
// among other things, set window-like properties such as maximize,
// fullscreen, and minimize, set application-specific metadata like title and
// id, and well as trigger user interactive operations such as interactive
// resize and move.
// 
// Unmapping an xdg_toplevel means that the surface cannot be shown
// by the compositor until it is explicitly mapped again.
// All active operations (e.g., move, resize) are canceled and all
// attributes (e.g. title, state, stacking, ...) are discarded for
// an xdg_toplevel surface when it is unmapped.
// 
// Attaching a null buffer to a toplevel unmaps the surface.
pub struct XdgToplevel {
}

impl XdgToplevel {
    // destroy the xdg_toplevel
    //
    // This request destroys the role surface and unmaps the surface;
    // see "Unmapping" behavior in interface section for details.
    pub fn destroy(
        request: Arc<RwLock<XdgToplevel>>,
        session: crate::protocol::session::Session,
        sender_object_id: u32,
    ) -> Box<futures::future::Future<Item = crate::protocol::session::Session, Error = ()> + Send> {
        Box::new(futures::future::ok(session))
    }

    // start an interactive move
    //
    // Start an interactive, user-driven move of the surface.
    // 
    // This request must be used in response to some sort of user action
    // like a button press, key press, or touch down event. The passed
    // serial is used to determine the type of interactive move (touch,
    // pointer, etc).
    // 
    // The server may ignore move requests depending on the state of
    // the surface (e.g. fullscreen or maximized), or if the passed serial
    // is no longer valid.
    // 
    // If triggered, the surface will lose the focus of the device
    // (wl_pointer, wl_touch, etc) used for the move. It is up to the
    // compositor to visually indicate that the move is taking place, such as
    // updating a pointer cursor, during the move. There is no guarantee
    // that the device focus will return when the move is completed.
    pub fn move_fn(
        request: Arc<RwLock<XdgToplevel>>,
        session: crate::protocol::session::Session,
        sender_object_id: u32,
        seat: u32, // object: the wl_seat of the user event
        serial: u32, // uint: the serial of the user event
    ) -> Box<futures::future::Future<Item = crate::protocol::session::Session, Error = ()> + Send> {
        Box::new(futures::future::ok(session))
    }

    // start an interactive resize
    //
    // Start a user-driven, interactive resize of the surface.
    // 
    // This request must be used in response to some sort of user action
    // like a button press, key press, or touch down event. The passed
    // serial is used to determine the type of interactive resize (touch,
    // pointer, etc).
    // 
    // The server may ignore resize requests depending on the state of
    // the surface (e.g. fullscreen or maximized).
    // 
    // If triggered, the client will receive configure events with the
    // "resize" state enum value and the expected sizes. See the "resize"
    // enum value for more details about what is required. The client
    // must also acknowledge configure events using "ack_configure". After
    // the resize is completed, the client will receive another "configure"
    // event without the resize state.
    // 
    // If triggered, the surface also will lose the focus of the device
    // (wl_pointer, wl_touch, etc) used for the resize. It is up to the
    // compositor to visually indicate that the resize is taking place,
    // such as updating a pointer cursor, during the resize. There is no
    // guarantee that the device focus will return when the resize is
    // completed.
    // 
    // The edges parameter specifies how the surface should be resized,
    // and is one of the values of the resize_edge enum. The compositor
    // may use this information to update the surface position for
    // example when dragging the top left corner. The compositor may also
    // use this information to adapt its behavior, e.g. choose an
    // appropriate cursor image.
    pub fn resize(
        request: Arc<RwLock<XdgToplevel>>,
        session: crate::protocol::session::Session,
        sender_object_id: u32,
        seat: u32, // object: the wl_seat of the user event
        serial: u32, // uint: the serial of the user event
        edges: u32, // uint: which edge or corner is being dragged
    ) -> Box<futures::future::Future<Item = crate::protocol::session::Session, Error = ()> + Send> {
        Box::new(futures::future::ok(session))
    }

    // set application ID
    //
    // Set an application identifier for the surface.
    // 
    // The app ID identifies the general class of applications to which
    // the surface belongs. The compositor can use this to group multiple
    // surfaces together, or to determine how to launch a new application.
    // 
    // For D-Bus activatable applications, the app ID is used as the D-Bus
    // service name.
    // 
    // The compositor shell will try to group application surfaces together
    // by their app ID. As a best practice, it is suggested to select app
    // ID's that match the basename of the application's .desktop file.
    // For example, "org.freedesktop.FooViewer" where the .desktop file is
    // "org.freedesktop.FooViewer.desktop".
    // 
    // See the desktop-entry specification [0] for more details on
    // application identifiers and how they relate to well-known D-Bus
    // names and .desktop files.
    // 
    // [0] http://standards.freedesktop.org/desktop-entry-spec/
    pub fn set_app_id(
        request: Arc<RwLock<XdgToplevel>>,
        session: crate::protocol::session::Session,
        sender_object_id: u32,
        app_id: String, // string: 
    ) -> Box<futures::future::Future<Item = crate::protocol::session::Session, Error = ()> + Send> {
        Box::new(futures::future::ok(session))
    }

    // set the window as fullscreen on an output
    //
    // Make the surface fullscreen.
    // 
    // After requesting that the surface should be fullscreened, the
    // compositor will respond by emitting a configure event. Whether the
    // client is actually put into a fullscreen state is subject to compositor
    // policies. The client must also acknowledge the configure when
    // committing the new content (see ack_configure).
    // 
    // The output passed by the request indicates the client's preference as
    // to which display it should be set fullscreen on. If this value is NULL,
    // it's up to the compositor to choose which display will be used to map
    // this surface.
    // 
    // If the surface doesn't cover the whole output, the compositor will
    // position the surface in the center of the output and compensate with
    // with border fill covering the rest of the output. The content of the
    // border fill is undefined, but should be assumed to be in some way that
    // attempts to blend into the surrounding area (e.g. solid black).
    // 
    // If the fullscreened surface is not opaque, the compositor must make
    // sure that other screen content not part of the same surface tree (made
    // up of subsurfaces, popups or similarly coupled surfaces) are not
    // visible below the fullscreened surface.
    pub fn set_fullscreen(
        request: Arc<RwLock<XdgToplevel>>,
        session: crate::protocol::session::Session,
        sender_object_id: u32,
        output: u32, // object: 
    ) -> Box<futures::future::Future<Item = crate::protocol::session::Session, Error = ()> + Send> {
        Box::new(futures::future::ok(session))
    }

    // set the maximum size
    //
    // Set a maximum size for the window.
    // 
    // The client can specify a maximum size so that the compositor does
    // not try to configure the window beyond this size.
    // 
    // The width and height arguments are in window geometry coordinates.
    // See xdg_surface.set_window_geometry.
    // 
    // Values set in this way are double-buffered. They will get applied
    // on the next commit.
    // 
    // The compositor can use this information to allow or disallow
    // different states like maximize or fullscreen and draw accurate
    // animations.
    // 
    // Similarly, a tiling window manager may use this information to
    // place and resize client windows in a more effective way.
    // 
    // The client should not rely on the compositor to obey the maximum
    // size. The compositor may decide to ignore the values set by the
    // client and request a larger size.
    // 
    // If never set, or a value of zero in the request, means that the
    // client has no expected maximum size in the given dimension.
    // As a result, a client wishing to reset the maximum size
    // to an unspecified state can use zero for width and height in the
    // request.
    // 
    // Requesting a maximum size to be smaller than the minimum size of
    // a surface is illegal and will result in a protocol error.
    // 
    // The width and height must be greater than or equal to zero. Using
    // strictly negative values for width and height will result in a
    // protocol error.
    pub fn set_max_size(
        request: Arc<RwLock<XdgToplevel>>,
        session: crate::protocol::session::Session,
        sender_object_id: u32,
        width: i32, // int: 
        height: i32, // int: 
    ) -> Box<futures::future::Future<Item = crate::protocol::session::Session, Error = ()> + Send> {
        Box::new(futures::future::ok(session))
    }

    // maximize the window
    //
    // Maximize the surface.
    // 
    // After requesting that the surface should be maximized, the compositor
    // will respond by emitting a configure event. Whether this configure
    // actually sets the window maximized is subject to compositor policies.
    // The client must then update its content, drawing in the configured
    // state. The client must also acknowledge the configure when committing
    // the new content (see ack_configure).
    // 
    // It is up to the compositor to decide how and where to maximize the
    // surface, for example which output and what region of the screen should
    // be used.
    // 
    // If the surface was already maximized, the compositor will still emit
    // a configure event with the "maximized" state.
    // 
    // If the surface is in a fullscreen state, this request has no direct
    // effect. It may alter the state the surface is returned to when
    // unmaximized unless overridden by the compositor.
    pub fn set_maximized(
        request: Arc<RwLock<XdgToplevel>>,
        session: crate::protocol::session::Session,
        sender_object_id: u32,
    ) -> Box<futures::future::Future<Item = crate::protocol::session::Session, Error = ()> + Send> {
        Box::new(futures::future::ok(session))
    }

    // set the minimum size
    //
    // Set a minimum size for the window.
    // 
    // The client can specify a minimum size so that the compositor does
    // not try to configure the window below this size.
    // 
    // The width and height arguments are in window geometry coordinates.
    // See xdg_surface.set_window_geometry.
    // 
    // Values set in this way are double-buffered. They will get applied
    // on the next commit.
    // 
    // The compositor can use this information to allow or disallow
    // different states like maximize or fullscreen and draw accurate
    // animations.
    // 
    // Similarly, a tiling window manager may use this information to
    // place and resize client windows in a more effective way.
    // 
    // The client should not rely on the compositor to obey the minimum
    // size. The compositor may decide to ignore the values set by the
    // client and request a smaller size.
    // 
    // If never set, or a value of zero in the request, means that the
    // client has no expected minimum size in the given dimension.
    // As a result, a client wishing to reset the minimum size
    // to an unspecified state can use zero for width and height in the
    // request.
    // 
    // Requesting a minimum size to be larger than the maximum size of
    // a surface is illegal and will result in a protocol error.
    // 
    // The width and height must be greater than or equal to zero. Using
    // strictly negative values for width and height will result in a
    // protocol error.
    pub fn set_min_size(
        request: Arc<RwLock<XdgToplevel>>,
        session: crate::protocol::session::Session,
        sender_object_id: u32,
        width: i32, // int: 
        height: i32, // int: 
    ) -> Box<futures::future::Future<Item = crate::protocol::session::Session, Error = ()> + Send> {
        Box::new(futures::future::ok(session))
    }

    // set the window as minimized
    //
    // Request that the compositor minimize your surface. There is no
    // way to know if the surface is currently minimized, nor is there
    // any way to unset minimization on this surface.
    // 
    // If you are looking to throttle redrawing when minimized, please
    // instead use the wl_surface.frame event for this, as this will
    // also work with live previews on windows in Alt-Tab, Expose or
    // similar compositor features.
    pub fn set_minimized(
        request: Arc<RwLock<XdgToplevel>>,
        session: crate::protocol::session::Session,
        sender_object_id: u32,
    ) -> Box<futures::future::Future<Item = crate::protocol::session::Session, Error = ()> + Send> {
        Box::new(futures::future::ok(session))
    }

    // set the parent of this surface
    //
    // Set the "parent" of this surface. This surface should be stacked
    // above the parent surface and all other ancestor surfaces.
    // 
    // Parent windows should be set on dialogs, toolboxes, or other
    // "auxiliary" surfaces, so that the parent is raised when the dialog
    // is raised.
    // 
    // Setting a null parent for a child window removes any parent-child
    // relationship for the child. Setting a null parent for a window which
    // currently has no parent is a no-op.
    // 
    // If the parent is unmapped then its children are managed as
    // though the parent of the now-unmapped parent has become the
    // parent of this surface. If no parent exists for the now-unmapped
    // parent then the children are managed as though they have no
    // parent surface.
    pub fn set_parent(
        request: Arc<RwLock<XdgToplevel>>,
        session: crate::protocol::session::Session,
        sender_object_id: u32,
        parent: u32, // object: 
    ) -> Box<futures::future::Future<Item = crate::protocol::session::Session, Error = ()> + Send> {
        Box::new(futures::future::ok(session))
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
        request: Arc<RwLock<XdgToplevel>>,
        session: crate::protocol::session::Session,
        sender_object_id: u32,
        title: String, // string: 
    ) -> Box<futures::future::Future<Item = crate::protocol::session::Session, Error = ()> + Send> {
        Box::new(futures::future::ok(session))
    }

    // show the window menu
    //
    // Clients implementing client-side decorations might want to show
    // a context menu when right-clicking on the decorations, giving the
    // user a menu that they can use to maximize or minimize the window.
    // 
    // This request asks the compositor to pop up such a window menu at
    // the given position, relative to the local surface coordinates of
    // the parent surface. There are no guarantees as to what menu items
    // the window menu contains.
    // 
    // This request must be used in response to some sort of user action
    // like a button press, key press, or touch down event.
    pub fn show_window_menu(
        request: Arc<RwLock<XdgToplevel>>,
        session: crate::protocol::session::Session,
        sender_object_id: u32,
        seat: u32, // object: the wl_seat of the user event
        serial: u32, // uint: the serial of the user event
        x: i32, // int: the x position to pop up the window menu at
        y: i32, // int: the y position to pop up the window menu at
    ) -> Box<futures::future::Future<Item = crate::protocol::session::Session, Error = ()> + Send> {
        Box::new(futures::future::ok(session))
    }

    // unset the window as fullscreen
    //
    // Make the surface no longer fullscreen.
    // 
    // After requesting that the surface should be unfullscreened, the
    // compositor will respond by emitting a configure event.
    // Whether this actually removes the fullscreen state of the client is
    // subject to compositor policies.
    // 
    // Making a surface unfullscreen sets states for the surface based on the following:
    // * the state(s) it may have had before becoming fullscreen
    // * any state(s) decided by the compositor
    // * any state(s) requested by the client while the surface was fullscreen
    // 
    // The compositor may include the previous window geometry dimensions in
    // the configure event, if applicable.
    // 
    // The client must also acknowledge the configure when committing the new
    // content (see ack_configure).
    pub fn unset_fullscreen(
        request: Arc<RwLock<XdgToplevel>>,
        session: crate::protocol::session::Session,
        sender_object_id: u32,
    ) -> Box<futures::future::Future<Item = crate::protocol::session::Session, Error = ()> + Send> {
        Box::new(futures::future::ok(session))
    }

    // unmaximize the window
    //
    // Unmaximize the surface.
    // 
    // After requesting that the surface should be unmaximized, the compositor
    // will respond by emitting a configure event. Whether this actually
    // un-maximizes the window is subject to compositor policies.
    // If available and applicable, the compositor will include the window
    // geometry dimensions the window had prior to being maximized in the
    // configure event. The client must then update its content, drawing it in
    // the configured state. The client must also acknowledge the configure
    // when committing the new content (see ack_configure).
    // 
    // It is up to the compositor to position the surface after it was
    // unmaximized; usually the position the surface had before maximizing, if
    // applicable.
    // 
    // If the surface was already not maximized, the compositor will still
    // emit a configure event without the "maximized" state.
    // 
    // If the surface is in a fullscreen state, this request has no direct
    // effect. It may alter the state the surface is returned to when
    // unmaximized unless overridden by the compositor.
    pub fn unset_maximized(
        request: Arc<RwLock<XdgToplevel>>,
        session: crate::protocol::session::Session,
        sender_object_id: u32,
    ) -> Box<futures::future::Future<Item = crate::protocol::session::Session, Error = ()> + Send> {
        Box::new(futures::future::ok(session))
    }
}

impl Into<crate::protocol::resource::Resource> for XdgToplevel {
    fn into(self) -> crate::protocol::resource::Resource {
        crate::protocol::resource::Resource::XdgToplevel(Arc::new(RwLock::new(self)))
    }
}
