use futures::{
    prelude::*,
    select,
    pin_mut,
    Future, 
    StreamExt, 
    SinkExt, 
    Stream, 
    stream::{SplitStream, SplitSink},
    io::ReadHalf,
    io::WriteHalf,
    channel::{
        oneshot, 
        mpsc::{unbounded, UnboundedSender, UnboundedReceiver}}};

pub use async_tungstenite::tungstenite::handshake::{headers::Headers, server::{Request, ErrorResponse}};
pub use async_tungstenite::tungstenite::protocol::Message;
pub use async_tungstenite::{accept_hdr_async, WebSocketStream, accept_async};
pub use async_std::net::{SocketAddr, TcpListener, TcpStream, ToSocketAddrs};
use async_trait::async_trait;

use log::*;
use slab::Slab;

use crate::*;

use error::{Error, ConnectError};


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
    Connect(TcpStream),
    Message(Token),
    Die(Token),
}

struct Session {
    writer: SplitSink<WebSocket, Message>,
    drop: oneshot::Sender<()>,
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

    let (event_sender, event_receiver) = unbounded();
    let conns = listener.incoming()
        .err_into::<ConnectError>()
        .err_into::<Error>()
        .map_ok(|stream| Event::Connect(stream));
    let mut events = futures::stream::select(conns, event_receiver);
    //let mut events = conns;
    //let mut events = event_receiver;
    loop {
        match events.next().await {
            Some(Ok(Event::Connect(stream))) => {
                let stream = accept_async(stream).await.unwrap();
                let (writer, reader) = stream.split();
                let (drop, drop_rx) = oneshot::channel();
                let token = sessions.insert(Session{
                    writer,
                    drop,
                });
                let event_sender = event_sender.clone();
                async_std::task::spawn(read(token, reader, event_sender, drop_rx));
            },
            Some(Ok(Event::Die(token))) => {
                sessions.remove(token);
            }
            Some(Err(err)) => {
                print!("{:?}", err);
            }
            _ => panic!("tcp listener die")
        }
    }
}

async fn read(token: Token, mut reader: SplitStream<WebSocket>, mut event_sender: UnboundedSender<Result<Event, Error>>, drop_rx: oneshot::Receiver<()>) {
    let mut drop = drop_rx.fuse();
    loop {
        select! {
            result = reader.next().fuse() => {
                match result {
                    Some(Ok(msg)) => {
                        println!("{}", msg)
                    }
                    Some(Err(err)) => {
                    }
                    None => {
                    }
                }
            }
            _ = drop => {
                return;
            }
        }
    }
}
