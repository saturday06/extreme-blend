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

pub mod events {
    use byteorder::{ByteOrder, NativeEndian};

    // introduce a new wl_data_offer
    //
    // The data_offer event introduces a new wl_data_offer object,
    // which will subsequently be used in either the
    // data_device.enter event (for drag-and-drop) or the
    // data_device.selection event (for selections).  Immediately
    // following the data_device_data_offer event, the new data_offer
    // object will send out data_offer.offer events to describe the
    // mime types it offers.
    pub struct DataOffer {
        pub sender_object_id: u32,
        pub id: u32, // new_id: the new data_offer object
    }

    impl super::super::super::event::Event for DataOffer {
        fn encode(&self, dst: &mut bytes::BytesMut) -> Result<(), std::io::Error> {
            let total_len = 8 + 4;
            if total_len > 0xffff {
                return Err(std::io::Error::new(std::io::ErrorKind::Other, "Oops!"));
            }

            let i = dst.len();
            dst.resize(i + total_len, 0);

            NativeEndian::write_u32(&mut dst[i..], self.sender_object_id);
            NativeEndian::write_u32(&mut dst[i + 4..], ((total_len << 16) | 0) as u32);

            NativeEndian::write_u32(&mut dst[i + 8..], self.id);
            Ok(())
        }
    }

    // end drag-and-drop session successfully
    //
    // The event is sent when a drag-and-drop operation is ended
    // because the implicit grab is removed.
    // 
    // The drag-and-drop destination is expected to honor the last action
    // received through wl_data_offer.action, if the resulting action is
    // "copy" or "move", the destination can still perform
    // wl_data_offer.receive requests, and is expected to end all
    // transfers with a wl_data_offer.finish request.
    // 
    // If the resulting action is "ask", the action will not be considered
    // final. The drag-and-drop destination is expected to perform one last
    // wl_data_offer.set_actions request, or wl_data_offer.destroy in order
    // to cancel the operation.
    pub struct Drop {
        pub sender_object_id: u32,
    }

    impl super::super::super::event::Event for Drop {
        fn encode(&self, dst: &mut bytes::BytesMut) -> Result<(), std::io::Error> {
            let total_len = 8;
            if total_len > 0xffff {
                return Err(std::io::Error::new(std::io::ErrorKind::Other, "Oops!"));
            }

            let i = dst.len();
            dst.resize(i + total_len, 0);

            NativeEndian::write_u32(&mut dst[i..], self.sender_object_id);
            NativeEndian::write_u32(&mut dst[i + 4..], ((total_len << 16) | 4) as u32);

            Ok(())
        }
    }

    // initiate drag-and-drop session
    //
    // This event is sent when an active drag-and-drop pointer enters
    // a surface owned by the client.  The position of the pointer at
    // enter time is provided by the x and y arguments, in surface-local
    // coordinates.
    pub struct Enter {
        pub sender_object_id: u32,
        pub serial: u32, // uint: serial number of the enter event
        pub surface: u32, // object: client surface entered
        pub x: u32, // fixed: surface-local x coordinate
        pub y: u32, // fixed: surface-local y coordinate
        pub id: u32, // object: source data_offer object
    }

    impl super::super::super::event::Event for Enter {
        fn encode(&self, dst: &mut bytes::BytesMut) -> Result<(), std::io::Error> {
            let total_len = 8 + 4 + 4 + 4 + 4 + 4;
            if total_len > 0xffff {
                return Err(std::io::Error::new(std::io::ErrorKind::Other, "Oops!"));
            }

            let i = dst.len();
            dst.resize(i + total_len, 0);

            NativeEndian::write_u32(&mut dst[i..], self.sender_object_id);
            NativeEndian::write_u32(&mut dst[i + 4..], ((total_len << 16) | 1) as u32);

            NativeEndian::write_u32(&mut dst[i + 8..], self.serial);
            NativeEndian::write_u32(&mut dst[i + 8 + 4..], self.surface);
            NativeEndian::write_u32(&mut dst[i + 8 + 4 + 4..], self.x);
            NativeEndian::write_u32(&mut dst[i + 8 + 4 + 4 + 4..], self.y);
            NativeEndian::write_u32(&mut dst[i + 8 + 4 + 4 + 4 + 4..], self.id);
            Ok(())
        }
    }

    // end drag-and-drop session
    //
    // This event is sent when the drag-and-drop pointer leaves the
    // surface and the session ends.  The client must destroy the
    // wl_data_offer introduced at enter time at this point.
    pub struct Leave {
        pub sender_object_id: u32,
    }

    impl super::super::super::event::Event for Leave {
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

    // drag-and-drop session motion
    //
    // This event is sent when the drag-and-drop pointer moves within
    // the currently focused surface. The new position of the pointer
    // is provided by the x and y arguments, in surface-local
    // coordinates.
    pub struct Motion {
        pub sender_object_id: u32,
        pub time: u32, // uint: timestamp with millisecond granularity
        pub x: u32, // fixed: surface-local x coordinate
        pub y: u32, // fixed: surface-local y coordinate
    }

    impl super::super::super::event::Event for Motion {
        fn encode(&self, dst: &mut bytes::BytesMut) -> Result<(), std::io::Error> {
            let total_len = 8 + 4 + 4 + 4;
            if total_len > 0xffff {
                return Err(std::io::Error::new(std::io::ErrorKind::Other, "Oops!"));
            }

            let i = dst.len();
            dst.resize(i + total_len, 0);

            NativeEndian::write_u32(&mut dst[i..], self.sender_object_id);
            NativeEndian::write_u32(&mut dst[i + 4..], ((total_len << 16) | 3) as u32);

            NativeEndian::write_u32(&mut dst[i + 8..], self.time);
            NativeEndian::write_u32(&mut dst[i + 8 + 4..], self.x);
            NativeEndian::write_u32(&mut dst[i + 8 + 4 + 4..], self.y);
            Ok(())
        }
    }

    // advertise new selection
    //
    // The selection event is sent out to notify the client of a new
    // wl_data_offer for the selection for this device.  The
    // data_device.data_offer and the data_offer.offer events are
    // sent out immediately before this event to introduce the data
    // offer object.  The selection event is sent to a client
    // immediately before receiving keyboard focus and when a new
    // selection is set while the client has keyboard focus.  The
    // data_offer is valid until a new data_offer or NULL is received
    // or until the client loses keyboard focus.  The client must
    // destroy the previous selection data_offer, if any, upon receiving
    // this event.
    pub struct Selection {
        pub sender_object_id: u32,
        pub id: u32, // object: selection data_offer object
    }

    impl super::super::super::event::Event for Selection {
        fn encode(&self, dst: &mut bytes::BytesMut) -> Result<(), std::io::Error> {
            let total_len = 8 + 4;
            if total_len > 0xffff {
                return Err(std::io::Error::new(std::io::ErrorKind::Other, "Oops!"));
            }

            let i = dst.len();
            dst.resize(i + total_len, 0);

            NativeEndian::write_u32(&mut dst[i..], self.sender_object_id);
            NativeEndian::write_u32(&mut dst[i + 4..], ((total_len << 16) | 5) as u32);

            NativeEndian::write_u32(&mut dst[i + 8..], self.id);
            Ok(())
        }
    }
}

pub fn dispatch_request(request: Arc<RwLock<WlDataDevice>>, session: RwLock<super::super::session::Session>, tx: tokio::sync::mpsc::Sender<Box<super::super::event::Event + Send>>, sender_object_id: u32, opcode: u16, args: Vec<u8>) -> Box<futures::future::Future<Item = (), Error = ()> + Send> {
    let mut cursor = Cursor::new(&args);
    match opcode {
        0 => {
            let source = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
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
            let origin = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
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
            let icon = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
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
            return WlDataDevice::start_drag(request, session, tx, sender_object_id, source, origin, icon, serial)
        },
        1 => {
            let source = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
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
            return WlDataDevice::set_selection(request, session, tx, sender_object_id, source, serial)
        },
        2 => {
            return WlDataDevice::release(request, session, tx, sender_object_id, )
        },
        _ => {},
    };
    Box::new(futures::future::ok(()))
}

// data transfer device
//
// There is one wl_data_device per seat which can be obtained
// from the global wl_data_device_manager singleton.
// 
// A wl_data_device provides access to inter-client data transfer
// mechanisms such as copy-and-paste and drag-and-drop.
pub struct WlDataDevice {
}

impl WlDataDevice {
    // destroy data device
    //
    // This request destroys the data device.
    pub fn release(
        request: Arc<RwLock<WlDataDevice>>,
        session: RwLock<super::super::session::Session>,
        tx: tokio::sync::mpsc::Sender<Box<super::super::event::Event + Send>>,
        sender_object_id: u32,
    ) -> Box<futures::future::Future<Item = (), Error = ()> + Send> {
        Box::new(futures::future::ok(()))
    }

    // copy data to the selection
    //
    // This request asks the compositor to set the selection
    // to the data from the source on behalf of the client.
    // 
    // To unset the selection, set the source to NULL.
    pub fn set_selection(
        request: Arc<RwLock<WlDataDevice>>,
        session: RwLock<super::super::session::Session>,
        tx: tokio::sync::mpsc::Sender<Box<super::super::event::Event + Send>>,
        sender_object_id: u32,
        source: u32, // object: data source for the selection
        serial: u32, // uint: serial number of the event that triggered this request
    ) -> Box<futures::future::Future<Item = (), Error = ()> + Send> {
        Box::new(futures::future::ok(()))
    }

    // start drag-and-drop operation
    //
    // This request asks the compositor to start a drag-and-drop
    // operation on behalf of the client.
    // 
    // The source argument is the data source that provides the data
    // for the eventual data transfer. If source is NULL, enter, leave
    // and motion events are sent only to the client that initiated the
    // drag and the client is expected to handle the data passing
    // internally.
    // 
    // The origin surface is the surface where the drag originates and
    // the client must have an active implicit grab that matches the
    // serial.
    // 
    // The icon surface is an optional (can be NULL) surface that
    // provides an icon to be moved around with the cursor.  Initially,
    // the top-left corner of the icon surface is placed at the cursor
    // hotspot, but subsequent wl_surface.attach request can move the
    // relative position. Attach requests must be confirmed with
    // wl_surface.commit as usual. The icon surface is given the role of
    // a drag-and-drop icon. If the icon surface already has another role,
    // it raises a protocol error.
    // 
    // The current and pending input regions of the icon wl_surface are
    // cleared, and wl_surface.set_input_region is ignored until the
    // wl_surface is no longer used as the icon surface. When the use
    // as an icon ends, the current and pending input regions become
    // undefined, and the wl_surface is unmapped.
    pub fn start_drag(
        request: Arc<RwLock<WlDataDevice>>,
        session: RwLock<super::super::session::Session>,
        tx: tokio::sync::mpsc::Sender<Box<super::super::event::Event + Send>>,
        sender_object_id: u32,
        source: u32, // object: data source for the eventual transfer
        origin: u32, // object: surface where the drag originates
        icon: u32, // object: drag-and-drop icon surface
        serial: u32, // uint: serial number of the implicit grab on the origin
    ) -> Box<futures::future::Future<Item = (), Error = ()> + Send> {
        Box::new(futures::future::ok(()))
    }
}
