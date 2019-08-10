use crate::protocol::request_stream::RequestStream;
use futures::future::Future;
use futures::stream::Stream;
use nix::sys::socket::{
    connect, sendmsg, socket, AddressFamily, ControlMessage, MsgFlags, SockAddr, SockFlag,
    SockType, UnixAddr,
};
use nix::sys::uio::IoVec;
use nix::unistd::pipe;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::time::Duration;
use std::sync::Arc;
use tokio::net::UnixListener;

fn receive_thread(socket_path: String) {
    let listener = UnixListener::bind(socket_path)
        .unwrap()
        .incoming()
        .for_each(move |default_stream| {
            let stream = RequestStream::new(Arc::new(default_stream));
            /*
            let mut received_r: Option<RawFd> = None;
            let mut buf = [0u8; 5];
            let iov = [IoVec::from_mut_slice(&mut buf[..])];
            let mut cmsgspace = cmsg_space!([RawFd; 1]);
            let msg = recvmsg(stream.fd, &iov, Some(&mut cmsgspace), MsgFlags::empty()).unwrap();

            for cmsg in msg.cmsgs() {
                if let ControlMessageOwned::ScmRights(fd) = cmsg {
                    assert_eq!(received_r, None);
                    assert_eq!(fd.len(), 1);
                    received_r = Some(fd[0]);
                } else {
                    panic!("unexpected cmsg");
                }
            }
            assert_eq!(msg.bytes, 5);
            assert!(!msg
                .flags
                .intersects(MsgFlags::MSG_TRUNC | MsgFlags::MSG_CTRUNC));
            //close(fd).unwrap();
            println!("fd={:?}", received_r);

            futures::future::err(std::io::Error::new(std::io::ErrorKind::Other, "ok"))
            */

            let _ = stream
                .for_each(|r| futures::future::ok(()))
                .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "ok"));
            futures::future::err(std::io::Error::new(std::io::ErrorKind::Other, "ok"))
        })
        .map_err(|_| ());
    tokio::run(listener);
}

fn send_thread(socket_path: String) {
    for _x in &[0..10] {
        if Path::exists(Path::new(&socket_path)) {
            break;
        }
        std::thread::sleep(Duration::from_millis(500));
    }
    let mut perms = fs::metadata(&socket_path).unwrap().permissions();
    perms.set_mode(0700);
    fs::set_permissions(&socket_path, perms).unwrap();

    let unix_addr = UnixAddr::new(socket_path.as_bytes()).unwrap();
    println!("unix_addr={:?}", unix_addr);
    let sock_addr = SockAddr::Unix(unix_addr);
    println!("sa={:?} f={:?}", sock_addr, sock_addr.family());

    let fd = socket(
        AddressFamily::Unix,
        SockType::Stream,
        SockFlag::empty(),
        //SockFlag::SOCK_NONBLOCK,
        None,
    )
    .unwrap();

    if let Err(err) = connect(fd, &sock_addr) {
        eprintln!("err: {:?}", err);
        return;
    }

    let (r, w) = pipe().unwrap();
    let buf = b"helloworldaaaaaaaaaaaaaaaaaa";
    let buf_len = buf.len();
    let iov = [IoVec::from_slice(buf)];
    let fds = [r];
    let cmsg = ControlMessage::ScmRights(&fds);
    assert_eq!(
        sendmsg(fd, &iov, &[cmsg], MsgFlags::empty(), None).unwrap(),
        buf_len
    );
}

#[test]
fn it_works() {
    println!("start");
    let socket_path = "/tmp/socket.uds";
    let _ = std::fs::remove_file(socket_path);

    let client = std::thread::spawn(move || send_thread(socket_path.to_string()));
    //let server = std::thread::spawn(move || receive_thread(socket_path.to_string()));
    receive_thread(socket_path.to_string());
    send_thread(socket_path.to_string());

    client.join().expect("client join");
    //server.join().expect("server join");
}
