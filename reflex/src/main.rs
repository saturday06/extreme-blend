use crate::protocol::connection_stream::ConnectionStream;
use crate::protocol::event_sink::EventSink;
use crate::protocol::fd_drop::FdDrop;
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
use tokio::net::UnixStream;
use tokio::io::AsyncRead;

mod protocol;

#[derive(Clone)]
struct Global {
    wl_display: Arc<RwLock<WlDisplay>>,
    wl_compositor: Arc<RwLock<WlCompositor>>,
    wl_registry: Arc<RwLock<WlRegistry>>,
    wl_shm: Arc<RwLock<WlShm>>,
    wl_data_device_manager: Arc<RwLock<WlDataDeviceManager>>,
    xdg_wm_base: Arc<RwLock<XdgWmBase>>,
}

fn handle_client_input(
    mut session: Session,
    request: Request,
) -> Box<dyn Future<Item = Session, Error = ()> + Send> {
    session.fds.extend(request.fds);
    let res = if let Some(x) = session.resources.remove(&request.sender_object_id) {
        x
    } else {
        let tx = session.tx.clone();
        let error = wl_display::events::Error {
            sender_object_id: 1,
            object_id: 1,
            code: wl_display::enums::Error::InvalidObject as u32,
            message: format!(
                "object_id={} opcode={} args={:?} not found",
                request.sender_object_id, request.opcode, request.args
            ),
        };
        let f: Box<dyn Future<Item = Session, Error = ()> + Send> =
            Box::new(tx.send(Box::new(error)).map(|_| session).map_err(|_| ()));
        return f;
    };

    let f: Box<dyn Future<Item = Session, Error = ()> + Send> =
        protocol::resource::dispatch_request(
            res,
            session,
            request.sender_object_id,
            request.opcode,
            request.args,
        );
    f
}

fn handle_client(
    stream: UnixStream,
    global: Global,
    fd: i32,
) -> Box<dyn Future<Item = (), Error = std::io::Error> + Send> {
    let (r, w) = stream.split();
    let fd_drop = Arc::new(FdDrop::new(fd));
    let tokio_registration = Arc::new(tokio::reactor::Registration::new());
    tokio_registration
        .register(&mio::unix::EventedFd(&fd))
        .expect("register request fd");
    let reader0 = RequestStream::new(fd, fd_drop.clone(), tokio_registration.clone());
    let writer0 = EventSink::new(fd, fd_drop.clone(), tokio_registration.clone());
    let (tx0, rx0) = tokio::sync::mpsc::channel::<Box<dyn Event + Send>>(48000);
    let output_session = rx0
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Oops!"))
        .forward(writer0)
        .map_err(|_| ())
        .and_then(|_| Ok(()));
    tokio::spawn(output_session);

    let mut session0 = Session {
        wl_display: global.wl_display,
        wl_registry: global.wl_registry,
        wl_compositor: global.wl_compositor,
        wl_shm: global.wl_shm,
        wl_data_device_manager: global.wl_data_device_manager,
        xdg_wm_base: global.xdg_wm_base,
        resources: HashMap::new(),
        tx: tx0,
        callback_data: 0,
        fds: Vec::new(),
        unix_stream: w,
    };

    session0
        .resources
        .insert(1, Resource::WlDisplay(session0.wl_display.clone()));
    let input_session0: Box<dyn Future<Item = (), Error = ()> + Send> = Box::new(
        reader0
            .fold(session0, handle_client_input)
            .map(|_| ())
            //.map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Oops!")),
            .then(|_| futures::future::ok(())),
    );

    tokio::spawn(input_session0);

    Box::new(futures::future::ok(()))
}

fn main() {
    let socket_path = "/tmp/temp.unix";
    let _ = std::fs::remove_file(socket_path);

    let global = Global {
        wl_display: Arc::new(RwLock::new(WlDisplay {})),
        wl_compositor: Arc::new(RwLock::new(WlCompositor {})),
        wl_registry: Arc::new(RwLock::new(WlRegistry {})),
        wl_shm: Arc::new(RwLock::new(WlShm {})),
        wl_data_device_manager: Arc::new(RwLock::new(WlDataDeviceManager {})),
        xdg_wm_base: Arc::new(RwLock::new(XdgWmBase {})),
    };

    let listener = ConnectionStream::bind(socket_path.to_string()).for_each(move |fd| {
        let inner_global = global.clone();
        UnixStream::connect("/mnt/c/Temp/reflex.unix")
            .and_then(move |stream| handle_client(stream, inner_global, fd))
    });

    tokio::run(listener.map_err(|_| ()));
    println!("Exit");
}
