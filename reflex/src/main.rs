use byteorder::{ByteOrder, NativeEndian, ReadBytesExt};
use bytes::BytesMut;
use futures::future::Future;
use futures::sink::Sink;
use futures::stream::Stream;
use std::collections::HashMap;
use std::io::Cursor;
use std::io::Read;
use std::os::raw::{c_char, c_int};
use std::sync::{Arc, RwLock};
use tokio::codec::Decoder;
use tokio::reactor::Handle;
use tokio::runtime::Runtime;

mod protocol;

fn main() {
    println!("ok");
}
