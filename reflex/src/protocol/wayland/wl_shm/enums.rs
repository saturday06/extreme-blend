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

// wl_shm error values
//
// These errors can be emitted in response to wl_shm requests.
pub enum Error {
    InvalidFormat = 0, // buffer format is not known
    InvalidStride = 1, // invalid size or stride during pool or buffer creation
    InvalidFd = 2,     // mmapping the file descriptor failed
}

// pixel formats
//
// This describes the memory layout of an individual pixel.
//
// All renderers should support argb8888 and xrgb8888 but any other
// formats are optional and may not be supported by the particular
// renderer in use.
//
// The drm format codes match the macros defined in drm_fourcc.h.
// The formats actually supported by the compositor will be
// reported by the format event.
pub enum Format {
    Argb8888 = 0,             // 32-bit ARGB format, [31:0] A:R:G:B 8:8:8:8 little endian
    Xrgb8888 = 1,             // 32-bit RGB format, [31:0] x:R:G:B 8:8:8:8 little endian
    C8 = 0x20203843,          // 8-bit color index format, [7:0] C
    Rgb332 = 0x38424752,      // 8-bit RGB format, [7:0] R:G:B 3:3:2
    Bgr233 = 0x38524742,      // 8-bit BGR format, [7:0] B:G:R 2:3:3
    Xrgb4444 = 0x32315258,    // 16-bit xRGB format, [15:0] x:R:G:B 4:4:4:4 little endian
    Xbgr4444 = 0x32314258,    // 16-bit xBGR format, [15:0] x:B:G:R 4:4:4:4 little endian
    Rgbx4444 = 0x32315852,    // 16-bit RGBx format, [15:0] R:G:B:x 4:4:4:4 little endian
    Bgrx4444 = 0x32315842,    // 16-bit BGRx format, [15:0] B:G:R:x 4:4:4:4 little endian
    Argb4444 = 0x32315241,    // 16-bit ARGB format, [15:0] A:R:G:B 4:4:4:4 little endian
    Abgr4444 = 0x32314241,    // 16-bit ABGR format, [15:0] A:B:G:R 4:4:4:4 little endian
    Rgba4444 = 0x32314152,    // 16-bit RBGA format, [15:0] R:G:B:A 4:4:4:4 little endian
    Bgra4444 = 0x32314142,    // 16-bit BGRA format, [15:0] B:G:R:A 4:4:4:4 little endian
    Xrgb1555 = 0x35315258,    // 16-bit xRGB format, [15:0] x:R:G:B 1:5:5:5 little endian
    Xbgr1555 = 0x35314258,    // 16-bit xBGR 1555 format, [15:0] x:B:G:R 1:5:5:5 little endian
    Rgbx5551 = 0x35315852,    // 16-bit RGBx 5551 format, [15:0] R:G:B:x 5:5:5:1 little endian
    Bgrx5551 = 0x35315842,    // 16-bit BGRx 5551 format, [15:0] B:G:R:x 5:5:5:1 little endian
    Argb1555 = 0x35315241,    // 16-bit ARGB 1555 format, [15:0] A:R:G:B 1:5:5:5 little endian
    Abgr1555 = 0x35314241,    // 16-bit ABGR 1555 format, [15:0] A:B:G:R 1:5:5:5 little endian
    Rgba5551 = 0x35314152,    // 16-bit RGBA 5551 format, [15:0] R:G:B:A 5:5:5:1 little endian
    Bgra5551 = 0x35314142,    // 16-bit BGRA 5551 format, [15:0] B:G:R:A 5:5:5:1 little endian
    Rgb565 = 0x36314752,      // 16-bit RGB 565 format, [15:0] R:G:B 5:6:5 little endian
    Bgr565 = 0x36314742,      // 16-bit BGR 565 format, [15:0] B:G:R 5:6:5 little endian
    Rgb888 = 0x34324752,      // 24-bit RGB format, [23:0] R:G:B little endian
    Bgr888 = 0x34324742,      // 24-bit BGR format, [23:0] B:G:R little endian
    Xbgr8888 = 0x34324258,    // 32-bit xBGR format, [31:0] x:B:G:R 8:8:8:8 little endian
    Rgbx8888 = 0x34325852,    // 32-bit RGBx format, [31:0] R:G:B:x 8:8:8:8 little endian
    Bgrx8888 = 0x34325842,    // 32-bit BGRx format, [31:0] B:G:R:x 8:8:8:8 little endian
    Abgr8888 = 0x34324241,    // 32-bit ABGR format, [31:0] A:B:G:R 8:8:8:8 little endian
    Rgba8888 = 0x34324152,    // 32-bit RGBA format, [31:0] R:G:B:A 8:8:8:8 little endian
    Bgra8888 = 0x34324142,    // 32-bit BGRA format, [31:0] B:G:R:A 8:8:8:8 little endian
    Xrgb2101010 = 0x30335258, // 32-bit xRGB format, [31:0] x:R:G:B 2:10:10:10 little endian
    Xbgr2101010 = 0x30334258, // 32-bit xBGR format, [31:0] x:B:G:R 2:10:10:10 little endian
    Rgbx1010102 = 0x30335852, // 32-bit RGBx format, [31:0] R:G:B:x 10:10:10:2 little endian
    Bgrx1010102 = 0x30335842, // 32-bit BGRx format, [31:0] B:G:R:x 10:10:10:2 little endian
    Argb2101010 = 0x30335241, // 32-bit ARGB format, [31:0] A:R:G:B 2:10:10:10 little endian
    Abgr2101010 = 0x30334241, // 32-bit ABGR format, [31:0] A:B:G:R 2:10:10:10 little endian
    Rgba1010102 = 0x30334152, // 32-bit RGBA format, [31:0] R:G:B:A 10:10:10:2 little endian
    Bgra1010102 = 0x30334142, // 32-bit BGRA format, [31:0] B:G:R:A 10:10:10:2 little endian
    Yuyv = 0x56595559,        // packed YCbCr format, [31:0] Cr0:Y1:Cb0:Y0 8:8:8:8 little endian
    Yvyu = 0x55595659,        // packed YCbCr format, [31:0] Cb0:Y1:Cr0:Y0 8:8:8:8 little endian
    Uyvy = 0x59565955,        // packed YCbCr format, [31:0] Y1:Cr0:Y0:Cb0 8:8:8:8 little endian
    Vyuy = 0x59555956,        // packed YCbCr format, [31:0] Y1:Cb0:Y0:Cr0 8:8:8:8 little endian
    Ayuv = 0x56555941,        // packed AYCbCr format, [31:0] A:Y:Cb:Cr 8:8:8:8 little endian
    Nv12 = 0x3231564e,        // 2 plane YCbCr Cr:Cb format, 2x2 subsampled Cr:Cb plane
    Nv21 = 0x3132564e,        // 2 plane YCbCr Cb:Cr format, 2x2 subsampled Cb:Cr plane
    Nv16 = 0x3631564e,        // 2 plane YCbCr Cr:Cb format, 2x1 subsampled Cr:Cb plane
    Nv61 = 0x3136564e,        // 2 plane YCbCr Cb:Cr format, 2x1 subsampled Cb:Cr plane
    Yuv410 = 0x39565559,      // 3 plane YCbCr format, 4x4 subsampled Cb (1) and Cr (2) planes
    Yvu410 = 0x39555659,      // 3 plane YCbCr format, 4x4 subsampled Cr (1) and Cb (2) planes
    Yuv411 = 0x31315559,      // 3 plane YCbCr format, 4x1 subsampled Cb (1) and Cr (2) planes
    Yvu411 = 0x31315659,      // 3 plane YCbCr format, 4x1 subsampled Cr (1) and Cb (2) planes
    Yuv420 = 0x32315559,      // 3 plane YCbCr format, 2x2 subsampled Cb (1) and Cr (2) planes
    Yvu420 = 0x32315659,      // 3 plane YCbCr format, 2x2 subsampled Cr (1) and Cb (2) planes
    Yuv422 = 0x36315559,      // 3 plane YCbCr format, 2x1 subsampled Cb (1) and Cr (2) planes
    Yvu422 = 0x36315659,      // 3 plane YCbCr format, 2x1 subsampled Cr (1) and Cb (2) planes
    Yuv444 = 0x34325559,      // 3 plane YCbCr format, non-subsampled Cb (1) and Cr (2) planes
    Yvu444 = 0x34325659,      // 3 plane YCbCr format, non-subsampled Cr (1) and Cb (2) planes
}
