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
// operation in response to destination side action changes through
// wl_data_offer.set_actions.
// 
// This event will no longer be emitted after wl_data_device.drop
// happened on the drag-and-drop destination, the client must
// honor the last action received, or the last preferred one set
// through wl_data_offer.set_actions when handling an "ask" action.
// 
// Compositors may also change the selected action on the fly, mainly
// in response to keyboard modifier changes during the drag-and-drop
// operation.
// 
// The most recent action received is always the valid one. Prior to
// receiving wl_data_device.drop, the chosen action may change (e.g.
// due to keyboard modifiers being pressed). At the time of receiving
// wl_data_device.drop the drag-and-drop destination must honor the
// last action received.
// 
// Action changes may still happen after wl_data_device.drop,
// especially on "ask" actions, where the drag-and-drop destination
// may choose another action afterwards. Action changes happening
// at this stage are always the result of inter-client negotiation, the
// compositor shall no longer be able to induce a different action.
// 
// Upon "ask" actions, it is expected that the drag-and-drop destination
// may potentially choose a different action and/or mime type,
// based on wl_data_offer.source_actions and finally chosen by the
// user (e.g. popping up a menu with the available options). The
// final wl_data_offer.set_actions and wl_data_offer.accept requests
// must happen before the call to wl_data_offer.finish.
#[allow(dead_code)]
pub struct Action {
    pub sender_object_id: u32,
    pub dnd_action: u32, // uint: action selected by the compositor
}

impl super::super::super::event::Event for Action {
    fn encode(&self, dst: &mut bytes::BytesMut) -> Result<(), std::io::Error> {
        let total_len = 8
 + 4;
        if total_len > 0xffff {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Oops!"));
        }

        let i = dst.len();
        dst.resize(i + total_len, 0);

        NativeEndian::write_u32(&mut dst[i..], self.sender_object_id);
        NativeEndian::write_u32(&mut dst[i + 4..], ((total_len << 16) | 2) as u32);

        NativeEndian::write_u32(&mut dst[i + 8..], self.dnd_action);
        Ok(())
    }
}

// advertise offered mime type
//
// Sent immediately after creating the wl_data_offer object.  One
// event per offered mime type.
#[allow(dead_code)]
pub struct Offer {
    pub sender_object_id: u32,
    pub mime_type: String, // string: offered mime type
}

impl super::super::super::event::Event for Offer {
    fn encode(&self, dst: &mut bytes::BytesMut) -> Result<(), std::io::Error> {
        let total_len = 8
 + (4 + (self.mime_type.len() + 1 + 3) / 4 * 4);
        if total_len > 0xffff {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Oops!"));
        }

        let i = dst.len();
        dst.resize(i + total_len, 0);

        NativeEndian::write_u32(&mut dst[i..], self.sender_object_id);
        NativeEndian::write_u32(&mut dst[i + 4..], ((total_len << 16) | 0) as u32);

                NativeEndian::write_u32(&mut dst[i + 8..], (self.mime_type.len() + 1) as u32);
        {
            let mut aligned = self.mime_type.clone();
            aligned.push(0u8.into());
            while aligned.len() % 4 != 0 {
                aligned.push(0u8.into());
            }
            dst[(i + 8 + 4)..(i + 8 + 4 + aligned.len())]
                .copy_from_slice(aligned.as_bytes());
        }

        Ok(())
    }
}

// notify the source-side available actions
//
// This event indicates the actions offered by the data source. It
// will be sent right after wl_data_device.enter, or anytime the source
// side changes its offered actions through wl_data_source.set_actions.
#[allow(dead_code)]
pub struct SourceActions {
    pub sender_object_id: u32,
    pub source_actions: u32, // uint: actions offered by the data source
}

impl super::super::super::event::Event for SourceActions {
    fn encode(&self, dst: &mut bytes::BytesMut) -> Result<(), std::io::Error> {
        let total_len = 8
 + 4;
        if total_len > 0xffff {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Oops!"));
        }

        let i = dst.len();
        dst.resize(i + total_len, 0);

        NativeEndian::write_u32(&mut dst[i..], self.sender_object_id);
        NativeEndian::write_u32(&mut dst[i + 4..], ((total_len << 16) | 1) as u32);

        NativeEndian::write_u32(&mut dst[i + 8..], self.source_actions);
        Ok(())
    }
}
