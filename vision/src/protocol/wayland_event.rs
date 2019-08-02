use bytes::BytesMut;

pub trait WaylandEvent {
    fn encode(&self, dst: &mut BytesMut) -> Result<(), std::io::Error>;
}
