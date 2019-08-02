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

pub mod enums {
    pub enum KeyState {
    }

    pub enum KeymapFormat {
    }
}

pub mod events {
    pub struct Enter {
    }

    impl super::super::super::event::Event for Enter {
        fn encode(&self, dst: &mut bytes::BytesMut) -> Result<(), std::io::Error> {
            Ok(())
        }
    }

    pub struct Key {
    }

    impl super::super::super::event::Event for Key {
        fn encode(&self, dst: &mut bytes::BytesMut) -> Result<(), std::io::Error> {
            Ok(())
        }
    }

    pub struct Keymap {
    }

    impl super::super::super::event::Event for Keymap {
        fn encode(&self, dst: &mut bytes::BytesMut) -> Result<(), std::io::Error> {
            Ok(())
        }
    }

    pub struct Leave {
    }

    impl super::super::super::event::Event for Leave {
        fn encode(&self, dst: &mut bytes::BytesMut) -> Result<(), std::io::Error> {
            Ok(())
        }
    }

    pub struct Modifiers {
    }

    impl super::super::super::event::Event for Modifiers {
        fn encode(&self, dst: &mut bytes::BytesMut) -> Result<(), std::io::Error> {
            Ok(())
        }
    }

    pub struct RepeatInfo {
    }

    impl super::super::super::event::Event for RepeatInfo {
        fn encode(&self, dst: &mut bytes::BytesMut) -> Result<(), std::io::Error> {
            Ok(())
        }
    }
}

pub fn dispatch_request(request: Rc<RefCell<WlKeyboard>>) -> Box<futures::future::Future<Item = (), Error = ()>> {
    Box::new(futures::future::ok(()))
}

pub struct WlKeyboard {
}
