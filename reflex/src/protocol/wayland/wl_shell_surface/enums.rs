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


// different method to set the surface fullscreen
//
// Hints to indicate to the compositor how to deal with a conflict
// between the dimensions of the surface and the dimensions of the
// output. The compositor is free to ignore this parameter.
pub enum FullscreenMethod {
    Default = 0, // no preference, apply default policy
    Scale = 1, // scale, preserve the surface's aspect ratio and center on output
    Driver = 2, // switch output mode to the smallest mode that can fit the surface, add black borders to compensate size mismatch
    Fill = 3, // no upscaling, center on output and add black borders to compensate size mismatch
}

// edge values for resizing
//
// These values are used to indicate which edge of a surface
// is being dragged in a resize operation. The server may
// use this information to adapt its behavior, e.g. choose
// an appropriate cursor image.
pub enum Resize {
    None = 0, // no edge
    Top = 1, // top edge
    Bottom = 2, // bottom edge
    Left = 4, // left edge
    TopLeft = 5, // top and left edges
    BottomLeft = 6, // bottom and left edges
    Right = 8, // right edge
    TopRight = 9, // top and right edges
    BottomRight = 10, // bottom and right edges
}

// details of transient behaviour
//
// These flags specify details of the expected behaviour
// of transient surfaces. Used in the set_transient request.
pub enum Transient {
    Inactive = 0x1, // do not set keyboard focus
}
