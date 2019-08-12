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
use server_socket::ServerSocket;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use tokio::codec::Decoder;
use tokio::runtime::Runtime;

mod protocol;
mod server_socket;

fn main() {
    let mut runtime = Runtime::new().unwrap();
    let wl_display = Arc::new(RwLock::new(WlDisplay {}));
    let wl_compositor = Arc::new(RwLock::new(WlCompositor {}));
    let wl_registry = Arc::new(RwLock::new(WlRegistry {}));
    let wl_shm = Arc::new(RwLock::new(WlShm {}));
    let wl_data_device_manager = Arc::new(RwLock::new(WlDataDeviceManager {}));
    let xdg_wm_base = Arc::new(RwLock::new(XdgWmBase {}));

    let mut server_socket = ServerSocket::bind().unwrap();
    loop {
        let inner_wl_display = wl_display.clone();
        let inner_wl_registry = wl_registry.clone();
        let inner_wl_compositor = wl_compositor.clone();
        let inner_wl_shm = wl_shm.clone();
        let inner_wl_data_device_manager = wl_data_device_manager.clone();
        let inner_xdg_wm_base = xdg_wm_base.clone();

        let stream = if let Some(x) = server_socket.accept() {
            x
        } else {
            eprintln!("Oops!");
            continue;
        };
        let (_writer0, reader0) = Codec::new().framed(stream).split();

        let (tx0, _) = tokio::sync::mpsc::channel::<Box<Event + Send>>(48000);

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
        };
        session0
            .resources
            .insert(1, Resource::WlDisplay(session0.wl_display.clone()));
        let input_session0: Box<Future<Item = (), Error = ()> + Send> = Box::new(
            reader0
                .map_err(|_| ())
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
                            let f: Box<Future<Item = Session, Error = ()> + Send> =
                                Box::new(tx.send(Box::new(error)).map(|_| session).map_err(|_| ()));
                            f
                        }
                    },
                )
                .map(|_| ())
                .then(|_| futures::future::ok(())),
        );

        runtime.spawn(input_session0);
    }
}
