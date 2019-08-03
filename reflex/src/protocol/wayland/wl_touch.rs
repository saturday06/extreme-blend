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

pub mod events {
    use byteorder::{ByteOrder, NativeEndian};

    // touch session cancelled
    //
    // Sent if the compositor decides the touch stream is a global
    // gesture. No further events are sent to the clients from that
    // particular gesture. Touch cancellation applies to all touch points
    // currently active on this client's surface. The client is
    // responsible for finalizing the touch points, future touch points on
    // this surface may reuse the touch point ID.
    pub struct Cancel {
        pub sender_object_id: u32,
    }

    impl super::super::super::event::Event for Cancel {
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

    // touch down event and beginning of a touch sequence
    //
    // A new touch point has appeared on the surface. This touch point is
    // assigned a unique ID. Future events from this touch point reference
    // this ID. The ID ceases to be valid after a touch up event and may be
    // reused in the future.
    pub struct Down {
        pub sender_object_id: u32,
        pub serial: u32, // uint: serial number of the touch down event
        pub time: u32, // uint: timestamp with millisecond granularity
        pub surface: u32, // object: surface touched
        pub id: i32, // int: the unique ID of this touch point
        pub x: u32, // fixed: surface-local x coordinate
        pub y: u32, // fixed: surface-local y coordinate
    }

    impl super::super::super::event::Event for Down {
        fn encode(&self, dst: &mut bytes::BytesMut) -> Result<(), std::io::Error> {
            let total_len = 8 + 4 + 4 + 4 + 4 + 4 + 4;
            if total_len > 0xffff {
                return Err(std::io::Error::new(std::io::ErrorKind::Other, "Oops!"));
            }

            let i = dst.len();
            dst.resize(i + total_len, 0);

            NativeEndian::write_u32(&mut dst[i..], self.sender_object_id);
            NativeEndian::write_u32(&mut dst[i + 4..], ((total_len << 16) | 0) as u32);

            NativeEndian::write_u32(&mut dst[i + 8..], self.serial);
            NativeEndian::write_u32(&mut dst[i + 8 + 4..], self.time);
            NativeEndian::write_u32(&mut dst[i + 8 + 4 + 4..], self.surface);
            NativeEndian::write_i32(&mut dst[i + 8 + 4 + 4 + 4..], self.id);
            NativeEndian::write_u32(&mut dst[i + 8 + 4 + 4 + 4 + 4..], self.x);
            NativeEndian::write_u32(&mut dst[i + 8 + 4 + 4 + 4 + 4 + 4..], self.y);
            Ok(())
        }
    }

    // end of touch frame event
    //
    // Indicates the end of a set of events that logically belong together.
    // A client is expected to accumulate the data in all events within the
    // frame before proceeding.
    // 
    // A wl_touch.frame terminates at least one event but otherwise no
    // guarantee is provided about the set of events within a frame. A client
    // must assume that any state not updated in a frame is unchanged from the
    // previously known state.
    pub struct Frame {
        pub sender_object_id: u32,
    }

    impl super::super::super::event::Event for Frame {
        fn encode(&self, dst: &mut bytes::BytesMut) -> Result<(), std::io::Error> {
            let total_len = 8;
            if total_len > 0xffff {
                return Err(std::io::Error::new(std::io::ErrorKind::Other, "Oops!"));
            }

            let i = dst.len();
            dst.resize(i + total_len, 0);

            NativeEndian::write_u32(&mut dst[i..], self.sender_object_id);
            NativeEndian::write_u32(&mut dst[i + 4..], ((total_len << 16) | 3) as u32);

            Ok(())
        }
    }

    // update of touch point coordinates
    //
    // A touch point has changed coordinates.
    pub struct Motion {
        pub sender_object_id: u32,
        pub time: u32, // uint: timestamp with millisecond granularity
        pub id: i32, // int: the unique ID of this touch point
        pub x: u32, // fixed: surface-local x coordinate
        pub y: u32, // fixed: surface-local y coordinate
    }

    impl super::super::super::event::Event for Motion {
        fn encode(&self, dst: &mut bytes::BytesMut) -> Result<(), std::io::Error> {
            let total_len = 8 + 4 + 4 + 4 + 4;
            if total_len > 0xffff {
                return Err(std::io::Error::new(std::io::ErrorKind::Other, "Oops!"));
            }

            let i = dst.len();
            dst.resize(i + total_len, 0);

            NativeEndian::write_u32(&mut dst[i..], self.sender_object_id);
            NativeEndian::write_u32(&mut dst[i + 4..], ((total_len << 16) | 2) as u32);

            NativeEndian::write_u32(&mut dst[i + 8..], self.time);
            NativeEndian::write_i32(&mut dst[i + 8 + 4..], self.id);
            NativeEndian::write_u32(&mut dst[i + 8 + 4 + 4..], self.x);
            NativeEndian::write_u32(&mut dst[i + 8 + 4 + 4 + 4..], self.y);
            Ok(())
        }
    }

    // update orientation of touch point
    //
    // Sent when a touchpoint has changed its orientation.
    // 
    // This event does not occur on its own. It is sent before a
    // wl_touch.frame event and carries the new shape information for
    // any previously reported, or new touch points of that frame.
    // 
    // Other events describing the touch point such as wl_touch.down,
    // wl_touch.motion or wl_touch.shape may be sent within the
    // same wl_touch.frame. A client should treat these events as a single
    // logical touch point update. The order of wl_touch.shape,
    // wl_touch.orientation and wl_touch.motion is not guaranteed.
    // A wl_touch.down event is guaranteed to occur before the first
    // wl_touch.orientation event for this touch ID but both events may occur
    // within the same wl_touch.frame.
    // 
    // The orientation describes the clockwise angle of a touchpoint's major
    // axis to the positive surface y-axis and is normalized to the -180 to
    // +180 degree range. The granularity of orientation depends on the touch
    // device, some devices only support binary rotation values between 0 and
    // 90 degrees.
    // 
    // This event is only sent by the compositor if the touch device supports
    // orientation reports.
    pub struct Orientation {
        pub sender_object_id: u32,
        pub id: i32, // int: the unique ID of this touch point
        pub orientation: u32, // fixed: angle between major axis and positive surface y-axis in degrees
    }

    impl super::super::super::event::Event for Orientation {
        fn encode(&self, dst: &mut bytes::BytesMut) -> Result<(), std::io::Error> {
            let total_len = 8 + 4 + 4;
            if total_len > 0xffff {
                return Err(std::io::Error::new(std::io::ErrorKind::Other, "Oops!"));
            }

            let i = dst.len();
            dst.resize(i + total_len, 0);

            NativeEndian::write_u32(&mut dst[i..], self.sender_object_id);
            NativeEndian::write_u32(&mut dst[i + 4..], ((total_len << 16) | 6) as u32);

            NativeEndian::write_i32(&mut dst[i + 8..], self.id);
            NativeEndian::write_u32(&mut dst[i + 8 + 4..], self.orientation);
            Ok(())
        }
    }

    // update shape of touch point
    //
    // Sent when a touchpoint has changed its shape.
    // 
    // This event does not occur on its own. It is sent before a
    // wl_touch.frame event and carries the new shape information for
    // any previously reported, or new touch points of that frame.
    // 
    // Other events describing the touch point such as wl_touch.down,
    // wl_touch.motion or wl_touch.orientation may be sent within the
    // same wl_touch.frame. A client should treat these events as a single
    // logical touch point update. The order of wl_touch.shape,
    // wl_touch.orientation and wl_touch.motion is not guaranteed.
    // A wl_touch.down event is guaranteed to occur before the first
    // wl_touch.shape event for this touch ID but both events may occur within
    // the same wl_touch.frame.
    // 
    // A touchpoint shape is approximated by an ellipse through the major and
    // minor axis length. The major axis length describes the longer diameter
    // of the ellipse, while the minor axis length describes the shorter
    // diameter. Major and minor are orthogonal and both are specified in
    // surface-local coordinates. The center of the ellipse is always at the
    // touchpoint location as reported by wl_touch.down or wl_touch.move.
    // 
    // This event is only sent by the compositor if the touch device supports
    // shape reports. The client has to make reasonable assumptions about the
    // shape if it did not receive this event.
    pub struct Shape {
        pub sender_object_id: u32,
        pub id: i32, // int: the unique ID of this touch point
        pub major: u32, // fixed: length of the major axis in surface-local coordinates
        pub minor: u32, // fixed: length of the minor axis in surface-local coordinates
    }

    impl super::super::super::event::Event for Shape {
        fn encode(&self, dst: &mut bytes::BytesMut) -> Result<(), std::io::Error> {
            let total_len = 8 + 4 + 4 + 4;
            if total_len > 0xffff {
                return Err(std::io::Error::new(std::io::ErrorKind::Other, "Oops!"));
            }

            let i = dst.len();
            dst.resize(i + total_len, 0);

            NativeEndian::write_u32(&mut dst[i..], self.sender_object_id);
            NativeEndian::write_u32(&mut dst[i + 4..], ((total_len << 16) | 5) as u32);

            NativeEndian::write_i32(&mut dst[i + 8..], self.id);
            NativeEndian::write_u32(&mut dst[i + 8 + 4..], self.major);
            NativeEndian::write_u32(&mut dst[i + 8 + 4 + 4..], self.minor);
            Ok(())
        }
    }

    // end of a touch event sequence
    //
    // The touch point has disappeared. No further events will be sent for
    // this touch point and the touch point's ID is released and may be
    // reused in a future touch down event.
    pub struct Up {
        pub sender_object_id: u32,
        pub serial: u32, // uint: serial number of the touch up event
        pub time: u32, // uint: timestamp with millisecond granularity
        pub id: i32, // int: the unique ID of this touch point
    }

    impl super::super::super::event::Event for Up {
        fn encode(&self, dst: &mut bytes::BytesMut) -> Result<(), std::io::Error> {
            let total_len = 8 + 4 + 4 + 4;
            if total_len > 0xffff {
                return Err(std::io::Error::new(std::io::ErrorKind::Other, "Oops!"));
            }

            let i = dst.len();
            dst.resize(i + total_len, 0);

            NativeEndian::write_u32(&mut dst[i..], self.sender_object_id);
            NativeEndian::write_u32(&mut dst[i + 4..], ((total_len << 16) | 1) as u32);

            NativeEndian::write_u32(&mut dst[i + 8..], self.serial);
            NativeEndian::write_u32(&mut dst[i + 8 + 4..], self.time);
            NativeEndian::write_i32(&mut dst[i + 8 + 4 + 4..], self.id);
            Ok(())
        }
    }
}

pub fn dispatch_request(request: Arc<RwLock<WlTouch>>, session: RwLock<super::super::session::Session>, tx: tokio::sync::mpsc::Sender<Box<super::super::event::Event + Send>>, sender_object_id: u32, opcode: u16, args: Vec<u8>) -> Box<futures::future::Future<Item = (), Error = ()>> {
    let mut cursor = Cursor::new(&args);
    match opcode {
        0 => {
            return WlTouch::release(request, session, tx, sender_object_id, )
        },
        _ => {},
    };
    Box::new(futures::future::ok(()))
}

// touchscreen input device
//
// The wl_touch interface represents a touchscreen
// associated with a seat.
// 
// Touch interactions can consist of one or more contacts.
// For each contact, a series of events is generated, starting
// with a down event, followed by zero or more motion events,
// and ending with an up event. Events relating to the same
// contact point can be identified by the ID of the sequence.
pub struct WlTouch {
}

impl WlTouch {
    // release the touch object
    pub fn release(
        request: Arc<RwLock<WlTouch>>,
        session: RwLock<super::super::session::Session>,
        tx: tokio::sync::mpsc::Sender<Box<super::super::event::Event + Send>>,
        sender_object_id: u32,
    ) -> Box<futures::future::Future<Item = (), Error = ()>> {
        Box::new(futures::future::ok(()))
    }
}
