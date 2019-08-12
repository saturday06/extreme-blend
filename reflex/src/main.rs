use crate::protocol::event_sink::EventSink;
use crate::protocol::request_stream::RequestStream;
use futures::future::Future;
use futures::sink::Sink;
use futures::stream::Stream;
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
use std::sync::{Arc, RwLock};
use crate::protocol::connection_stream::ConnectionStream;
use crate::protocol::fd_drop::FdDrop;
use tokio::net::UnixStream;

mod protocol;

fn main() {
    let socket_path = "/tmp/temp.unix";
    let _ = std::fs::remove_file(socket_path);

    let wl_display = Arc::new(RwLock::new(WlDisplay {}));
    let wl_compositor = Arc::new(RwLock::new(WlCompositor {}));
    let wl_registry = Arc::new(RwLock::new(WlRegistry {}));
    let wl_shm = Arc::new(RwLock::new(WlShm {}));
    let wl_data_device_manager = Arc::new(RwLock::new(WlDataDeviceManager {}));
    let xdg_wm_base = Arc::new(RwLock::new(XdgWmBase {}));

    let listener = ConnectionStream::bind(socket_path.to_string()).for_each(move |fd| {
        let inner_wl_display = wl_display.clone();
        let inner_wl_registry = wl_registry.clone();
        let inner_wl_compositor = wl_compositor.clone();
        let inner_wl_shm = wl_shm.clone();
        let inner_wl_data_device_manager = wl_data_device_manager.clone();
        let inner_xdg_wm_base = xdg_wm_base.clone();

        UnixStream::connect("/mnt/c/Temp/reflex.unix").and_then(move |stream| {
            //let fd = default_stream.as_raw_fd();
            //unsafe { libc::fcntl(fd, libc::F_SETFL, libc::O_NONBLOCK) };
            //let arc_stream = Arc::new(default_stream);
            let fd_drop = Arc::new(FdDrop::new(fd));
            let tokio_registration = Arc::new(tokio::reactor::Registration::new());
            tokio_registration
                .register(&mio::unix::EventedFd(&fd))
                .expect("register request fd");
            let reader0 = RequestStream::new(fd, fd_drop.clone(), tokio_registration.clone());
            let writer0 = EventSink::new(fd, fd_drop.clone(), tokio_registration.clone());
            let (tx0, rx0) = tokio::sync::mpsc::channel::<Box<Event + Send>>(48000);
            let output_session = rx0
                .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Oops!"))
                .forward(writer0)
                .map_err(|_| ())
                .and_then(|_| Ok(()));
            tokio::spawn(output_session);

            let mut session0 = Session {
                wl_display: inner_wl_display,
                wl_registry: inner_wl_registry,
                wl_compositor: inner_wl_compositor,
                wl_shm: inner_wl_shm,
                wl_data_device_manager: inner_wl_data_device_manager,
                xdg_wm_base: inner_xdg_wm_base,
                resources: HashMap::new(),
                tx: tx0,
                callback_data: 0,
                fds: Vec::new(),
                unix_stream: stream,
            };
            session0
                .resources
                .insert(1, Resource::WlDisplay(session0.wl_display.clone()));
            let input_session0: Box<Future<Item = (), Error = ()> + Send> = Box::new(
                reader0
                    .fold(
                        session0,
                        |mut session: Session,
                         req: Request|
                         -> Box<Future<Item = Session, Error = ()> + Send> {
                            session.fds.extend(req.fds);
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
                                let f: Box<Future<Item = Session, Error = ()> + Send> =
                                    Box::new(tx.send(Box::new(error)).map(|_| session).map_err(|_| ()));
                                f
                            }
                        },
                    )
                    .map(|_| ())
                    //.map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Oops!")),
                    .then(|_| futures::future::ok(())),
            );

            tokio::spawn(input_session0);

            futures::future::ok(())
        })
    });
    //.map_err(|err| println!("Error: {:?}", err));
    tokio::run(listener.map_err(|_| ()));
    println!("Exit");
}
