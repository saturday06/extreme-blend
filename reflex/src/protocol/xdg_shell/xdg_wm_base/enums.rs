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


pub enum Error {
    Role = 0, // given wl_surface has another role
    DefunctSurfaces = 1, // xdg_wm_base was destroyed before children
    NotTheTopmostPopup = 2, // the client tried to map or destroy a non-topmost popup
    InvalidPopupParent = 3, // the client specified an invalid popup parent surface
    InvalidSurfaceState = 4, // the client provided an invalid surface state
    InvalidPositioner = 5, // the client provided an invalid positioner
}
