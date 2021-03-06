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

use byteorder::{ByteOrder, NativeEndian};

// touch session cancelled
//
// Sent if the compositor decides the touch stream is a global
// gesture. No further events are sent to the clients from that
// particular gesture. Touch cancellation applies to all touch points
// currently active on this client's surface. The client is
// responsible for finalizing the touch points, future touch points on
// this surface may reuse the touch point ID.
#[allow(dead_code)]
pub struct Cancel {
    pub sender_object_id: u32,
}

impl super::super::super::event::Event for Cancel {
    fn encode(&self, dst: &mut bytes::BytesMut) -> Result<(), std::io::Error> {
        let total_len = 8;
        if total_len > 0xffff {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Oops!"));
        }

        let mut encode_offset = dst.len();
        dst.resize(encode_offset + total_len, 0);

        NativeEndian::write_u32(&mut dst[encode_offset..], self.sender_object_id);
        let event_opcode = 4;
        NativeEndian::write_u32(
            &mut dst[encode_offset + 4..],
            ((total_len << 16) | event_opcode) as u32,
        );

        encode_offset += 8;
        let _ = encode_offset;
        Ok(())
    }
}

// touch down event and beginning of a touch sequence
//
// A new touch point has appeared on the surface. This touch point is
// assigned a unique ID. Future events from this touch point reference
// this ID. The ID ceases to be valid after a touch up event and may be
// reused in the future.
#[allow(dead_code)]
pub struct Down {
    pub sender_object_id: u32,
    pub serial: u32,  // uint: serial number of the touch down event
    pub time: u32,    // uint: timestamp with millisecond granularity
    pub surface: u32, // object: surface touched
    pub id: i32,      // int: the unique ID of this touch point
    pub x: u32,       // fixed: surface-local x coordinate
    pub y: u32,       // fixed: surface-local y coordinate
}

impl super::super::super::event::Event for Down {
    fn encode(&self, dst: &mut bytes::BytesMut) -> Result<(), std::io::Error> {
        let total_len = 8 + 4 + 4 + 4 + 4 + 4 + 4;
        if total_len > 0xffff {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Oops!"));
        }

        let mut encode_offset = dst.len();
        dst.resize(encode_offset + total_len, 0);

        NativeEndian::write_u32(&mut dst[encode_offset..], self.sender_object_id);
        let event_opcode = 0;
        NativeEndian::write_u32(
            &mut dst[encode_offset + 4..],
            ((total_len << 16) | event_opcode) as u32,
        );

        encode_offset += 8;
        NativeEndian::write_u32(&mut dst[encode_offset..], self.serial);
        encode_offset += 4;
        NativeEndian::write_u32(&mut dst[encode_offset..], self.time);
        encode_offset += 4;
        NativeEndian::write_u32(&mut dst[encode_offset..], self.surface);
        encode_offset += 4;
        NativeEndian::write_i32(&mut dst[encode_offset..], self.id);
        encode_offset += 4;
        NativeEndian::write_u32(&mut dst[encode_offset..], self.x);
        encode_offset += 4;
        NativeEndian::write_u32(&mut dst[encode_offset..], self.y);
        encode_offset += 4;
        let _ = encode_offset;
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
#[allow(dead_code)]
pub struct Frame {
    pub sender_object_id: u32,
}

impl super::super::super::event::Event for Frame {
    fn encode(&self, dst: &mut bytes::BytesMut) -> Result<(), std::io::Error> {
        let total_len = 8;
        if total_len > 0xffff {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Oops!"));
        }

        let mut encode_offset = dst.len();
        dst.resize(encode_offset + total_len, 0);

        NativeEndian::write_u32(&mut dst[encode_offset..], self.sender_object_id);
        let event_opcode = 3;
        NativeEndian::write_u32(
            &mut dst[encode_offset + 4..],
            ((total_len << 16) | event_opcode) as u32,
        );

        encode_offset += 8;
        let _ = encode_offset;
        Ok(())
    }
}

// update of touch point coordinates
//
// A touch point has changed coordinates.
#[allow(dead_code)]
pub struct Motion {
    pub sender_object_id: u32,
    pub time: u32, // uint: timestamp with millisecond granularity
    pub id: i32,   // int: the unique ID of this touch point
    pub x: u32,    // fixed: surface-local x coordinate
    pub y: u32,    // fixed: surface-local y coordinate
}

impl super::super::super::event::Event for Motion {
    fn encode(&self, dst: &mut bytes::BytesMut) -> Result<(), std::io::Error> {
        let total_len = 8 + 4 + 4 + 4 + 4;
        if total_len > 0xffff {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Oops!"));
        }

        let mut encode_offset = dst.len();
        dst.resize(encode_offset + total_len, 0);

        NativeEndian::write_u32(&mut dst[encode_offset..], self.sender_object_id);
        let event_opcode = 2;
        NativeEndian::write_u32(
            &mut dst[encode_offset + 4..],
            ((total_len << 16) | event_opcode) as u32,
        );

        encode_offset += 8;
        NativeEndian::write_u32(&mut dst[encode_offset..], self.time);
        encode_offset += 4;
        NativeEndian::write_i32(&mut dst[encode_offset..], self.id);
        encode_offset += 4;
        NativeEndian::write_u32(&mut dst[encode_offset..], self.x);
        encode_offset += 4;
        NativeEndian::write_u32(&mut dst[encode_offset..], self.y);
        encode_offset += 4;
        let _ = encode_offset;
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
#[allow(dead_code)]
pub struct Orientation {
    pub sender_object_id: u32,
    pub id: i32,          // int: the unique ID of this touch point
    pub orientation: u32, // fixed: angle between major axis and positive surface y-axis in degrees
}

impl super::super::super::event::Event for Orientation {
    fn encode(&self, dst: &mut bytes::BytesMut) -> Result<(), std::io::Error> {
        let total_len = 8 + 4 + 4;
        if total_len > 0xffff {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Oops!"));
        }

        let mut encode_offset = dst.len();
        dst.resize(encode_offset + total_len, 0);

        NativeEndian::write_u32(&mut dst[encode_offset..], self.sender_object_id);
        let event_opcode = 6;
        NativeEndian::write_u32(
            &mut dst[encode_offset + 4..],
            ((total_len << 16) | event_opcode) as u32,
        );

        encode_offset += 8;
        NativeEndian::write_i32(&mut dst[encode_offset..], self.id);
        encode_offset += 4;
        NativeEndian::write_u32(&mut dst[encode_offset..], self.orientation);
        encode_offset += 4;
        let _ = encode_offset;
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
#[allow(dead_code)]
pub struct Shape {
    pub sender_object_id: u32,
    pub id: i32,    // int: the unique ID of this touch point
    pub major: u32, // fixed: length of the major axis in surface-local coordinates
    pub minor: u32, // fixed: length of the minor axis in surface-local coordinates
}

impl super::super::super::event::Event for Shape {
    fn encode(&self, dst: &mut bytes::BytesMut) -> Result<(), std::io::Error> {
        let total_len = 8 + 4 + 4 + 4;
        if total_len > 0xffff {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Oops!"));
        }

        let mut encode_offset = dst.len();
        dst.resize(encode_offset + total_len, 0);

        NativeEndian::write_u32(&mut dst[encode_offset..], self.sender_object_id);
        let event_opcode = 5;
        NativeEndian::write_u32(
            &mut dst[encode_offset + 4..],
            ((total_len << 16) | event_opcode) as u32,
        );

        encode_offset += 8;
        NativeEndian::write_i32(&mut dst[encode_offset..], self.id);
        encode_offset += 4;
        NativeEndian::write_u32(&mut dst[encode_offset..], self.major);
        encode_offset += 4;
        NativeEndian::write_u32(&mut dst[encode_offset..], self.minor);
        encode_offset += 4;
        let _ = encode_offset;
        Ok(())
    }
}

// end of a touch event sequence
//
// The touch point has disappeared. No further events will be sent for
// this touch point and the touch point's ID is released and may be
// reused in a future touch down event.
#[allow(dead_code)]
pub struct Up {
    pub sender_object_id: u32,
    pub serial: u32, // uint: serial number of the touch up event
    pub time: u32,   // uint: timestamp with millisecond granularity
    pub id: i32,     // int: the unique ID of this touch point
}

impl super::super::super::event::Event for Up {
    fn encode(&self, dst: &mut bytes::BytesMut) -> Result<(), std::io::Error> {
        let total_len = 8 + 4 + 4 + 4;
        if total_len > 0xffff {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Oops!"));
        }

        let mut encode_offset = dst.len();
        dst.resize(encode_offset + total_len, 0);

        NativeEndian::write_u32(&mut dst[encode_offset..], self.sender_object_id);
        let event_opcode = 1;
        NativeEndian::write_u32(
            &mut dst[encode_offset + 4..],
            ((total_len << 16) | event_opcode) as u32,
        );

        encode_offset += 8;
        NativeEndian::write_u32(&mut dst[encode_offset..], self.serial);
        encode_offset += 4;
        NativeEndian::write_u32(&mut dst[encode_offset..], self.time);
        encode_offset += 4;
        NativeEndian::write_i32(&mut dst[encode_offset..], self.id);
        encode_offset += 4;
        let _ = encode_offset;
        Ok(())
    }
}
