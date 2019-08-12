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
use crate::protocol::session::{Context, NextAction, Session};
#[allow(unused_imports)]
use futures::future::{err, ok, Future};
#[allow(unused_imports)]
use futures::sink::Sink;
#[allow(unused_imports)]
use std::sync::{Arc, RwLock};

pub mod enums;
pub mod events;
mod lib;
pub use lib::*;

// core global object
//
// The core global object.  This is a special singleton object.  It
// is used for internal Wayland protocol features.
pub struct WlDisplay {}

impl WlDisplay {
    // get global registry object
    //
    // This request creates a registry object that allows the client
    // to list and bind the global objects available from the
    // compositor.
    //
    // It should be noted that the server side resources consumed in
    // response to a get_registry request can only be released when the
    // client disconnects, not when the client side proxy is destroyed.
    // Therefore, clients should invoke get_registry as infrequently as
    // possible to avoid wasting memory.
    pub fn get_registry(
        mut context: Context<Arc<RwLock<WlDisplay>>>,
        registry: u32, // new_id: global registry object
    ) -> Box<Future<Item = (Session, NextAction), Error = ()> + Send> {
        println!("WlDisplay::get_registry({})", registry);
        context
            .resources
            .insert(registry, context.wl_registry.clone().into());

        Box::new(
            futures::future::ok(context.tx.clone())
                .and_then(move |tx| {
                    tx.send(Box::new(
                        crate::protocol::wayland::wl_registry::events::Global {
                            sender_object_id: registry,
                            name: crate::protocol::wayland::wl_compositor::GLOBAL_SINGLETON_NAME,
                            interface: "wl_compositor".to_owned(),
                            version: crate::protocol::wayland::wl_compositor::VERSION,
                        },
                    ))
                })
                .and_then(move |tx| {
                    tx.send(Box::new(
                        crate::protocol::wayland::wl_registry::events::Global {
                            sender_object_id: registry,
                            name: crate::protocol::wayland::wl_shm::GLOBAL_SINGLETON_NAME,
                            interface: "wl_shm".to_owned(),
                            version: crate::protocol::wayland::wl_shm::VERSION,
                        },
                    ))
                })
                .and_then(move |tx| {
                    tx.send(Box::new(
                        crate::protocol::wayland::wl_registry::events::Global {
                            sender_object_id: registry,
                            name: crate::protocol::xdg_shell::xdg_wm_base::GLOBAL_SINGLETON_NAME,
                            interface: "xdg_wm_base".to_owned(),
                            version: crate::protocol::xdg_shell::xdg_wm_base::VERSION,
                        },
                    ))
                })
                .map_err(|_| ())
                .and_then(|_| context.ok()),
        )
    }

    // asynchronous roundtrip
    //
    // The sync request asks the server to emit the 'done' event
    // on the returned wl_callback object.  Since requests are
    // handled in-order and events are delivered in-order, this can
    // be used as a barrier to ensure all previous requests and the
    // resulting events have been handled.
    //
    // The object returned by this request will be destroyed by the
    // compositor after the callback is fired and as such the client must not
    // attempt to use it after that point.
    //
    // The callback_data passed in the callback is the event serial.
    pub fn sync(
        mut context: Context<Arc<RwLock<WlDisplay>>>,
        callback: u32, // new_id: callback object for the sync request
    ) -> Box<Future<Item = (Session, NextAction), Error = ()> + Send> {
        println!("WlDisplay::sync({})", callback);
        context.callback_data += 1;
        let tx = context.tx.clone();
        Box::new(
            tx.send(Box::new(
                crate::protocol::wayland::wl_callback::events::Done {
                    sender_object_id: callback,
                    callback_data: context.callback_data,
                },
            ))
            .map_err(|_| ())
            .and_then(|_| context.ok()),
        )
    }
}
