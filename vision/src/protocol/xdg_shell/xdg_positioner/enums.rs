// Copyright © 2008-2013 Kristian Høgsberg
// Copyright © 2013      Rafael Antognolli
// Copyright © 2013      Jasper St. Pierre
// Copyright © 2010-2013 Intel Corporation
// Copyright © 2015-2017 Samsung Electronics Co., Ltd
// Copyright © 2015-2017 Red Hat Inc.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice (including the next
// paragraph) shall be included in all copies or substantial portions of the
// Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

#[allow(dead_code)]
pub enum Anchor {
    None = 0,        //
    Top = 1,         //
    Bottom = 2,      //
    Left = 3,        //
    Right = 4,       //
    TopLeft = 5,     //
    BottomLeft = 6,  //
    TopRight = 7,    //
    BottomRight = 8, //
}

// constraint adjustments
//
// The constraint adjustment value define ways the compositor will adjust
// the position of the surface, if the unadjusted position would result
// in the surface being partly constrained.
//
// Whether a surface is considered 'constrained' is left to the compositor
// to determine. For example, the surface may be partly outside the
// compositor's defined 'work area', thus necessitating the child surface's
// position be adjusted until it is entirely inside the work area.
//
// The adjustments can be combined, according to a defined precedence: 1)
// Flip, 2) Slide, 3) Resize.
#[allow(dead_code)]
pub enum ConstraintAdjustment {
    None = 0,     //
    SlideX = 1,   //
    SlideY = 2,   //
    FlipX = 4,    //
    FlipY = 8,    //
    ResizeX = 16, //
    ResizeY = 32, //
}

#[allow(dead_code)]
pub enum Error {
    InvalidInput = 0, // invalid input provided
}

#[allow(dead_code)]
pub enum Gravity {
    None = 0,        //
    Top = 1,         //
    Bottom = 2,      //
    Left = 3,        //
    Right = 4,       //
    TopLeft = 5,     //
    BottomLeft = 6,  //
    TopRight = 7,    //
    BottomRight = 8, //
}
