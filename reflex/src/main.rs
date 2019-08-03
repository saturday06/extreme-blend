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
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::rc::Rc;
use std::cell::RefCell;
use tokio::codec::Decoder;
use tokio::runtime::Runtime;
use tokio_uds::{UnixListener, UnixStream};

mod protocol;

fn main() {
    let mut runtime = Runtime::new().unwrap();

    let socket_path = "/tmp/temp.unix";
    let _ = std::fs::remove_file(socket_path);
    let listener = UnixListener::bind(socket_path)
        .unwrap()
        .incoming()
        .for_each(move |stream| {
            let (reader0, tx0) = {
                let (tx0, rx0) = tokio::sync::mpsc::channel::<Box<Event + Send>>(1000);
                let (writer0, reader0) = Codec::new().framed(stream).split();
                let output_session = rx0
                    .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Oops!"))
                    .forward(writer0)
                    .map_err(|_| ())
                    .and_then(|_| Ok(()));
                runtime.spawn(output_session);
                (reader0, tx0)
            };

            let mut session0 = Session {
                wl_registry: WlRegistry {},
                wl_compositor: WlCompositor {},
                wl_shm: WlShm {},
                xdg_wm_base: XdgWmBase {},
                resources: HashMap::new(),
                tx: tx0,
            };
            session0
                .resources
                .insert(1, Resource::WlDisplay(Arc::new(RwLock::new(WlDisplay {}))));

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
                //.map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Oops!")));

            //let input_session = futures::future::ok(());
            /*
                        session0
                            .write()
                            .unwrap()
                            .resources
                            .insert(1, Resource::WlDisplay(Arc::new(RwLock::new(WlDisplay {}))));

                        let input_session = reader0.for_each(move |req: Request| {
                            let opt_res = session0
                                .read()
                                .unwrap()
                                .resources
                                .get(&req.sender_object_id)
                                .map(|r| r.clone());
                            let tx = tx0;
                            if let Some(res) = opt_res {
                                let f: Box<Future<Item = (), Error = std::io::Error> + Send> = Box::new(
                                    protocol::resource::dispatch_request(
                                        res,
                                        session0.clone(),
                                        tx,
                                        req.sender_object_id,
                                        req.opcode,
                                        req.args,
                                    )
                                    .map(|_tx| ())
                                    .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Oops!")),
                                );
                                f
                            } else {
                                let f: Box<Future<Item = (), Error = std::io::Error> + Send> = Box::new(
                                    tx.send(Box::new(wl_display::events::Error {
                                        sender_object_id: 1,
                                        object_id: 1,
                                        code: wl_display::enums::Error::InvalidObject as u32,
                                        message: format!(
                                            "object_id={} opcode={} args={:?} not found",
                                            req.sender_object_id, req.opcode, req.args
                                        ),
                                    }))
                                    .map(|_tx| ())
                                    .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Oops!")),
                                );
                                f
                            }
                        });*/
            input_session0
        })
        .map_err(|_| ());
    tokio::run(listener);
    println!("ok");
}
