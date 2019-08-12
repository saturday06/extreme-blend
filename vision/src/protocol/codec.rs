use super::event::Event;
use super::request::Request;
use byteorder::{NativeEndian, ReadBytesExt};
use bytes::BytesMut;
use std::io::{Cursor, Read};

pub struct Codec;

impl Codec {
    pub fn new() -> Codec {
        Codec {}
    }
}

impl tokio::codec::Encoder for Codec {
    type Item = Box<Event + Send>;
    type Error = std::io::Error;

    fn encode(&mut self, res: Box<Event + Send>, dst: &mut BytesMut) -> Result<(), Self::Error> {
        res.encode(dst)
    }
}

impl tokio::codec::Decoder for Codec {
    type Item = Request;
    type Error = std::io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Request>, Self::Error> {
        panic!("not unimplemented");
        let header_size = 8;
        if src.is_empty() || src.len() < header_size {
            return Ok(None);
        }

        // https://wayland.freedesktop.org/docs/html/ch04.html#sect-Protocol-Wire-Format
        let (req, message_size) = {
            let mut cursor = Cursor::new(&src);
            let sender_object_id = cursor.read_u32::<NativeEndian>().unwrap();
            let message_size_and_opcode = cursor.read_u32::<NativeEndian>().unwrap();
            let message_size = (message_size_and_opcode >> 16) as usize;
            if src.len() < message_size {
                return Ok(None);
            }

            let opcode = (0x0000ffff & message_size_and_opcode) as u16;
            if message_size < header_size {
                return Err(std::io::Error::new(std::io::ErrorKind::Other, "Oops!"));
            }
            let mut args = Vec::new();
            args.resize(message_size - header_size, 0);
            cursor.read_exact(&mut args).unwrap();
            println!(
                "decode: id={} opcode={} args={:?}",
                sender_object_id, opcode, &args
            );
            (
                Request {
                    sender_object_id,
                    opcode,
                    args,
                    fds: Vec::new(),
                },
                message_size,
            )
        };
        src.advance(message_size);
        return Ok(Some(req));
    }
}
