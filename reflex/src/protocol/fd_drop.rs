use std::os::unix::io::RawFd;

pub struct FdDrop {
    fd: RawFd,
}

impl FdDrop {
    pub(crate) fn new(fd: RawFd) -> FdDrop {
        FdDrop { fd }
    }
}

impl Drop for FdDrop {
    fn drop(&mut self) {
        unsafe {
            libc::close(self.fd);
        }
    }
}
