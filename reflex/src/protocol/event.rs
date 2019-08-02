use bytes::BytesMut;

pub trait Event {
    fn encode(&self, dst: &mut BytesMut) -> Result<(), std::io::Error>;
}
