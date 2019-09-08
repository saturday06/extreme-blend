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

use crate::protocol::session::{Context, NextAction, Session};
use byteorder::{ByteOrder, NativeEndian, ReadBytesExt};
use futures::future::{ok, Future};
use futures::sink::Sink;
use std::convert::TryInto;
use std::io::{Cursor, Read};
use std::sync::{Arc, RwLock};

pub mod events;
mod lib;
pub use lib::{GLOBAL_SINGLETON_NAME, VERSION};

pub fn dispatch_request(
    context: crate::protocol::session::Context<
        Arc<RwLock<crate::protocol::wayland::wl_registry::WlRegistry>>,
    >,
    opcode: u16,
    args: Vec<u8>,
) -> Box<dyn futures::future::Future<Item = crate::protocol::session::Session, Error = ()> + Send> {
    if opcode != 0 || args.len() <= 8 {
        return lib::dispatch_request(context, opcode, args);
    }

    let mut cursor = Cursor::new(&args);
    let name = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
        x
    } else {
        return context
            .invalid_method_dispatch(format!("opcode={} args={:?} not found", opcode, &args));
    };
    let name_buf_len = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
        x as usize
    } else {
        return context
            .invalid_method_dispatch(format!("opcode={} args={:?} not found", opcode, &args));
    };
    let name_buf_len_with_pad = (name_buf_len + 3) / 4 * 4;
    let mut name_buf = Vec::new();
    name_buf.resize(name_buf_len, 0);
    cursor.read_exact(&mut name_buf).unwrap();
    cursor.set_position(cursor.position() + (name_buf_len_with_pad - name_buf_len) as u64);
    let _version = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
        x
    } else {
        return context
            .invalid_method_dispatch(format!("opcode={} args={:?} not found", opcode, &args));
    };

    let id = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
        x
    } else {
        return context
            .invalid_method_dispatch(format!("opcode={} args={:?} not found", opcode, &args));
    };

    if Ok(cursor.position()) != args.len().try_into() {
        return context
            .invalid_method_dispatch(format!("opcode={} args={:?} not found", opcode, &args));
    }

    let relay_buf = {
        let total_len = 8 + 4 + 4;
        if total_len > 0xffff {
            println!("Oops! total_len={}", total_len);
            return Box::new(futures::future::err(()));
        }

        let mut dst: Vec<u8> = Vec::new();
        dst.resize(total_len, 0);

        NativeEndian::write_u32(&mut dst[0..], context.sender_object_id);
        NativeEndian::write_u32(&mut dst[4..], (total_len << 16) as u32 | u32::from(opcode));

        #[allow(unused_mut)]
        let mut encode_offset = 8;

        NativeEndian::write_u32(&mut dst[encode_offset..], name);
        encode_offset += 4;
        NativeEndian::write_u32(&mut dst[encode_offset..], id);
        encode_offset += 4;
        let _ = encode_offset;
        dst
    };

    Box::new(WlRegistry::bind(context, name, id).and_then(
        |(session, next_action)| -> Box<
            dyn futures::future::Future<Item = crate::protocol::session::Session, Error = ()>
                + Send,
        > {
            match next_action {
                NextAction::Nop => Box::new(futures::future::ok(session)),
                NextAction::Relay => {
                    println!("[WlRegistry Relay]: {:?}", &relay_buf);
                    session.relay(relay_buf)
                }
                NextAction::RelayWait => session.relay_wait(relay_buf),
            }
        },
    ))
}

// global registry object
//
// The singleton global registry object.  The server has a number of
// global objects that are available to all clients.  These objects
// typically represent an actual object in the server (for example,
// an input device) or they are singleton objects that provide
// extension functionality.
//
// When a client creates a registry object, the registry object
// will emit a global event for each global currently in the
// registry.  Globals come and go as a result of device or
// monitor hotplugs, reconfiguration or other events, and the
// registry will send out global and global_remove events to
// keep the client up to date with the changes.  To mark the end
// of the initial burst of events, the client can use the
// wl_display.sync request immediately after calling
// wl_display.get_registry.
//
// A client can bind to a global object by using the bind
// request.  This creates a client-side handle that lets the object
// emit events to the client and lets the client invoke requests on
// the object.
pub struct WlRegistry {}

impl WlRegistry {
    // bind an object to the display
    //
    // Binds a new, client-created object to the server using the
    // specified name as the identifier.
    pub fn bind(
        mut context: Context<Arc<RwLock<WlRegistry>>>,
        name: u32, // uint: unique numeric name of the object
        id: u32,   // new_id: bounded object
    ) -> Box<dyn Future<Item = (Session, NextAction), Error = ()> + Send> {
        println!("WlRegistry::bind(name: {}, id: {})", name, id);

        match name {
            crate::protocol::wayland::wl_registry::GLOBAL_SINGLETON_NAME => {
                context
                    .resources
                    .insert(id, context.wl_registry.clone().into());
                return context.ok();
            }
            crate::protocol::wayland::wl_display::GLOBAL_SINGLETON_NAME => {
                context
                    .resources
                    .insert(id, context.wl_display.clone().into());
                return context.ok();
            }
            crate::protocol::wayland::wl_compositor::GLOBAL_SINGLETON_NAME => {
                context
                    .resources
                    .insert(id, context.wl_compositor.clone().into());
                return context.ok();
            }
            crate::protocol::wayland::wl_data_device_manager::GLOBAL_SINGLETON_NAME => {
                context
                    .resources
                    .insert(id, context.wl_data_device_manager.clone().into());
                return context.ok();
            }
            crate::protocol::xdg_shell::xdg_wm_base::GLOBAL_SINGLETON_NAME => {
                context
                    .resources
                    .insert(id, context.xdg_wm_base.clone().into());
                return context.ok();
            }
            crate::protocol::wayland::wl_shm::GLOBAL_SINGLETON_NAME => {
                context.resources.insert(id, context.wl_shm.clone().into());
                return context.ok();
            }
            _ => {}
        }

        context.ok()
    }
}
