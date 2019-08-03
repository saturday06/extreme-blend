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
    pub enum Error {
        Role = 0, // given wl_surface has another role
    }
}

pub fn dispatch_request(request: Arc<RwLock<WlShell>>, session: crate::protocol::session::Session, sender_object_id: u32, opcode: u16, args: Vec<u8>) -> Box<futures::future::Future<Item = crate::protocol::session::Session, Error = ()> + Send> {
    let mut cursor = Cursor::new(&args);
    match opcode {
        0 => {
            let id = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
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
            let surface = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
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
            return WlShell::get_shell_surface(request, session, sender_object_id, id, surface)
        },
        _ => {},
    };
    Box::new(futures::future::ok(session))
}

// create desktop-style surfaces
//
// This interface is implemented by servers that provide
// desktop-style user interfaces.
// 
// It allows clients to associate a wl_shell_surface with
// a basic surface.
// 
// Note! This protocol is deprecated and not intended for production use.
// For desktop-style user interfaces, use xdg_shell.
pub struct WlShell {
}

impl WlShell {
    // create a shell surface from a surface
    //
    // Create a shell surface for an existing surface. This gives
    // the wl_surface the role of a shell surface. If the wl_surface
    // already has another role, it raises a protocol error.
    // 
    // Only one shell surface can be associated with a given surface.
    pub fn get_shell_surface(
        request: Arc<RwLock<WlShell>>,
        session: crate::protocol::session::Session,
        sender_object_id: u32,
        id: u32, // new_id: shell surface to create
        surface: u32, // object: surface to be given the shell surface role
    ) -> Box<futures::future::Future<Item = crate::protocol::session::Session, Error = ()> + Send> {
        Box::new(futures::future::ok(session))
    }
}

impl Into<crate::protocol::resource::Resource> for WlShell {
    fn into(self) -> crate::protocol::resource::Resource {
        crate::protocol::resource::Resource::WlShell(Arc::new(RwLock::new(self)))
    }
}
