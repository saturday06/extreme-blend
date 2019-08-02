pub struct WaylandRequest {
    pub sender_object_id: u32,
    pub opcode: u16,
    pub args: Vec<u8>,
}
