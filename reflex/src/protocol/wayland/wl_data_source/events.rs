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

// notify the selected action
//
// This event indicates the action selected by the compositor after
// matching the source/destination side actions. Only one action (or
// none) will be offered here.
// 
// This event can be emitted multiple times during the drag-and-drop
// operation, mainly in response to destination side changes through
// wl_data_offer.set_actions, and as the data device enters/leaves
// surfaces.
// 
// It is only possible to receive this event after
// wl_data_source.dnd_drop_performed if the drag-and-drop operation
// ended in an "ask" action, in which case the final wl_data_source.action
// event will happen immediately before wl_data_source.dnd_finished.
// 
// Compositors may also change the selected action on the fly, mainly
// in response to keyboard modifier changes during the drag-and-drop
// operation.
// 
// The most recent action received is always the valid one. The chosen
// action may change alongside negotiation (e.g. an "ask" action can turn
// into a "move" operation), so the effects of the final action must
// always be applied in wl_data_offer.dnd_finished.
// 
// Clients can trigger cursor surface changes from this point, so
// they reflect the current action.
pub struct Action {
    pub sender_object_id: u32,
    pub dnd_action: u32, // uint: action selected by the compositor
}

impl super::super::super::event::Event for Action {
    fn encode(&self, dst: &mut bytes::BytesMut) -> Result<(), std::io::Error> {
        let total_len = 8 + 4;
        if total_len > 0xffff {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Oops!"));
        }

        let i = dst.len();
        dst.resize(i + total_len, 0);

        NativeEndian::write_u32(&mut dst[i..], self.sender_object_id);
        NativeEndian::write_u32(&mut dst[i + 4..], ((total_len << 16) | 5) as u32);

        NativeEndian::write_u32(&mut dst[i + 8..], self.dnd_action);
        Ok(())
    }
}

// selection was cancelled
//
// This data source is no longer valid. There are several reasons why
// this could happen:
// 
// - The data source has been replaced by another data source.
// - The drag-and-drop operation was performed, but the drop destination
//   did not accept any of the mime types offered through
//   wl_data_source.target.
// - The drag-and-drop operation was performed, but the drop destination
//   did not select any of the actions present in the mask offered through
//   wl_data_source.action.
// - The drag-and-drop operation was performed but didn't happen over a
//   surface.
// - The compositor cancelled the drag-and-drop operation (e.g. compositor
//   dependent timeouts to avoid stale drag-and-drop transfers).
// 
// The client should clean up and destroy this data source.
// 
// For objects of version 2 or older, wl_data_source.cancelled will
// only be emitted if the data source was replaced by another data
// source.
pub struct Cancelled {
    pub sender_object_id: u32,
}

impl super::super::super::event::Event for Cancelled {
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

// the drag-and-drop operation physically finished
//
// The user performed the drop action. This event does not indicate
// acceptance, wl_data_source.cancelled may still be emitted afterwards
// if the drop destination does not accept any mime type.
// 
// However, this event might however not be received if the compositor
// cancelled the drag-and-drop operation before this event could happen.
// 
// Note that the data_source may still be used in the future and should
// not be destroyed here.
pub struct DndDropPerformed {
    pub sender_object_id: u32,
}

impl super::super::super::event::Event for DndDropPerformed {
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

// the drag-and-drop operation concluded
//
// The drop destination finished interoperating with this data
// source, so the client is now free to destroy this data source and
// free all associated data.
// 
// If the action used to perform the operation was "move", the
// source can now delete the transferred data.
pub struct DndFinished {
    pub sender_object_id: u32,
}

impl super::super::super::event::Event for DndFinished {
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

// send the data
//
// Request for data from the client.  Send the data as the
// specified mime type over the passed file descriptor, then
// close it.
pub struct Send {
    pub sender_object_id: u32,
    pub mime_type: String, // string: mime type for the data
    pub fd: i32, // fd: file descriptor for the data
}

impl super::super::super::event::Event for Send {
    fn encode(&self, dst: &mut bytes::BytesMut) -> Result<(), std::io::Error> {
        let total_len = 8 + (4 + (self.mime_type.len() + 1 + 3) / 4 * 4) + 4;
        if total_len > 0xffff {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Oops!"));
        }

        let i = dst.len();
        dst.resize(i + total_len, 0);

        NativeEndian::write_u32(&mut dst[i..], self.sender_object_id);
        NativeEndian::write_u32(&mut dst[i + 4..], ((total_len << 16) | 1) as u32);

        
            NativeEndian::write_u32(&mut dst[i + 8..], self.mime_type.len() as u32);
            let mut aligned_mime_type = self.mime_type.clone();
            aligned_mime_type.push(0u8.into());
            while aligned_mime_type.len() % 4 != 0 {
                aligned_mime_type.push(0u8.into());
            }
            dst[(i + 8 + 4)..(i + 8 + 4 + aligned_mime_type.len())].copy_from_slice(aligned_mime_type.as_bytes());

        NativeEndian::write_i32(&mut dst[i + 8 + (4 + (self.mime_type.len() + 1 + 3) / 4 * 4)..], self.fd);
        Ok(())
    }
}

// a target accepts an offered mime type
//
// Sent when a target accepts pointer_focus or motion events.  If
// a target does not accept any of the offered types, type is NULL.
// 
// Used for feedback during drag-and-drop.
pub struct Target {
    pub sender_object_id: u32,
    pub mime_type: String, // string: mime type accepted by the target
}

impl super::super::super::event::Event for Target {
    fn encode(&self, dst: &mut bytes::BytesMut) -> Result<(), std::io::Error> {
        let total_len = 8 + (4 + (self.mime_type.len() + 1 + 3) / 4 * 4);
        if total_len > 0xffff {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Oops!"));
        }

        let i = dst.len();
        dst.resize(i + total_len, 0);

        NativeEndian::write_u32(&mut dst[i..], self.sender_object_id);
        NativeEndian::write_u32(&mut dst[i + 4..], ((total_len << 16) | 0) as u32);

        
            NativeEndian::write_u32(&mut dst[i + 8..], self.mime_type.len() as u32);
            let mut aligned_mime_type = self.mime_type.clone();
            aligned_mime_type.push(0u8.into());
            while aligned_mime_type.len() % 4 != 0 {
                aligned_mime_type.push(0u8.into());
            }
            dst[(i + 8 + 4)..(i + 8 + 4 + aligned_mime_type.len())].copy_from_slice(aligned_mime_type.as_bytes());

        Ok(())
    }
}
