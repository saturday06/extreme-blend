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

// axis types
//
// Describes the axis types of scroll events.
pub enum Axis {
    VerticalScroll = 0,   // vertical axis
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
    Wheel = 0,      // a physical wheel rotation
    Finger = 1,     // finger on a touch surface
    Continuous = 2, // continuous coordinate space
    WheelTilt = 3,  // a physical wheel tilt
}

// physical button state
//
// Describes the physical state of a button that produced the button
// event.
pub enum ButtonState {
    Released = 0, // the button is not pressed
    Pressed = 1,  // the button is pressed
}

pub enum Error {
    Role = 0, // given wl_surface has another role
}
