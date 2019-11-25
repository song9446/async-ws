use async_trait::async_trait;
use futures::Future;
//use futures::channel::oneshot::Canceled;
use futures::channel::oneshot;
use futures::channel::mpsc::{UnboundedSender, UnboundedReceiver};
use log::*;
use slab::Slab;
//use async_trait::async_trait;
pub use async_std::net::SocketAddr;
pub use async_tungstenite::tungstenite::handshake::{headers::Headers, server::{Request, ErrorResponse}};
pub use async_tungstenite::tungstenite::protocol::Message;
pub use async_tungstenite::{accept_hdr_async, WebSocketStream, accept_async};
pub use async_std::net::{TcpListener, TcpStream, ToSocketAddrs};
use async_std::sync::Arc;


pub type WebSocket = WebSocketStream<TcpStream>;

/*
 * copied from actix_web::block
 */
pub fn block<F, R>(f: F) -> impl Future<Output = Result<R, oneshot::Canceled>>
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
        actix_threadpool::run(f)
}

type Token = usize;

enum Event{
    Connection(io::Result<TcpStream>),
    //Message(Token, io::Result<Message>),
    Message(Token),
    Die(Token),
}

struct Session {
    writer: UnboundedSender,
    drop: onshot::Sender,
}

pub async fn run(addr: &str)
{
    let addr = addr.to_socket_addrs().await
        .expect("Not a valid address")
        .next()
        .expect("Not a socket address");
    let listener = TcpListener::bind(&addr).await.unwrap();
    info!("Listening on: {}", addr);

    let mut sessions = Slab::new();

    let (event_sender, event_receiver) = futures::channel::mpsc::unbounded();
    let conns = listener.incoming().map(|stream| Event::Connection(accept_async(stream)))
    let mut events = futures::stream::select(conns, event_receiver);
    loop {
        match events.next().await {
            Some(Event::Connection(Ok(stream)) => {
                let (reader, writer) = futures::stream.split();
                let (drop, drop_rx) = futures::channel::oneshot();
                let token = sessions.insert(Session{
                    writer,
                    drop,
                });
                let event_sender = event_sender.clone();
                async_std::task::spawn(async move {
                    token, drop_rx, reader, event_sender
                });
            },
            Some(Event::Die(token)) => {
                writers.remove(token);
            }
        }
    }
}

async fn read(token: Token, reader: UnboundedReceiver, event_sender: UnboundedSender, drop_rx: oneshot::Receiver) {
    let mut drop = drop_rx.fuse();
    loop {
        let mut buffer: Vec<u8> = vec![0; 1024];
        select! {
            result = reader.read(&mut buffer).fuse() => {
                match result {
                }
            }
            _ = drop => {

            }
        }
    }
}
