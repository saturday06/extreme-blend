use futures::future::Future;
use futures::sink::Sink;
use futures::stream::Stream;
use protocol::codec::Codec;
use protocol::event::Event;
use protocol::request::Request;
use protocol::resource::Resource;
use protocol::session::Session;
use protocol::wayland::wl_compositor::WlCompositor;
use protocol::wayland::wl_data_device_manager::WlDataDeviceManager;
use protocol::wayland::wl_display;
use protocol::wayland::wl_display::WlDisplay;
use protocol::wayland::wl_registry::WlRegistry;
use protocol::wayland::wl_shm::WlShm;
use protocol::xdg_shell::xdg_wm_base::XdgWmBase;
use std::os::unix::io::AsRawFd;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::os::unix::io::FromRawFd;
use tokio::codec::Decoder;
use tokio::runtime::Runtime;
use tokio::net::{UnixListener, UnixStream};
use mio::Ready;

mod protocol;
mod uds;

fn main() {
    let runtime = Runtime::new().unwrap();
    let executor = runtime.executor();
    let reactor = runtime.reactor().clone();

    let socket_path = "/tmp/temp.unix";
    let _ = std::fs::remove_file(socket_path);

    let wl_display = Arc::new(RwLock::new(WlDisplay {}));
    let wl_compositor = Arc::new(RwLock::new(WlCompositor {}));
    let wl_registry = Arc::new(RwLock::new(WlRegistry {}));
    let wl_shm = Arc::new(RwLock::new(WlShm {}));
    let wl_data_device_manager = Arc::new(RwLock::new(WlDataDeviceManager {}));
    let xdg_wm_base = Arc::new(RwLock::new(XdgWmBase {}));

    let listener = UnixListener::bind(socket_path)
        .unwrap()
        .incoming()
        .for_each(move |stream| {
            let input_stream0 = UnixStream::from_std(unsafe {
                std::os::unix::net::UnixStream::from_raw_fd(stream.as_raw_fd())
            }, &reactor).unwrap();

            let tx0 = {
                let (tx0, rx0) = tokio::sync::mpsc::channel::<Box<Event + Send>>(1000);
                let (writer0, reader0) = Codec::new().framed(stream).split();
                let output_session = rx0
                    .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Oops!"))
                    .forward(writer0)
                    .map_err(|_| ())
                    .and_then(|_| Ok(()));
                executor.spawn(output_session);
                tx0
            };

            let mut session0 = Session {
                wl_display: wl_display.clone(),
                wl_registry: wl_registry.clone(),
                wl_compositor: wl_compositor.clone(),
                wl_shm: wl_shm.clone(),
                wl_data_device_manager: wl_data_device_manager.clone(),
                xdg_wm_base: xdg_wm_base.clone(),
                resources: HashMap::new(),
                tx: tx0,
                callback_data: 0,
            };
            session0
                .resources
                .insert(1, Resource::WlDisplay(wl_display.clone()));
/*
            let x: Box<Future<Item = (), Error = std::io::Error>> = Box::new(futures::future::loop_fn(input_stream0, |input_stream| {
                let fd = input_stream.as_raw_fd();
                let poll = input_stream.poll_read_ready(Ready::readable());
                futures::future::ok(input_stream)
                    .and_then(|input_stream| poll)
                    .and_then(|_| futures::future::ok(()))
                    .and_then(|_: ()|  Ok(futures::future::Loop::Continue(input_stream)))
            }));
*/

/*
            let input_session0: Box<Future<Item = (), Error = std::io::Error> + Send> = Box::new(reader0
                .fold(session0, |mut session: Session, req: Request| -> Box<Future<Item = Session, Error = std::io::Error> + Send> {
                    let opt_res = session
                        .resources
                        .remove(&req.sender_object_id);
                    if let Some(res) = opt_res {
                        let f: Box<Future<Item = Session, Error = std::io::Error> + Send> = Box::new(
                            protocol::resource::dispatch_request(
                                res,
                                session,
                                req.sender_object_id,
                                req.opcode,
                                req.args,
                            ).map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Oops!")),
                        );
                        f
                    } else {
                        let tx = session.tx.clone();
                        let f: Box<Future<Item = Session, Error = std::io::Error> + Send> = Box::new(
                            tx.send(Box::new(wl_display::events::Error {
                                sender_object_id: 1,
                                object_id: 1,
                                code: wl_display::enums::Error::InvalidObject as u32,
                                message: format!(
                                    "object_id={} opcode={} args={:?} not found",
                                    req.sender_object_id, req.opcode, req.args
                                ),
                            }))
                                .map(|_| session)
                                .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Oops!")),
                        );
                        f
                    }
                }).map(|_| ()).map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Oops!")));
*/
            let input_session0 = Box::new(futures::future::ok(()));
            //input_session0.or_else(|_| futures::future::ok(()))
            input_session0
        })
        .map_err(|err| println!("Error: {:?}", err));
    if let Err(err) = runtime.block_on_all(listener) {
        println!("Error: {:?}", err);
    }
    println!("Exit");
}
