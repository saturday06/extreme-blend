use crate::protocol::event_sink::EventSink;
use crate::protocol::request_stream::RequestStream;
use futures::future::Future;
use futures::sink::Sink;
use futures::stream::Stream;
//use protocol::codec::Codec;
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
use std::collections::HashMap;
use std::os::unix::io::AsRawFd;
use std::os::unix::io::RawFd;
//use std::os::unix::io::FromRawFd;
use std::sync::{Arc, RwLock};
//use tokio::codec::Decoder;
use tokio::net::UnixListener;
//use tokio::reactor::Handle;
use tokio::runtime::Runtime;

mod protocol;
//mod uds;

fn main() {
    let runtime = Runtime::new().unwrap();
    let executor = runtime.executor();

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
            let fd = stream.as_raw_fd();
            let tokio_registration = Arc::new(tokio::reactor::Registration::new());
            tokio_registration
                .register(&mio::unix::EventedFd(&fd))
                .expect("register request fd");
            let arc_stream = Arc::new(stream);
            let reader0 = RequestStream::new(arc_stream.clone(), fd, tokio_registration.clone());
            let writer0 = EventSink::new(arc_stream.clone(), fd, tokio_registration.clone());
            let (tx0, rx0) = tokio::sync::mpsc::channel::<Box<Event + Send>>(48000);
            let output_session = rx0
                .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Oops!"))
                .forward(writer0)
                .map_err(|_| ())
                .and_then(|_| Ok(()));
            executor.spawn(output_session);

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
            let input_session0: Box<Future<Item = (), Error = std::io::Error> + Send> = Box::new(
                reader0
                    .fold(
                        session0,
                        |mut session: Session,
                         req: Request|
                         -> Box<Future<Item = Session, Error = ()> + Send> {
                            let opt_res = session.resources.remove(&req.sender_object_id);
                            if let Some(res) = opt_res {
                                let f: Box<Future<Item = Session, Error = ()> + Send> = Box::new(
                                    protocol::resource::dispatch_request(
                                        res,
                                        session,
                                        req.sender_object_id,
                                        req.opcode,
                                        req.args,
                                    )
                                    .map_err(|_| ()),
                                );
                                f
                            } else {
                                let tx = session.tx.clone();
                                let error = wl_display::events::Error {
                                    sender_object_id: 1,
                                    object_id: 1,
                                    code: wl_display::enums::Error::InvalidObject as u32,
                                    message: format!(
                                        "object_id={} opcode={} args={:?} not found",
                                        req.sender_object_id, req.opcode, req.args
                                    ),
                                };
                                let f: Box<Future<Item = Session, Error = ()> + Send> = Box::new(
                                    tx.send(Box::new(error)).map(|_| session).map_err(|_| ()),
                                );
                                f
                            }
                        },
                    )
                    .map(|_| ())
                    .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Oops!")),
            );
            input_session0
        })
        .map_err(|err| println!("Error: {:?}", err));
    if let Err(err) = runtime.block_on_all(listener) {
        println!("Error");
    }
    println!("Exit");
}
