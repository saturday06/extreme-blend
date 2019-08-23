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
    req: Request,
) -> Box<dyn Future<Item = Session, Error = std::io::Error> + Send> {
    let res = if let Some(x) = session.resources.remove(&req.sender_object_id) {
        x
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
        println!(
            "object_id={} opcode={} args={:?} not found",
            req.sender_object_id, req.opcode, req.args
        );
        let f: Box<dyn Future<Item = Session, Error = std::io::Error> + Send> = Box::new(
            tx.send(Box::new(error))
                .map(|_| session)
                .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err)),
        );
        return f;
    };
    let f: Box<dyn Future<Item = Session, Error = std::io::Error> + Send> = Box::new(
        protocol::resource::dispatch_request(
            res,
            session,
            req.sender_object_id,
            req.opcode,
            req.args,
        )
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Oops!")),
    );
    f
}

fn handle_client(runtime: &mut Runtime, stream: tokio::net::TcpStream, global: Global) {
    let (writer0, reader0) = Codec::new().framed(stream).split();

    let (tx0, rx0) = tokio::sync::mpsc::channel::<Box<dyn Event + Send>>(48000);

    let output_session = rx0
        .map_err(|err| {
            println!("err={:?}", err);
            std::io::Error::new(std::io::ErrorKind::Other, "Oops!")
        })
        .forward(writer0)
        .map_err(|err| println!("err={:?}", err))
        .and_then(|_| Ok(()));
    runtime.spawn(output_session);

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
    };
    session0
        .resources
        .insert(1, Resource::WlDisplay(session0.wl_display.clone()));
    let input_session0 = reader0
        .fold(session0, handle_client_input)
        .map_err(|err| println!("err: {:?}", err))
        .then(|_| futures::future::ok(()));
    runtime.spawn(input_session0);
}

fn main() {
    let mut runtime = Runtime::new().unwrap();
    let global = Global {
        wl_display: Arc::new(RwLock::new(WlDisplay {})),
        wl_compositor: Arc::new(RwLock::new(WlCompositor {})),
        wl_registry: Arc::new(RwLock::new(WlRegistry {})),
        wl_shm: Arc::new(RwLock::new(WlShm {})),
        wl_data_device_manager: Arc::new(RwLock::new(WlDataDeviceManager {})),
        xdg_wm_base: Arc::new(RwLock::new(XdgWmBase {})),
    };

    let mut server_socket = ServerSocket::bind().unwrap();
    loop {
        let stream = if let Some(x) = server_socket.accept() {
            x
        } else {
            eprintln!("Oops!");
            continue;
        };
        handle_client(&mut runtime, stream, global.clone());
    }
}
