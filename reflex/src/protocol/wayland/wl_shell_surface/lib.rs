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
use byteorder::{ByteOrder, NativeEndian, ReadBytesExt};
#[allow(unused_imports)]
use futures::future::Future;
#[allow(unused_imports)]
use futures::sink::Sink;
#[allow(unused_imports)]
use std::convert::TryInto;
#[allow(unused_imports)]
use std::io::{Cursor, Read};

#[allow(unused_variables)]
pub fn dispatch_request(
    request: crate::protocol::session::Context<super::WlShellSurface>,
    opcode: u16,
    args: Vec<u8>,
) -> Box<futures::future::Future<Item = crate::protocol::session::Session, Error = ()> + Send> {
    let mut cursor = Cursor::new(&args);
    match opcode {
        0 => {
            let serial = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
                x
            } else {
                let tx = request.tx.clone();
                return Box::new(
                    tx.send(Box::new(
                        crate::protocol::wayland::wl_display::events::Error {
                            sender_object_id: 1,
                            object_id: request.sender_object_id,
                            code: crate::protocol::wayland::wl_display::enums::Error::InvalidMethod
                                as u32,
                            message: format!(
                                "wl_shell_surface@{} opcode={} args={:?} not found",
                                request.sender_object_id, opcode, args
                            ),
                        },
                    ))
                    .map_err(|_| ())
                    .map(|_tx| request.into()),
                );
            };

            if Ok(cursor.position()) != args.len().try_into() {
                let tx = request.tx.clone();
                return Box::new(
                    tx.send(Box::new(
                        crate::protocol::wayland::wl_display::events::Error {
                            sender_object_id: 1,
                            object_id: request.sender_object_id,
                            code: crate::protocol::wayland::wl_display::enums::Error::InvalidMethod
                                as u32,
                            message: format!(
                                "wl_shell_surface@{} opcode={} args={:?} not found",
                                request.sender_object_id, opcode, args
                            ),
                        },
                    ))
                    .map_err(|_| ())
                    .map(|_tx| request.into()),
                );
            }
            return super::WlShellSurface::pong(request, serial);
        }
        1 => {
            let seat = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
                x
            } else {
                let tx = request.tx.clone();
                return Box::new(
                    tx.send(Box::new(
                        crate::protocol::wayland::wl_display::events::Error {
                            sender_object_id: 1,
                            object_id: request.sender_object_id,
                            code: crate::protocol::wayland::wl_display::enums::Error::InvalidMethod
                                as u32,
                            message: format!(
                                "wl_shell_surface@{} opcode={} args={:?} not found",
                                request.sender_object_id, opcode, args
                            ),
                        },
                    ))
                    .map_err(|_| ())
                    .map(|_tx| request.into()),
                );
            };
            let serial = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
                x
            } else {
                let tx = request.tx.clone();
                return Box::new(
                    tx.send(Box::new(
                        crate::protocol::wayland::wl_display::events::Error {
                            sender_object_id: 1,
                            object_id: request.sender_object_id,
                            code: crate::protocol::wayland::wl_display::enums::Error::InvalidMethod
                                as u32,
                            message: format!(
                                "wl_shell_surface@{} opcode={} args={:?} not found",
                                request.sender_object_id, opcode, args
                            ),
                        },
                    ))
                    .map_err(|_| ())
                    .map(|_tx| request.into()),
                );
            };

            if Ok(cursor.position()) != args.len().try_into() {
                let tx = request.tx.clone();
                return Box::new(
                    tx.send(Box::new(
                        crate::protocol::wayland::wl_display::events::Error {
                            sender_object_id: 1,
                            object_id: request.sender_object_id,
                            code: crate::protocol::wayland::wl_display::enums::Error::InvalidMethod
                                as u32,
                            message: format!(
                                "wl_shell_surface@{} opcode={} args={:?} not found",
                                request.sender_object_id, opcode, args
                            ),
                        },
                    ))
                    .map_err(|_| ())
                    .map(|_tx| request.into()),
                );
            }
            return super::WlShellSurface::move_fn(request, seat, serial);
        }
        2 => {
            let seat = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
                x
            } else {
                let tx = request.tx.clone();
                return Box::new(
                    tx.send(Box::new(
                        crate::protocol::wayland::wl_display::events::Error {
                            sender_object_id: 1,
                            object_id: request.sender_object_id,
                            code: crate::protocol::wayland::wl_display::enums::Error::InvalidMethod
                                as u32,
                            message: format!(
                                "wl_shell_surface@{} opcode={} args={:?} not found",
                                request.sender_object_id, opcode, args
                            ),
                        },
                    ))
                    .map_err(|_| ())
                    .map(|_tx| request.into()),
                );
            };
            let serial = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
                x
            } else {
                let tx = request.tx.clone();
                return Box::new(
                    tx.send(Box::new(
                        crate::protocol::wayland::wl_display::events::Error {
                            sender_object_id: 1,
                            object_id: request.sender_object_id,
                            code: crate::protocol::wayland::wl_display::enums::Error::InvalidMethod
                                as u32,
                            message: format!(
                                "wl_shell_surface@{} opcode={} args={:?} not found",
                                request.sender_object_id, opcode, args
                            ),
                        },
                    ))
                    .map_err(|_| ())
                    .map(|_tx| request.into()),
                );
            };
            let edges = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
                x
            } else {
                let tx = request.tx.clone();
                return Box::new(
                    tx.send(Box::new(
                        crate::protocol::wayland::wl_display::events::Error {
                            sender_object_id: 1,
                            object_id: request.sender_object_id,
                            code: crate::protocol::wayland::wl_display::enums::Error::InvalidMethod
                                as u32,
                            message: format!(
                                "wl_shell_surface@{} opcode={} args={:?} not found",
                                request.sender_object_id, opcode, args
                            ),
                        },
                    ))
                    .map_err(|_| ())
                    .map(|_tx| request.into()),
                );
            };

            if Ok(cursor.position()) != args.len().try_into() {
                let tx = request.tx.clone();
                return Box::new(
                    tx.send(Box::new(
                        crate::protocol::wayland::wl_display::events::Error {
                            sender_object_id: 1,
                            object_id: request.sender_object_id,
                            code: crate::protocol::wayland::wl_display::enums::Error::InvalidMethod
                                as u32,
                            message: format!(
                                "wl_shell_surface@{} opcode={} args={:?} not found",
                                request.sender_object_id, opcode, args
                            ),
                        },
                    ))
                    .map_err(|_| ())
                    .map(|_tx| request.into()),
                );
            }
            return super::WlShellSurface::resize(request, seat, serial, edges);
        }
        3 => {
            if Ok(cursor.position()) != args.len().try_into() {
                let tx = request.tx.clone();
                return Box::new(
                    tx.send(Box::new(
                        crate::protocol::wayland::wl_display::events::Error {
                            sender_object_id: 1,
                            object_id: request.sender_object_id,
                            code: crate::protocol::wayland::wl_display::enums::Error::InvalidMethod
                                as u32,
                            message: format!(
                                "wl_shell_surface@{} opcode={} args={:?} not found",
                                request.sender_object_id, opcode, args
                            ),
                        },
                    ))
                    .map_err(|_| ())
                    .map(|_tx| request.into()),
                );
            }
            return super::WlShellSurface::set_toplevel(request);
        }
        4 => {
            let parent = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
                x
            } else {
                let tx = request.tx.clone();
                return Box::new(
                    tx.send(Box::new(
                        crate::protocol::wayland::wl_display::events::Error {
                            sender_object_id: 1,
                            object_id: request.sender_object_id,
                            code: crate::protocol::wayland::wl_display::enums::Error::InvalidMethod
                                as u32,
                            message: format!(
                                "wl_shell_surface@{} opcode={} args={:?} not found",
                                request.sender_object_id, opcode, args
                            ),
                        },
                    ))
                    .map_err(|_| ())
                    .map(|_tx| request.into()),
                );
            };
            let x = if let Ok(x) = cursor.read_i32::<NativeEndian>() {
                x
            } else {
                let tx = request.tx.clone();
                return Box::new(
                    tx.send(Box::new(
                        crate::protocol::wayland::wl_display::events::Error {
                            sender_object_id: 1,
                            object_id: request.sender_object_id,
                            code: crate::protocol::wayland::wl_display::enums::Error::InvalidMethod
                                as u32,
                            message: format!(
                                "wl_shell_surface@{} opcode={} args={:?} not found",
                                request.sender_object_id, opcode, args
                            ),
                        },
                    ))
                    .map_err(|_| ())
                    .map(|_tx| request.into()),
                );
            };
            let y = if let Ok(x) = cursor.read_i32::<NativeEndian>() {
                x
            } else {
                let tx = request.tx.clone();
                return Box::new(
                    tx.send(Box::new(
                        crate::protocol::wayland::wl_display::events::Error {
                            sender_object_id: 1,
                            object_id: request.sender_object_id,
                            code: crate::protocol::wayland::wl_display::enums::Error::InvalidMethod
                                as u32,
                            message: format!(
                                "wl_shell_surface@{} opcode={} args={:?} not found",
                                request.sender_object_id, opcode, args
                            ),
                        },
                    ))
                    .map_err(|_| ())
                    .map(|_tx| request.into()),
                );
            };
            let flags = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
                x
            } else {
                let tx = request.tx.clone();
                return Box::new(
                    tx.send(Box::new(
                        crate::protocol::wayland::wl_display::events::Error {
                            sender_object_id: 1,
                            object_id: request.sender_object_id,
                            code: crate::protocol::wayland::wl_display::enums::Error::InvalidMethod
                                as u32,
                            message: format!(
                                "wl_shell_surface@{} opcode={} args={:?} not found",
                                request.sender_object_id, opcode, args
                            ),
                        },
                    ))
                    .map_err(|_| ())
                    .map(|_tx| request.into()),
                );
            };

            if Ok(cursor.position()) != args.len().try_into() {
                let tx = request.tx.clone();
                return Box::new(
                    tx.send(Box::new(
                        crate::protocol::wayland::wl_display::events::Error {
                            sender_object_id: 1,
                            object_id: request.sender_object_id,
                            code: crate::protocol::wayland::wl_display::enums::Error::InvalidMethod
                                as u32,
                            message: format!(
                                "wl_shell_surface@{} opcode={} args={:?} not found",
                                request.sender_object_id, opcode, args
                            ),
                        },
                    ))
                    .map_err(|_| ())
                    .map(|_tx| request.into()),
                );
            }
            return super::WlShellSurface::set_transient(request, parent, x, y, flags);
        }
        5 => {
            let method = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
                x
            } else {
                let tx = request.tx.clone();
                return Box::new(
                    tx.send(Box::new(
                        crate::protocol::wayland::wl_display::events::Error {
                            sender_object_id: 1,
                            object_id: request.sender_object_id,
                            code: crate::protocol::wayland::wl_display::enums::Error::InvalidMethod
                                as u32,
                            message: format!(
                                "wl_shell_surface@{} opcode={} args={:?} not found",
                                request.sender_object_id, opcode, args
                            ),
                        },
                    ))
                    .map_err(|_| ())
                    .map(|_tx| request.into()),
                );
            };
            let framerate = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
                x
            } else {
                let tx = request.tx.clone();
                return Box::new(
                    tx.send(Box::new(
                        crate::protocol::wayland::wl_display::events::Error {
                            sender_object_id: 1,
                            object_id: request.sender_object_id,
                            code: crate::protocol::wayland::wl_display::enums::Error::InvalidMethod
                                as u32,
                            message: format!(
                                "wl_shell_surface@{} opcode={} args={:?} not found",
                                request.sender_object_id, opcode, args
                            ),
                        },
                    ))
                    .map_err(|_| ())
                    .map(|_tx| request.into()),
                );
            };
            let output = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
                x
            } else {
                let tx = request.tx.clone();
                return Box::new(
                    tx.send(Box::new(
                        crate::protocol::wayland::wl_display::events::Error {
                            sender_object_id: 1,
                            object_id: request.sender_object_id,
                            code: crate::protocol::wayland::wl_display::enums::Error::InvalidMethod
                                as u32,
                            message: format!(
                                "wl_shell_surface@{} opcode={} args={:?} not found",
                                request.sender_object_id, opcode, args
                            ),
                        },
                    ))
                    .map_err(|_| ())
                    .map(|_tx| request.into()),
                );
            };

            if Ok(cursor.position()) != args.len().try_into() {
                let tx = request.tx.clone();
                return Box::new(
                    tx.send(Box::new(
                        crate::protocol::wayland::wl_display::events::Error {
                            sender_object_id: 1,
                            object_id: request.sender_object_id,
                            code: crate::protocol::wayland::wl_display::enums::Error::InvalidMethod
                                as u32,
                            message: format!(
                                "wl_shell_surface@{} opcode={} args={:?} not found",
                                request.sender_object_id, opcode, args
                            ),
                        },
                    ))
                    .map_err(|_| ())
                    .map(|_tx| request.into()),
                );
            }
            return super::WlShellSurface::set_fullscreen(request, method, framerate, output);
        }
        6 => {
            let seat = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
                x
            } else {
                let tx = request.tx.clone();
                return Box::new(
                    tx.send(Box::new(
                        crate::protocol::wayland::wl_display::events::Error {
                            sender_object_id: 1,
                            object_id: request.sender_object_id,
                            code: crate::protocol::wayland::wl_display::enums::Error::InvalidMethod
                                as u32,
                            message: format!(
                                "wl_shell_surface@{} opcode={} args={:?} not found",
                                request.sender_object_id, opcode, args
                            ),
                        },
                    ))
                    .map_err(|_| ())
                    .map(|_tx| request.into()),
                );
            };
            let serial = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
                x
            } else {
                let tx = request.tx.clone();
                return Box::new(
                    tx.send(Box::new(
                        crate::protocol::wayland::wl_display::events::Error {
                            sender_object_id: 1,
                            object_id: request.sender_object_id,
                            code: crate::protocol::wayland::wl_display::enums::Error::InvalidMethod
                                as u32,
                            message: format!(
                                "wl_shell_surface@{} opcode={} args={:?} not found",
                                request.sender_object_id, opcode, args
                            ),
                        },
                    ))
                    .map_err(|_| ())
                    .map(|_tx| request.into()),
                );
            };
            let parent = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
                x
            } else {
                let tx = request.tx.clone();
                return Box::new(
                    tx.send(Box::new(
                        crate::protocol::wayland::wl_display::events::Error {
                            sender_object_id: 1,
                            object_id: request.sender_object_id,
                            code: crate::protocol::wayland::wl_display::enums::Error::InvalidMethod
                                as u32,
                            message: format!(
                                "wl_shell_surface@{} opcode={} args={:?} not found",
                                request.sender_object_id, opcode, args
                            ),
                        },
                    ))
                    .map_err(|_| ())
                    .map(|_tx| request.into()),
                );
            };
            let x = if let Ok(x) = cursor.read_i32::<NativeEndian>() {
                x
            } else {
                let tx = request.tx.clone();
                return Box::new(
                    tx.send(Box::new(
                        crate::protocol::wayland::wl_display::events::Error {
                            sender_object_id: 1,
                            object_id: request.sender_object_id,
                            code: crate::protocol::wayland::wl_display::enums::Error::InvalidMethod
                                as u32,
                            message: format!(
                                "wl_shell_surface@{} opcode={} args={:?} not found",
                                request.sender_object_id, opcode, args
                            ),
                        },
                    ))
                    .map_err(|_| ())
                    .map(|_tx| request.into()),
                );
            };
            let y = if let Ok(x) = cursor.read_i32::<NativeEndian>() {
                x
            } else {
                let tx = request.tx.clone();
                return Box::new(
                    tx.send(Box::new(
                        crate::protocol::wayland::wl_display::events::Error {
                            sender_object_id: 1,
                            object_id: request.sender_object_id,
                            code: crate::protocol::wayland::wl_display::enums::Error::InvalidMethod
                                as u32,
                            message: format!(
                                "wl_shell_surface@{} opcode={} args={:?} not found",
                                request.sender_object_id, opcode, args
                            ),
                        },
                    ))
                    .map_err(|_| ())
                    .map(|_tx| request.into()),
                );
            };
            let flags = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
                x
            } else {
                let tx = request.tx.clone();
                return Box::new(
                    tx.send(Box::new(
                        crate::protocol::wayland::wl_display::events::Error {
                            sender_object_id: 1,
                            object_id: request.sender_object_id,
                            code: crate::protocol::wayland::wl_display::enums::Error::InvalidMethod
                                as u32,
                            message: format!(
                                "wl_shell_surface@{} opcode={} args={:?} not found",
                                request.sender_object_id, opcode, args
                            ),
                        },
                    ))
                    .map_err(|_| ())
                    .map(|_tx| request.into()),
                );
            };

            if Ok(cursor.position()) != args.len().try_into() {
                let tx = request.tx.clone();
                return Box::new(
                    tx.send(Box::new(
                        crate::protocol::wayland::wl_display::events::Error {
                            sender_object_id: 1,
                            object_id: request.sender_object_id,
                            code: crate::protocol::wayland::wl_display::enums::Error::InvalidMethod
                                as u32,
                            message: format!(
                                "wl_shell_surface@{} opcode={} args={:?} not found",
                                request.sender_object_id, opcode, args
                            ),
                        },
                    ))
                    .map_err(|_| ())
                    .map(|_tx| request.into()),
                );
            }
            return super::WlShellSurface::set_popup(request, seat, serial, parent, x, y, flags);
        }
        7 => {
            let output = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
                x
            } else {
                let tx = request.tx.clone();
                return Box::new(
                    tx.send(Box::new(
                        crate::protocol::wayland::wl_display::events::Error {
                            sender_object_id: 1,
                            object_id: request.sender_object_id,
                            code: crate::protocol::wayland::wl_display::enums::Error::InvalidMethod
                                as u32,
                            message: format!(
                                "wl_shell_surface@{} opcode={} args={:?} not found",
                                request.sender_object_id, opcode, args
                            ),
                        },
                    ))
                    .map_err(|_| ())
                    .map(|_tx| request.into()),
                );
            };

            if Ok(cursor.position()) != args.len().try_into() {
                let tx = request.tx.clone();
                return Box::new(
                    tx.send(Box::new(
                        crate::protocol::wayland::wl_display::events::Error {
                            sender_object_id: 1,
                            object_id: request.sender_object_id,
                            code: crate::protocol::wayland::wl_display::enums::Error::InvalidMethod
                                as u32,
                            message: format!(
                                "wl_shell_surface@{} opcode={} args={:?} not found",
                                request.sender_object_id, opcode, args
                            ),
                        },
                    ))
                    .map_err(|_| ())
                    .map(|_tx| request.into()),
                );
            }
            return super::WlShellSurface::set_maximized(request, output);
        }
        8 => {
            let title = {
                let buf_len = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
                    x
                } else {
                    let tx = request.tx.clone();
                    return Box::new(tx.send(Box::new(crate::protocol::wayland::wl_display::events::Error {
                    sender_object_id: 1,
                    object_id: request.sender_object_id,
                    code: crate::protocol::wayland::wl_display::enums::Error::InvalidMethod as u32,
                    message: format!(
                        "wl_shell_surface@{} opcode={} args={:?} not found",
                        request.sender_object_id, opcode, args
                    ),
                })).map_err(|_| ()).map(|_tx| request.into()));
                };
                let padded_buf_len = (buf_len + 3) / 4 * 4;
                let mut buf = Vec::new();
                buf.resize(buf_len as usize, 0);
                if let Err(_) = cursor.read_exact(&mut buf) {
                    let tx = request.tx.clone();
                    return Box::new(tx.send(Box::new(crate::protocol::wayland::wl_display::events::Error {
                    sender_object_id: 1,
                    object_id: request.sender_object_id,
                    code: crate::protocol::wayland::wl_display::enums::Error::InvalidMethod as u32,
                    message: format!(
                        "wl_shell_surface@{} opcode={} args={:?} not found",
                        request.sender_object_id, opcode, args
                    ),
                })).map_err(|_| ()).map(|_tx| request.into()));
                }
                let s = if let Ok(x) = String::from_utf8(buf) {
                    x
                } else {
                    let tx = request.tx.clone();
                    return Box::new(tx.send(Box::new(crate::protocol::wayland::wl_display::events::Error {
                    sender_object_id: 1,
                    object_id: request.sender_object_id,
                    code: crate::protocol::wayland::wl_display::enums::Error::InvalidMethod as u32,
                    message: format!(
                        "wl_shell_surface@{} opcode={} args={:?} not found",
                        request.sender_object_id, opcode, args
                    ),
                })).map_err(|_| ()).map(|_tx| request.into()));
                };
                cursor.set_position(cursor.position() + (padded_buf_len - buf_len) as u64);
                s
            };

            if Ok(cursor.position()) != args.len().try_into() {
                let tx = request.tx.clone();
                return Box::new(
                    tx.send(Box::new(
                        crate::protocol::wayland::wl_display::events::Error {
                            sender_object_id: 1,
                            object_id: request.sender_object_id,
                            code: crate::protocol::wayland::wl_display::enums::Error::InvalidMethod
                                as u32,
                            message: format!(
                                "wl_shell_surface@{} opcode={} args={:?} not found",
                                request.sender_object_id, opcode, args
                            ),
                        },
                    ))
                    .map_err(|_| ())
                    .map(|_tx| request.into()),
                );
            }
            return super::WlShellSurface::set_title(request, title);
        }
        9 => {
            let class_ = {
                let buf_len = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
                    x
                } else {
                    let tx = request.tx.clone();
                    return Box::new(tx.send(Box::new(crate::protocol::wayland::wl_display::events::Error {
                    sender_object_id: 1,
                    object_id: request.sender_object_id,
                    code: crate::protocol::wayland::wl_display::enums::Error::InvalidMethod as u32,
                    message: format!(
                        "wl_shell_surface@{} opcode={} args={:?} not found",
                        request.sender_object_id, opcode, args
                    ),
                })).map_err(|_| ()).map(|_tx| request.into()));
                };
                let padded_buf_len = (buf_len + 3) / 4 * 4;
                let mut buf = Vec::new();
                buf.resize(buf_len as usize, 0);
                if let Err(_) = cursor.read_exact(&mut buf) {
                    let tx = request.tx.clone();
                    return Box::new(tx.send(Box::new(crate::protocol::wayland::wl_display::events::Error {
                    sender_object_id: 1,
                    object_id: request.sender_object_id,
                    code: crate::protocol::wayland::wl_display::enums::Error::InvalidMethod as u32,
                    message: format!(
                        "wl_shell_surface@{} opcode={} args={:?} not found",
                        request.sender_object_id, opcode, args
                    ),
                })).map_err(|_| ()).map(|_tx| request.into()));
                }
                let s = if let Ok(x) = String::from_utf8(buf) {
                    x
                } else {
                    let tx = request.tx.clone();
                    return Box::new(tx.send(Box::new(crate::protocol::wayland::wl_display::events::Error {
                    sender_object_id: 1,
                    object_id: request.sender_object_id,
                    code: crate::protocol::wayland::wl_display::enums::Error::InvalidMethod as u32,
                    message: format!(
                        "wl_shell_surface@{} opcode={} args={:?} not found",
                        request.sender_object_id, opcode, args
                    ),
                })).map_err(|_| ()).map(|_tx| request.into()));
                };
                cursor.set_position(cursor.position() + (padded_buf_len - buf_len) as u64);
                s
            };

            if Ok(cursor.position()) != args.len().try_into() {
                let tx = request.tx.clone();
                return Box::new(
                    tx.send(Box::new(
                        crate::protocol::wayland::wl_display::events::Error {
                            sender_object_id: 1,
                            object_id: request.sender_object_id,
                            code: crate::protocol::wayland::wl_display::enums::Error::InvalidMethod
                                as u32,
                            message: format!(
                                "wl_shell_surface@{} opcode={} args={:?} not found",
                                request.sender_object_id, opcode, args
                            ),
                        },
                    ))
                    .map_err(|_| ())
                    .map(|_tx| request.into()),
                );
            }
            return super::WlShellSurface::set_class(request, class_);
        }
        _ => {}
    };
    Box::new(futures::future::ok(request.into()))
}

impl Into<crate::protocol::resource::Resource> for super::WlShellSurface {
    fn into(self) -> crate::protocol::resource::Resource {
        crate::protocol::resource::Resource::WlShellSurface(self)
    }
}
