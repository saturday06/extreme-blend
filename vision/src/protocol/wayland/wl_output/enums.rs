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


// mode information
//
// These flags describe properties of an output mode.
// They are used in the flags bitfield of the mode event.
#[allow(dead_code)]
pub enum Mode {
    Current = 0x1, // indicates this is the current mode
    Preferred = 0x2, // indicates this is the preferred mode
}

// subpixel geometry information
//
// This enumeration describes how the physical
// pixels on an output are laid out.
#[allow(dead_code)]
pub enum Subpixel {
    Unknown = 0, // unknown geometry
    None = 1, // no geometry
    HorizontalRgb = 2, // horizontal RGB
    HorizontalBgr = 3, // horizontal BGR
    VerticalRgb = 4, // vertical RGB
    VerticalBgr = 5, // vertical BGR
}

// transform from framebuffer to output
//
// This describes the transform that a compositor will apply to a
// surface to compensate for the rotation or mirroring of an
// output device.
// 
// The flipped values correspond to an initial flip around a
// vertical axis followed by rotation.
// 
// The purpose is mainly to allow clients to render accordingly and
// tell the compositor, so that for fullscreen surfaces, the
// compositor will still be able to scan out directly from client
// surfaces.
#[allow(dead_code)]
pub enum Transform {
    TransformNormal = 0, // no transform
    Transform90 = 1, // 90 degrees counter-clockwise
    Transform180 = 2, // 180 degrees counter-clockwise
    Transform270 = 3, // 270 degrees counter-clockwise
    TransformFlipped = 4, // 180 degree flip around a vertical axis
    TransformFlipped90 = 5, // flip and rotate 90 degrees counter-clockwise
    TransformFlipped180 = 6, // flip and rotate 180 degrees counter-clockwise
    TransformFlipped270 = 7, // flip and rotate 270 degrees counter-clockwise
}
