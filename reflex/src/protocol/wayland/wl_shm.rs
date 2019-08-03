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
    // wl_shm error values
    //
    // These errors can be emitted in response to wl_shm requests.
    pub enum Error {
        InvalidFormat = 0, // buffer format is not known
        InvalidStride = 1, // invalid size or stride during pool or buffer creation
        InvalidFd = 2, // mmapping the file descriptor failed
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
        Argb8888 = 0, // 32-bit ARGB format, [31:0] A:R:G:B 8:8:8:8 little endian
        Xrgb8888 = 1, // 32-bit RGB format, [31:0] x:R:G:B 8:8:8:8 little endian
        C8 = 0x20203843, // 8-bit color index format, [7:0] C
        Rgb332 = 0x38424752, // 8-bit RGB format, [7:0] R:G:B 3:3:2
        Bgr233 = 0x38524742, // 8-bit BGR format, [7:0] B:G:R 2:3:3
        Xrgb4444 = 0x32315258, // 16-bit xRGB format, [15:0] x:R:G:B 4:4:4:4 little endian
        Xbgr4444 = 0x32314258, // 16-bit xBGR format, [15:0] x:B:G:R 4:4:4:4 little endian
        Rgbx4444 = 0x32315852, // 16-bit RGBx format, [15:0] R:G:B:x 4:4:4:4 little endian
        Bgrx4444 = 0x32315842, // 16-bit BGRx format, [15:0] B:G:R:x 4:4:4:4 little endian
        Argb4444 = 0x32315241, // 16-bit ARGB format, [15:0] A:R:G:B 4:4:4:4 little endian
        Abgr4444 = 0x32314241, // 16-bit ABGR format, [15:0] A:B:G:R 4:4:4:4 little endian
        Rgba4444 = 0x32314152, // 16-bit RBGA format, [15:0] R:G:B:A 4:4:4:4 little endian
        Bgra4444 = 0x32314142, // 16-bit BGRA format, [15:0] B:G:R:A 4:4:4:4 little endian
        Xrgb1555 = 0x35315258, // 16-bit xRGB format, [15:0] x:R:G:B 1:5:5:5 little endian
        Xbgr1555 = 0x35314258, // 16-bit xBGR 1555 format, [15:0] x:B:G:R 1:5:5:5 little endian
        Rgbx5551 = 0x35315852, // 16-bit RGBx 5551 format, [15:0] R:G:B:x 5:5:5:1 little endian
        Bgrx5551 = 0x35315842, // 16-bit BGRx 5551 format, [15:0] B:G:R:x 5:5:5:1 little endian
        Argb1555 = 0x35315241, // 16-bit ARGB 1555 format, [15:0] A:R:G:B 1:5:5:5 little endian
        Abgr1555 = 0x35314241, // 16-bit ABGR 1555 format, [15:0] A:B:G:R 1:5:5:5 little endian
        Rgba5551 = 0x35314152, // 16-bit RGBA 5551 format, [15:0] R:G:B:A 5:5:5:1 little endian
        Bgra5551 = 0x35314142, // 16-bit BGRA 5551 format, [15:0] B:G:R:A 5:5:5:1 little endian
        Rgb565 = 0x36314752, // 16-bit RGB 565 format, [15:0] R:G:B 5:6:5 little endian
        Bgr565 = 0x36314742, // 16-bit BGR 565 format, [15:0] B:G:R 5:6:5 little endian
        Rgb888 = 0x34324752, // 24-bit RGB format, [23:0] R:G:B little endian
        Bgr888 = 0x34324742, // 24-bit BGR format, [23:0] B:G:R little endian
        Xbgr8888 = 0x34324258, // 32-bit xBGR format, [31:0] x:B:G:R 8:8:8:8 little endian
        Rgbx8888 = 0x34325852, // 32-bit RGBx format, [31:0] R:G:B:x 8:8:8:8 little endian
        Bgrx8888 = 0x34325842, // 32-bit BGRx format, [31:0] B:G:R:x 8:8:8:8 little endian
        Abgr8888 = 0x34324241, // 32-bit ABGR format, [31:0] A:B:G:R 8:8:8:8 little endian
        Rgba8888 = 0x34324152, // 32-bit RGBA format, [31:0] R:G:B:A 8:8:8:8 little endian
        Bgra8888 = 0x34324142, // 32-bit BGRA format, [31:0] B:G:R:A 8:8:8:8 little endian
        Xrgb2101010 = 0x30335258, // 32-bit xRGB format, [31:0] x:R:G:B 2:10:10:10 little endian
        Xbgr2101010 = 0x30334258, // 32-bit xBGR format, [31:0] x:B:G:R 2:10:10:10 little endian
        Rgbx1010102 = 0x30335852, // 32-bit RGBx format, [31:0] R:G:B:x 10:10:10:2 little endian
        Bgrx1010102 = 0x30335842, // 32-bit BGRx format, [31:0] B:G:R:x 10:10:10:2 little endian
        Argb2101010 = 0x30335241, // 32-bit ARGB format, [31:0] A:R:G:B 2:10:10:10 little endian
        Abgr2101010 = 0x30334241, // 32-bit ABGR format, [31:0] A:B:G:R 2:10:10:10 little endian
        Rgba1010102 = 0x30334152, // 32-bit RGBA format, [31:0] R:G:B:A 10:10:10:2 little endian
        Bgra1010102 = 0x30334142, // 32-bit BGRA format, [31:0] B:G:R:A 10:10:10:2 little endian
        Yuyv = 0x56595559, // packed YCbCr format, [31:0] Cr0:Y1:Cb0:Y0 8:8:8:8 little endian
        Yvyu = 0x55595659, // packed YCbCr format, [31:0] Cb0:Y1:Cr0:Y0 8:8:8:8 little endian
        Uyvy = 0x59565955, // packed YCbCr format, [31:0] Y1:Cr0:Y0:Cb0 8:8:8:8 little endian
        Vyuy = 0x59555956, // packed YCbCr format, [31:0] Y1:Cb0:Y0:Cr0 8:8:8:8 little endian
        Ayuv = 0x56555941, // packed AYCbCr format, [31:0] A:Y:Cb:Cr 8:8:8:8 little endian
        Nv12 = 0x3231564e, // 2 plane YCbCr Cr:Cb format, 2x2 subsampled Cr:Cb plane
        Nv21 = 0x3132564e, // 2 plane YCbCr Cb:Cr format, 2x2 subsampled Cb:Cr plane
        Nv16 = 0x3631564e, // 2 plane YCbCr Cr:Cb format, 2x1 subsampled Cr:Cb plane
        Nv61 = 0x3136564e, // 2 plane YCbCr Cb:Cr format, 2x1 subsampled Cb:Cr plane
        Yuv410 = 0x39565559, // 3 plane YCbCr format, 4x4 subsampled Cb (1) and Cr (2) planes
        Yvu410 = 0x39555659, // 3 plane YCbCr format, 4x4 subsampled Cr (1) and Cb (2) planes
        Yuv411 = 0x31315559, // 3 plane YCbCr format, 4x1 subsampled Cb (1) and Cr (2) planes
        Yvu411 = 0x31315659, // 3 plane YCbCr format, 4x1 subsampled Cr (1) and Cb (2) planes
        Yuv420 = 0x32315559, // 3 plane YCbCr format, 2x2 subsampled Cb (1) and Cr (2) planes
        Yvu420 = 0x32315659, // 3 plane YCbCr format, 2x2 subsampled Cr (1) and Cb (2) planes
        Yuv422 = 0x36315559, // 3 plane YCbCr format, 2x1 subsampled Cb (1) and Cr (2) planes
        Yvu422 = 0x36315659, // 3 plane YCbCr format, 2x1 subsampled Cr (1) and Cb (2) planes
        Yuv444 = 0x34325559, // 3 plane YCbCr format, non-subsampled Cb (1) and Cr (2) planes
        Yvu444 = 0x34325659, // 3 plane YCbCr format, non-subsampled Cr (1) and Cb (2) planes
    }
}

pub mod events {
    use byteorder::{ByteOrder, NativeEndian};

    // pixel format description
    //
    // Informs the client about a valid pixel format that
    // can be used for buffers. Known formats include
    // argb8888 and xrgb8888.
    pub struct Format {
        pub sender_object_id: u32,
        pub format: u32, // uint: buffer pixel format
    }

    impl super::super::super::event::Event for Format {
        fn encode(&self, dst: &mut bytes::BytesMut) -> Result<(), std::io::Error> {
            let total_len = 8 + 4;
            if total_len > 0xffff {
                return Err(std::io::Error::new(std::io::ErrorKind::Other, "Oops!"));
            }

            let i = dst.len();
            dst.resize(i + total_len, 0);

            NativeEndian::write_u32(&mut dst[i..], self.sender_object_id);
            NativeEndian::write_u32(&mut dst[i + 4..], ((total_len << 16) | 0) as u32);

            NativeEndian::write_u32(&mut dst[i + 8..], self.format);
            Ok(())
        }
    }
}

pub fn dispatch_request(request: Arc<RwLock<WlShm>>, session: crate::protocol::session::Session, sender_object_id: u32, opcode: u16, args: Vec<u8>) -> Box<futures::future::Future<Item = crate::protocol::session::Session, Error = ()> + Send> {
    let mut cursor = Cursor::new(&args);
    match opcode {
        0 => {
            let id = if let Ok(x) = cursor.read_u32::<NativeEndian>() {
                x 
            } else {
                let tx = session.tx.clone();
                return Box::new(tx.send(Box::new(super::super::wayland::wl_display::events::Error {
                    sender_object_id: 1,
                    object_id: sender_object_id,
                    code: super::super::wayland::wl_display::enums::Error::InvalidMethod as u32,
                    message: format!(
                        "@{} opcode={} args={:?} not found",
                        sender_object_id, opcode, args
                    ),
                })).map_err(|_| ()).map(|_tx| session));

            };
            let fd = if let Ok(x) = cursor.read_i32::<NativeEndian>() {
                x 
            } else {
                let tx = session.tx.clone();
                return Box::new(tx.send(Box::new(super::super::wayland::wl_display::events::Error {
                    sender_object_id: 1,
                    object_id: sender_object_id,
                    code: super::super::wayland::wl_display::enums::Error::InvalidMethod as u32,
                    message: format!(
                        "@{} opcode={} args={:?} not found",
                        sender_object_id, opcode, args
                    ),
                })).map_err(|_| ()).map(|_tx| session));

            };
            let size = if let Ok(x) = cursor.read_i32::<NativeEndian>() {
                x
            } else {
                let tx = session.tx.clone();
                return Box::new(tx.send(Box::new(super::super::wayland::wl_display::events::Error {
                    sender_object_id: 1,
                    object_id: sender_object_id,
                    code: super::super::wayland::wl_display::enums::Error::InvalidMethod as u32,
                    message: format!(
                        "@{} opcode={} args={:?} not found",
                        sender_object_id, opcode, args
                    ),
                })).map_err(|_| ()).map(|_tx| session));

            };
            return WlShm::create_pool(request, session, sender_object_id, id, fd, size)
        },
        _ => {},
    };
    Box::new(futures::future::ok(session))
}

// shared memory support
//
// A singleton global object that provides support for shared
// memory.
// 
// Clients can create wl_shm_pool objects using the create_pool
// request.
// 
// At connection setup time, the wl_shm object emits one or more
// format events to inform clients about the valid pixel formats
// that can be used for buffers.
pub struct WlShm {
}

impl WlShm {
    // create a shm pool
    //
    // Create a new wl_shm_pool object.
    // 
    // The pool can be used to create shared memory based buffer
    // objects.  The server will mmap size bytes of the passed file
    // descriptor, to use as backing memory for the pool.
    pub fn create_pool(
        request: Arc<RwLock<WlShm>>,
        session: crate::protocol::session::Session,
        sender_object_id: u32,
        id: u32, // new_id: pool to create
        fd: i32, // fd: file descriptor for the pool
        size: i32, // int: pool size, in bytes
    ) -> Box<futures::future::Future<Item = crate::protocol::session::Session, Error = ()> + Send> {
        Box::new(futures::future::ok(session))
    }
}

impl Into<crate::protocol::resource::Resource> for WlShm {
    fn into(self) -> crate::protocol::resource::Resource {
        crate::protocol::resource::Resource::WlShm(Arc::new(RwLock::new(self)))
    }
}
