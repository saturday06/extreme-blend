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
use std::rc::Rc;
use std::cell::RefCell;

pub mod events {
    pub struct Cancel {
    }

    impl super::super::super::event::Event for Cancel {
        fn encode(&self, dst: &mut bytes::BytesMut) -> Result<(), std::io::Error> {
            Ok(())
        }
    }

    pub struct Down {
    }

    impl super::super::super::event::Event for Down {
        fn encode(&self, dst: &mut bytes::BytesMut) -> Result<(), std::io::Error> {
            Ok(())
        }
    }

    pub struct Frame {
    }

    impl super::super::super::event::Event for Frame {
        fn encode(&self, dst: &mut bytes::BytesMut) -> Result<(), std::io::Error> {
            Ok(())
        }
    }

    pub struct Motion {
    }

    impl super::super::super::event::Event for Motion {
        fn encode(&self, dst: &mut bytes::BytesMut) -> Result<(), std::io::Error> {
            Ok(())
        }
    }

    pub struct Orientation {
    }

    impl super::super::super::event::Event for Orientation {
        fn encode(&self, dst: &mut bytes::BytesMut) -> Result<(), std::io::Error> {
            Ok(())
        }
    }

    pub struct Shape {
    }

    impl super::super::super::event::Event for Shape {
        fn encode(&self, dst: &mut bytes::BytesMut) -> Result<(), std::io::Error> {
            Ok(())
        }
    }

    pub struct Up {
    }

    impl super::super::super::event::Event for Up {
        fn encode(&self, dst: &mut bytes::BytesMut) -> Result<(), std::io::Error> {
            Ok(())
        }
    }
}

pub fn dispatch_request(request: Rc<RefCell<WlTouch>>) -> Box<futures::future::Future<Item = (), Error = ()>> {
    Box::new(futures::future::ok(()))
}

pub struct WlTouch {
}
