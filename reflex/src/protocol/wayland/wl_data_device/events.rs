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
