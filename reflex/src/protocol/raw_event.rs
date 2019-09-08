use super::event::Event;
use bytes::BytesMut;

pub struct RawEvent {
    pub data: Vec<u8>,
}

impl Event for RawEvent {
    fn encode(&self, dst: &mut BytesMut) -> Result<(), std::io::Error> {
        dst.extend_from_slice(&self.data[..]);
        Ok(())
    }
}
