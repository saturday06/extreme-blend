use byteorder::{ByteOrder, NativeEndian, ReadBytesExt};
use bytes::BytesMut;
use std::cell::RefCell;
use futures::future::Future;
use futures::sink::Sink;
use futures::stream::Stream;
use std::collections::HashMap;
use std::io::Cursor;
use std::io::Read;
use std::os::raw::{c_char, c_int};
use std::sync::{Arc, RwLock};
use std::rc::Rc;
use tokio::codec::Decoder;
use tokio::reactor::Handle;
use tokio::runtime::Runtime;
use tokio_uds::{UnixListener, UnixStream};
use protocol::session::Session;
use protocol::wayland::wl_registry::WlRegistry;
use protocol::wayland::wl_compositor::WlCompositor;
use protocol::wayland::wl_shm::WlShm;
use protocol::wayland::wl_display;
use protocol::wayland::wl_display::WlDisplay;
use protocol::xdg_shell::xdg_wm_base::XdgWmBase;
use protocol::event::Event;
use protocol::codec::Codec;
use protocol::request::Request;
use protocol::resource::Resource;

mod protocol;

fn handle_client(mut runtime: Arc<RwLock<Runtime>>, stream: UnixStream) -> Box<Future<Item = (), Error = std::io::Error> + Send> {
    let mut session0 = Session {
        wl_registry: WlRegistry {},
        wl_compositor: WlCompositor {},
        wl_shm: WlShm { },
        xdg_wm_base: XdgWmBase { },
        resources: HashMap::new(),
    };
    session0
        .resources
        .insert(1, Resource::WlDisplay(Rc::new(RefCell::new(WlDisplay{}))));
    let (tx0, rx0) = tokio::sync::mpsc::channel::<Box<Event + Send>>(1000);
    let (writer0, reader0) = Codec::new().framed(stream).split();
    let output_session = rx0
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Oops!"))
        .forward(writer0)
        .map_err(|_| ())
        .and_then(|_| Ok(()));
    runtime.write().unwrap().spawn(output_session);

    let input_session = reader0
        .for_each(move |req: Request| {
            let opt_res = session0
                .resources
                .get(&req.sender_object_id)
                .map(|r| r.clone());
            let tx = tx0.clone();
            let h = if let Some(res) = opt_res {
                protocol::resource::dispatch_request(
                    res,
                    &mut session0,
                    tx,
                    req.sender_object_id,
                    req.opcode,
                    req.args,
                )
            } else {
                Box::new(
                    tx.send(Box::new(wl_display::events::Error {
                        /*
                        object_id: 1,
                        code: WL_DISPLAY_ERROR_INVALID_OBJECT,
                        message: format!(
                            "object_id={} opcode={} args={:?} not found",
                            req.sender_object_id, req.opcode, req.args
                        ),*/
                    }))
                        .map_err(|_| ())
                        .map(|_tx| ()),
                )
            };
            h.map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Oops!"))
        })
        .map_err(|_| ());

    Box::new(futures::future::ok(()))
}

fn main() {
    let mut runtime = Arc::new(RwLock::new(Runtime::new().unwrap()));

    let socket_path = "/mnt/c/Temp/temp.unix";
    std::fs::remove_file(socket_path).unwrap();
    tokio::run(UnixListener::bind(socket_path)
        .unwrap()
        .incoming()
        .for_each(move |stream| {
            handle_client(runtime.clone(), stream)
        })
        .map_err(|_| ()));
    println!("ok");
}
