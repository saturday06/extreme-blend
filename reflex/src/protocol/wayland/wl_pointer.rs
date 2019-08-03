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
    // axis types
    //
    // Describes the axis types of scroll events.
    pub enum Axis {
        VerticalScroll = 0, // vertical axis
        HorizontalScroll = 1, // horizontal axis
    }

    // axis source types
    //
    // Describes the source types for axis events. This indicates to the
    // client how an axis event was physically generated; a client may
    // adjust the user interface accordingly. For example, scroll events
    // from a "finger" source may be in a smooth coordinate space with
    // kinetic scrolling whereas a "wheel" source may be in discrete steps
    // of a number of lines.
    // 
    // The "continuous" axis source is a device generating events in a
    // continuous coordinate space, but using something other than a
    // finger. One example for this source is button-based scrolling where
    // the vertical motion of a device is converted to scroll events while
    // a button is held down.
    // 
    // The "wheel tilt" axis source indicates that the actual device is a
    // wheel but the scroll event is not caused by a rotation but a
    // (usually sideways) tilt of the wheel.
    pub enum AxisSource {
        Wheel = 0, // a physical wheel rotation
        Finger = 1, // finger on a touch surface
        Continuous = 2, // continuous coordinate space
        WheelTilt = 3, // a physical wheel tilt
    }

    // physical button state
    //
    // Describes the physical state of a button that produced the button
    // event.
    pub enum ButtonState {
        Released = 0, // the button is not pressed
        Pressed = 1, // the button is pressed
    }

    pub enum Error {
        Role = 0, // given wl_surface has another role
    }
}

pub mod events {
    use byteorder::{ByteOrder, NativeEndian};

    // axis event
    //
    // Scroll and other axis notifications.
    // 
    // For scroll events (vertical and horizontal scroll axes), the
    // value parameter is the length of a vector along the specified
    // axis in a coordinate space identical to those of motion events,
    // representing a relative movement along the specified axis.
    // 
    // For devices that support movements non-parallel to axes multiple
    // axis events will be emitted.
    // 
    // When applicable, for example for touch pads, the server can
    // choose to emit scroll events where the motion vector is
    // equivalent to a motion event vector.
    // 
    // When applicable, a client can transform its content relative to the
    // scroll distance.
    pub struct Axis {
        pub sender_object_id: u32,
        pub time: u32, // uint: timestamp with millisecond granularity
        pub axis: u32, // uint: axis type
        pub value: u32, // fixed: length of vector in surface-local coordinate space
    }

    impl super::super::super::event::Event for Axis {
        fn encode(&self, dst: &mut bytes::BytesMut) -> Result<(), std::io::Error> {
            let total_len = 8 + 4 + 4 + 4;
            if total_len > 0xffff {
                return Err(std::io::Error::new(std::io::ErrorKind::Other, "Oops!"));
            }

            let i = dst.len();
            dst.resize(i + total_len, 0);

            NativeEndian::write_u32(&mut dst[i..], self.sender_object_id);
            NativeEndian::write_u32(&mut dst[i + 4..], ((total_len << 16) | 4) as u32);

            NativeEndian::write_u32(&mut dst[i + 8..], self.time);
            NativeEndian::write_u32(&mut dst[i + 8 + 4..], self.axis);
            NativeEndian::write_u32(&mut dst[i + 8 + 4 + 4..], self.value);
            Ok(())
        }
    }

    // axis click event
    //
    // Discrete step information for scroll and other axes.
    // 
    // This event carries the axis value of the wl_pointer.axis event in
    // discrete steps (e.g. mouse wheel clicks).
    // 
    // This event does not occur on its own, it is coupled with a
    // wl_pointer.axis event that represents this axis value on a
    // continuous scale. The protocol guarantees that each axis_discrete
    // event is always followed by exactly one axis event with the same
    // axis number within the same wl_pointer.frame. Note that the protocol
    // allows for other events to occur between the axis_discrete and
    // its coupled axis event, including other axis_discrete or axis
    // events.
    // 
    // This event is optional; continuous scrolling devices
    // like two-finger scrolling on touchpads do not have discrete
    // steps and do not generate this event.
    // 
    // The discrete value carries the directional information. e.g. a value
    // of -2 is two steps towards the negative direction of this axis.
    // 
    // The axis number is identical to the axis number in the associated
    // axis event.
    // 
    // The order of wl_pointer.axis_discrete and wl_pointer.axis_source is
    // not guaranteed.
    pub struct AxisDiscrete {
        pub sender_object_id: u32,
        pub axis: u32, // uint: axis type
        pub discrete: i32, // int: number of steps
    }

    impl super::super::super::event::Event for AxisDiscrete {
        fn encode(&self, dst: &mut bytes::BytesMut) -> Result<(), std::io::Error> {
            let total_len = 8 + 4 + 4;
            if total_len > 0xffff {
                return Err(std::io::Error::new(std::io::ErrorKind::Other, "Oops!"));
            }

            let i = dst.len();
            dst.resize(i + total_len, 0);

            NativeEndian::write_u32(&mut dst[i..], self.sender_object_id);
            NativeEndian::write_u32(&mut dst[i + 4..], ((total_len << 16) | 8) as u32);

            NativeEndian::write_u32(&mut dst[i + 8..], self.axis);
            NativeEndian::write_i32(&mut dst[i + 8 + 4..], self.discrete);
            Ok(())
        }
    }

    // axis source event
    //
    // Source information for scroll and other axes.
    // 
    // This event does not occur on its own. It is sent before a
    // wl_pointer.frame event and carries the source information for
    // all events within that frame.
    // 
    // The source specifies how this event was generated. If the source is
    // wl_pointer.axis_source.finger, a wl_pointer.axis_stop event will be
    // sent when the user lifts the finger off the device.
    // 
    // If the source is wl_pointer.axis_source.wheel,
    // wl_pointer.axis_source.wheel_tilt or
    // wl_pointer.axis_source.continuous, a wl_pointer.axis_stop event may
    // or may not be sent. Whether a compositor sends an axis_stop event
    // for these sources is hardware-specific and implementation-dependent;
    // clients must not rely on receiving an axis_stop event for these
    // scroll sources and should treat scroll sequences from these scroll
    // sources as unterminated by default.
    // 
    // This event is optional. If the source is unknown for a particular
    // axis event sequence, no event is sent.
    // Only one wl_pointer.axis_source event is permitted per frame.
    // 
    // The order of wl_pointer.axis_discrete and wl_pointer.axis_source is
    // not guaranteed.
    pub struct AxisSource {
        pub sender_object_id: u32,
        pub axis_source: u32, // uint: source of the axis event
    }

    impl super::super::super::event::Event for AxisSource {
        fn encode(&self, dst: &mut bytes::BytesMut) -> Result<(), std::io::Error> {
            let total_len = 8 + 4;
            if total_len > 0xffff {
                return Err(std::io::Error::new(std::io::ErrorKind::Other, "Oops!"));
            }

            let i = dst.len();
            dst.resize(i + total_len, 0);

            NativeEndian::write_u32(&mut dst[i..], self.sender_object_id);
            NativeEndian::write_u32(&mut dst[i + 4..], ((total_len << 16) | 6) as u32);

            NativeEndian::write_u32(&mut dst[i + 8..], self.axis_source);
            Ok(())
        }
    }

    // axis stop event
    //
    // Stop notification for scroll and other axes.
    // 
    // For some wl_pointer.axis_source types, a wl_pointer.axis_stop event
    // is sent to notify a client that the axis sequence has terminated.
    // This enables the client to implement kinetic scrolling.
    // See the wl_pointer.axis_source documentation for information on when
    // this event may be generated.
    // 
    // Any wl_pointer.axis events with the same axis_source after this
    // event should be considered as the start of a new axis motion.
    // 
    // The timestamp is to be interpreted identical to the timestamp in the
    // wl_pointer.axis event. The timestamp value may be the same as a
    // preceding wl_pointer.axis event.
    pub struct AxisStop {
        pub sender_object_id: u32,
        pub time: u32, // uint: timestamp with millisecond granularity
        pub axis: u32, // uint: the axis stopped with this event
    }

    impl super::super::super::event::Event for AxisStop {
        fn encode(&self, dst: &mut bytes::BytesMut) -> Result<(), std::io::Error> {
            let total_len = 8 + 4 + 4;
            if total_len > 0xffff {
                return Err(std::io::Error::new(std::io::ErrorKind::Other, "Oops!"));
            }

            let i = dst.len();
            dst.resize(i + total_len, 0);

            NativeEndian::write_u32(&mut dst[i..], self.sender_object_id);
            NativeEndian::write_u32(&mut dst[i + 4..], ((total_len << 16) | 7) as u32);

            NativeEndian::write_u32(&mut dst[i + 8..], self.time);
            NativeEndian::write_u32(&mut dst[i + 8 + 4..], self.axis);
            Ok(())
        }
    }

    // pointer button event
    //
    // Mouse button click and release notifications.
    // 
    // The location of the click is given by the last motion or
    // enter event.
    // The time argument is a timestamp with millisecond
    // granularity, with an undefined base.
    // 
    // The button is a button code as defined in the Linux kernel's
    // linux/input-event-codes.h header file, e.g. BTN_LEFT.
    // 
    // Any 16-bit button code value is reserved for future additions to the
    // kernel's event code list. All other button codes above 0xFFFF are
    // currently undefined but may be used in future versions of this
    // protocol.
    pub struct Button {
        pub sender_object_id: u32,
        pub serial: u32, // uint: serial number of the button event
        pub time: u32, // uint: timestamp with millisecond granularity
        pub button: u32, // uint: button that produced the event
        pub state: u32, // uint: physical state of the button
    }

    impl super::super::super::event::Event for Button {
        fn encode(&self, dst: &mut bytes::BytesMut) -> Result<(), std::io::Error> {
            let total_len = 8 + 4 + 4 + 4 + 4;
            if total_len > 0xffff {
                return Err(std::io::Error::new(std::io::ErrorKind::Other, "Oops!"));
            }

            let i = dst.len();
            dst.resize(i + total_len, 0);

            NativeEndian::write_u32(&mut dst[i..], self.sender_object_id);
            NativeEndian::write_u32(&mut dst[i + 4..], ((total_len << 16) | 3) as u32);

            NativeEndian::write_u32(&mut dst[i + 8..], self.serial);
            NativeEndian::write_u32(&mut dst[i + 8 + 4..], self.time);
            NativeEndian::write_u32(&mut dst[i + 8 + 4 + 4..], self.button);
            NativeEndian::write_u32(&mut dst[i + 8 + 4 + 4 + 4..], self.state);
            Ok(())
        }
    }

    // enter event
    //
    // Notification that this seat's pointer is focused on a certain
    // surface.
    // 
    // When a seat's focus enters a surface, the pointer image
    // is undefined and a client should respond to this event by setting
    // an appropriate pointer image with the set_cursor request.
    pub struct Enter {
        pub sender_object_id: u32,
        pub serial: u32, // uint: serial number of the enter event
        pub surface: u32, // object: surface entered by the pointer
        pub surface_x: u32, // fixed: surface-local x coordinate
        pub surface_y: u32, // fixed: surface-local y coordinate
    }

    impl super::super::super::event::Event for Enter {
        fn encode(&self, dst: &mut bytes::BytesMut) -> Result<(), std::io::Error> {
            let total_len = 8 + 4 + 4 + 4 + 4;
            if total_len > 0xffff {
                return Err(std::io::Error::new(std::io::ErrorKind::Other, "Oops!"));
            }

            let i = dst.len();
            dst.resize(i + total_len, 0);

            NativeEndian::write_u32(&mut dst[i..], self.sender_object_id);
            NativeEndian::write_u32(&mut dst[i + 4..], ((total_len << 16) | 0) as u32);

            NativeEndian::write_u32(&mut dst[i + 8..], self.serial);
            NativeEndian::write_u32(&mut dst[i + 8 + 4..], self.surface);
            NativeEndian::write_u32(&mut dst[i + 8 + 4 + 4..], self.surface_x);
            NativeEndian::write_u32(&mut dst[i + 8 + 4 + 4 + 4..], self.surface_y);
            Ok(())
        }
    }

    // end of a pointer event sequence
    //
    // Indicates the end of a set of events that logically belong together.
    // A client is expected to accumulate the data in all events within the
    // frame before proceeding.
    // 
    // All wl_pointer events before a wl_pointer.frame event belong
    // logically together. For example, in a diagonal scroll motion the
    // compositor will send an optional wl_pointer.axis_source event, two
    // wl_pointer.axis events (horizontal and vertical) and finally a
    // wl_pointer.frame event. The client may use this information to
    // calculate a diagonal vector for scrolling.
    // 
    // When multiple wl_pointer.axis events occur within the same frame,
    // the motion vector is the combined motion of all events.
    // When a wl_pointer.axis and a wl_pointer.axis_stop event occur within
    // the same frame, this indicates that axis movement in one axis has
    // stopped but continues in the other axis.
    // When multiple wl_pointer.axis_stop events occur within the same
    // frame, this indicates that these axes stopped in the same instance.
    // 
    // A wl_pointer.frame event is sent for every logical event group,
    // even if the group only contains a single wl_pointer event.
    // Specifically, a client may get a sequence: motion, frame, button,
    // frame, axis, frame, axis_stop, frame.
    // 
    // The wl_pointer.enter and wl_pointer.leave events are logical events
    // generated by the compositor and not the hardware. These events are
    // also grouped by a wl_pointer.frame. When a pointer moves from one
    // surface to another, a compositor should group the
    // wl_pointer.leave event within the same wl_pointer.frame.
    // However, a client must not rely on wl_pointer.leave and
    // wl_pointer.enter being in the same wl_pointer.frame.
    // Compositor-specific policies may require the wl_pointer.leave and
    // wl_pointer.enter event being split across multiple wl_pointer.frame
    // groups.
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
            NativeEndian::write_u32(&mut dst[i + 4..], ((total_len << 16) | 5) as u32);

            Ok(())
        }
    }

    // leave event
    //
    // Notification that this seat's pointer is no longer focused on
    // a certain surface.
    // 
    // The leave notification is sent before the enter notification
    // for the new focus.
    pub struct Leave {
        pub sender_object_id: u32,
        pub serial: u32, // uint: serial number of the leave event
        pub surface: u32, // object: surface left by the pointer
    }

    impl super::super::super::event::Event for Leave {
        fn encode(&self, dst: &mut bytes::BytesMut) -> Result<(), std::io::Error> {
            let total_len = 8 + 4 + 4;
            if total_len > 0xffff {
                return Err(std::io::Error::new(std::io::ErrorKind::Other, "Oops!"));
            }

            let i = dst.len();
            dst.resize(i + total_len, 0);

            NativeEndian::write_u32(&mut dst[i..], self.sender_object_id);
            NativeEndian::write_u32(&mut dst[i + 4..], ((total_len << 16) | 1) as u32);

            NativeEndian::write_u32(&mut dst[i + 8..], self.serial);
            NativeEndian::write_u32(&mut dst[i + 8 + 4..], self.surface);
            Ok(())
        }
    }

    // pointer motion event
    //
    // Notification of pointer location change. The arguments
    // surface_x and surface_y are the location relative to the
    // focused surface.
    pub struct Motion {
        pub sender_object_id: u32,
        pub time: u32, // uint: timestamp with millisecond granularity
        pub surface_x: u32, // fixed: surface-local x coordinate
        pub surface_y: u32, // fixed: surface-local y coordinate
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
            NativeEndian::write_u32(&mut dst[i + 4..], ((total_len << 16) | 2) as u32);

            NativeEndian::write_u32(&mut dst[i + 8..], self.time);
            NativeEndian::write_u32(&mut dst[i + 8 + 4..], self.surface_x);
            NativeEndian::write_u32(&mut dst[i + 8 + 4 + 4..], self.surface_y);
            Ok(())
        }
    }
}

pub fn dispatch_request(request: Arc<RwLock<WlPointer>>, session: RwLock<super::super::session::Session>, tx: tokio::sync::mpsc::Sender<Box<super::super::event::Event + Send>>, sender_object_id: u32, opcode: u16, args: Vec<u8>) -> Box<futures::future::Future<Item = (), Error = ()>> {
    let mut cursor = Cursor::new(&args);
    match opcode {
        0 => {
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
            let surface = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
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
            let hotspot_x = if let Ok(x) = cursor.read_i32::<NativeEndian>() {
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
            let hotspot_y = if let Ok(x) = cursor.read_i32::<NativeEndian>() {
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
            return WlPointer::set_cursor(request, session, tx, sender_object_id, serial, surface, hotspot_x, hotspot_y)
        },
        1 => {
            return WlPointer::release(request, session, tx, sender_object_id, )
        },
        _ => {},
    };
    Box::new(futures::future::ok(()))
}

// pointer input device
//
// The wl_pointer interface represents one or more input devices,
// such as mice, which control the pointer location and pointer_focus
// of a seat.
// 
// The wl_pointer interface generates motion, enter and leave
// events for the surfaces that the pointer is located over,
// and button and axis events for button presses, button releases
// and scrolling.
pub struct WlPointer {
}

impl WlPointer {
    // release the pointer object
    //
    // Using this request a client can tell the server that it is not going to
    // use the pointer object anymore.
    // 
    // This request destroys the pointer proxy object, so clients must not call
    // wl_pointer_destroy() after using this request.
    pub fn release(
        request: Arc<RwLock<WlPointer>>,
        session: RwLock<super::super::session::Session>,
        tx: tokio::sync::mpsc::Sender<Box<super::super::event::Event + Send>>,
        sender_object_id: u32,
    ) -> Box<futures::future::Future<Item = (), Error = ()>> {
        Box::new(futures::future::ok(()))
    }

    // set the pointer surface
    //
    // Set the pointer surface, i.e., the surface that contains the
    // pointer image (cursor). This request gives the surface the role
    // of a cursor. If the surface already has another role, it raises
    // a protocol error.
    // 
    // The cursor actually changes only if the pointer
    // focus for this device is one of the requesting client's surfaces
    // or the surface parameter is the current pointer surface. If
    // there was a previous surface set with this request it is
    // replaced. If surface is NULL, the pointer image is hidden.
    // 
    // The parameters hotspot_x and hotspot_y define the position of
    // the pointer surface relative to the pointer location. Its
    // top-left corner is always at (x, y) - (hotspot_x, hotspot_y),
    // where (x, y) are the coordinates of the pointer location, in
    // surface-local coordinates.
    // 
    // On surface.attach requests to the pointer surface, hotspot_x
    // and hotspot_y are decremented by the x and y parameters
    // passed to the request. Attach must be confirmed by
    // wl_surface.commit as usual.
    // 
    // The hotspot can also be updated by passing the currently set
    // pointer surface to this request with new values for hotspot_x
    // and hotspot_y.
    // 
    // The current and pending input regions of the wl_surface are
    // cleared, and wl_surface.set_input_region is ignored until the
    // wl_surface is no longer used as the cursor. When the use as a
    // cursor ends, the current and pending input regions become
    // undefined, and the wl_surface is unmapped.
    pub fn set_cursor(
        request: Arc<RwLock<WlPointer>>,
        session: RwLock<super::super::session::Session>,
        tx: tokio::sync::mpsc::Sender<Box<super::super::event::Event + Send>>,
        sender_object_id: u32,
        serial: u32, // uint: serial number of the enter event
        surface: u32, // object: pointer surface
        hotspot_x: i32, // int: surface-local x coordinate
        hotspot_y: i32, // int: surface-local y coordinate
    ) -> Box<futures::future::Future<Item = (), Error = ()>> {
        Box::new(futures::future::ok(()))
    }
}
