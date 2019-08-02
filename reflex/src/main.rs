use byteorder::{ByteOrder, NativeEndian, ReadBytesExt};
use bytes::BytesMut;
use futures::future::Future;
use futures::sink::Sink;
use futures::stream::Stream;
use protocol::codec::Codec;
use protocol::event::Event;
use protocol::request::Request;
use protocol::resource::Resource;
use protocol::session::Session;
use protocol::wayland::wl_compositor::WlCompositor;
use protocol::wayland::wl_display;
use protocol::wayland::wl_display::WlDisplay;
use protocol::wayland::wl_registry::WlRegistry;
use protocol::wayland::wl_shm::WlShm;
use protocol::xdg_shell::xdg_wm_base::XdgWmBase;
use std::cell::RefCell;
use std::collections::HashMap;
use std::io::Cursor;
use std::io::Read;
use std::os::raw::{c_char, c_int};
use std::rc::Rc;
use std::sync::{Arc, RwLock};
use tokio::codec::Decoder;
use tokio::reactor::Handle;
use tokio::runtime::Runtime;
use tokio_uds::{UnixListener, UnixStream};

mod protocol;

fn main() {
    let mut runtime = Arc::new(RwLock::new(Runtime::new().unwrap()));

    let socket_path = "/tmp/temp.unix";
    let _ = std::fs::remove_file(socket_path);
    tokio::run(
        UnixListener::bind(socket_path)
            .unwrap()
            .incoming()
            .for_each(move |stream| {
                let (tx0, rx0) = tokio::sync::mpsc::channel::<Box<Event + Send>>(1000);
                let (writer0, reader0) = Codec::new().framed(stream).split();
                let output_session = rx0
                    .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Oops!"))
                    .forward(writer0)
                    .map_err(|_| ())
                    .and_then(|_| Ok(()));
                runtime.write().unwrap().spawn(output_session);

                let mut session0 = Session {
                    wl_registry: WlRegistry {},
                    wl_compositor: WlCompositor {},
                    wl_shm: WlShm {},
                    xdg_wm_base: XdgWmBase {},
                    resources: HashMap::new(),
                };
                /*
                session0
                    .resources
                    .insert(1, Resource::WlDisplay(Arc::new(RefCell::new(WlDisplay {}))));
                    */
                let input_session = reader0.for_each(move |req: Request| {
                    let opt_res = session0
                        .resources
                        .get(&req.sender_object_id)
                        .map(|r| r.clone());
                    let tx = tx0.clone();
                    /*
                    let h = if let Some(res) = opt_res {
                        protocol::resource::dispatch_request(
                            res,
                            &mut session0,
                            tx,
                            req.sender_object_id,
                            req.opcode,
                            req.args,
                        )
                    } else { */
                        Box::new(
                            tx.send(Box::new(wl_display::events::Error {
                                sender_object_id: 1,
                                object_id: 1,
                                code: wl_display::enums::Error::InvalidObject as u32,
                                message: format!(
                                    "object_id={} opcode={} args={:?} not found",
                                    req.sender_object_id, req.opcode, req.args
                                ),
                            }))
                            .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Oops!"))
                            .map(|_tx| ()),
                        )
                    //};
                    //h.map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Oops!"))
                });
                Box::new(input_session)
            })
            .map_err(|_| ()),
    );
    println!("ok");
}
