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


// edge values for resizing
//
// These values are used to indicate which edge of a surface
// is being dragged in a resize operation.
#[allow(dead_code)]
pub enum ResizeEdge {
    None = 0, // 
    Top = 1, // 
    Bottom = 2, // 
    Left = 4, // 
    TopLeft = 5, // 
    BottomLeft = 6, // 
    Right = 8, // 
    TopRight = 9, // 
    BottomRight = 10, // 
}

// types of state on the surface
//
// The different state values used on the surface. This is designed for
// state values like maximized, fullscreen. It is paired with the
// configure event to ensure that both the client and the compositor
// setting the state can be synchronized.
// 
// States set in this way are double-buffered. They will get applied on
// the next commit.
#[allow(dead_code)]
pub enum State {
    Maximized = 1, // the surface is maximized
    Fullscreen = 2, // the surface is fullscreen
    Resizing = 3, // the surface is being resized
    Activated = 4, // the surface is now activated
    TiledLeft = 5, // 
    TiledRight = 6, // 
    TiledTop = 7, // 
    TiledBottom = 8, // 
}
