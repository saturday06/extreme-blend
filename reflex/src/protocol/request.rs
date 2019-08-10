use std::os::unix::io::RawFd;

pub struct Request {
    pub sender_object_id: u32,
    pub opcode: u16,
    pub args: Vec<u8>,
    pub fds: Vec<RawFd>,
}
