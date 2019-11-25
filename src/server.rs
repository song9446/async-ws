use std::time::{Duration, Instant};
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

use async_tungstenite::tungstenite::handshake::{headers::Headers, server::{Request, ErrorResponse}};
use async_tungstenite::tungstenite::protocol::Message as WsMessage;
use async_tungstenite::{accept_hdr_async, WebSocketStream, accept_async};

use async_std::net::{SocketAddr, TcpListener, TcpStream, ToSocketAddrs};
use async_std::task;
use async_trait::async_trait;

use log::*;
use slab::Slab;

use crate::*;

use error::{Error, ConnectError, ReadError};

pub fn block<F, R>(f: F) -> impl Future<Output = Result<R, oneshot::Canceled>>
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
        actix_threadpool::run(f)
}

pub type WebSocket = WebSocketStream<TcpStream>;
pub type EventSender<'a> = UnboundedSender<Result<Event<'a>, Error>>;
type Token = usize;

enum Event<'a>{
    Connect(TcpStream),
    Message{
        token: Token,
        message: message_generated::Message<'a>,
    },
    Die{
        token: Token, 
        reason: Option<ReadError>
    },
}

struct Session {
    writer: SplitSink<WebSocket, WsMessage>,
    drop: oneshot::Sender<()>,
}

pub struct Server {
    sessions: Slab::<Session>,
    capacity: usize,
    timeout: Duration,
}
impl Server {
    pub fn new() -> Self {
        Server {
            sessions: Slab::new(),
            capacity: std::usize::MAX,
            timeout: Duration::from_secs(60*60)
        }
    }
    pub async fn run(mut self, addr: &str) {
        let addr = addr.to_socket_addrs().await
            .expect("Not a valid address")
            .next()
            .expect("Not a socket address");
        let listener = TcpListener::bind(&addr).await.unwrap();
        info!("Listening on: {}", addr);

        let (event_sender, event_receiver) = unbounded();
        let conns = listener.incoming()
            .err_into::<ConnectError>()
            .err_into::<Error>()
            .map_ok(|stream| Event::Connect(stream));
        let mut events = futures::stream::select(conns, event_receiver);
        loop {
            match events.next().await {
                Some(Ok(Event::Connect(stream))) => {
                    let stream = accept_async(stream).await.unwrap();
                    let (writer, reader) = stream.split();
                    let (drop, drop_rx) = oneshot::channel();
                    let token = self.sessions.insert(Session{
                        writer,
                        drop,
                    });
                    let event_sender = event_sender.clone();
                    task::spawn(Self::read(token, reader, event_sender, drop_rx, self.timeout));
                },
                Some(Ok(Event::Die{token, reason})) => {
                    self.sessions.remove(token);
                    println!("{} die, due to {:?}", token, reason);
                }
                Some(Ok(Event::Message{token, message})) => {
                    self.sessions.remove(token);
                }
                Some(Err(err)) => {
                    println!("{:?}", err);
                }
                _ => panic!("tcp listener die")
            }
        }
    }
    async fn read(token: Token, mut reader: SplitStream<WebSocket>, mut event_sender: EventSender<'_>, drop_rx: oneshot::Receiver<()>, timeout: Duration) {
        let mut drop = drop_rx.fuse();
        Self::on_open(token, &mut event_sender).await;
        loop {
            select! {
                res = async_std::future::timeout(timeout, reader.next()).fuse() => {
                    match res{
                        Ok(Some(Ok(msg))) if msg.len() > 0 => {
                            Self::on_message(token, msg, &mut event_sender).await;
                        }
                        Ok(Some(Err(err))) => {
                            event_sender.send(Ok(Event::Die{token, reason: Some(err.into())})).await;
                            return;
                        }
                        Err(err) => {
                            event_sender.send(Ok(Event::Die{token, reason: Some(err.into())})).await;
                            return;
                        }
                        Ok(Some(Ok(_))) | Ok(None) => {
                            event_sender.send(Ok(Event::Die{token, reason: None})).await;
                            return;
                        }
                    }
                }
                _ = drop => {
                    println!("drop {}", token);
                    return;
                }
            }
        }
    }
    async fn on_open(token: Token, event_sender: &mut EventSender<'_>) {
        println!("hi {}", token);
    }
    async fn on_message(token: Token, msg: WsMessage, event_sender: &mut EventSender<'_>) {
        block(move ||{
            println!("{}: {}", token ,msg);
        }).await;
    }
}
