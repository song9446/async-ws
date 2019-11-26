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

use error::{Error};

use message_generated::Message as GameMessage;
use message_generated::get_root_as_message;

pub fn block<F, R>(f: F) -> impl Future<Output = Result<R, oneshot::Canceled>>
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
        actix_threadpool::run(f)
}

pub type WebSocket = WebSocketStream<TcpStream>;
type Token = usize;

enum Event{
    Connect(TcpStream),
    WebsocketUpgraded{
        token: Token, 
        writer: SplitSink<WebSocket, WsMessage>,
    },
    Message{
        token: Token,
        message: WsMessage,
    },
    Die{
        token: Token, 
        reason: Option<Error>
    },
}

struct Session {
    writer: Option<SplitSink<WebSocket, WsMessage>>,
    drop: Option<oneshot::Sender<()>>,
}
pub struct Server {
    sessions: Slab::<Session>,
    capacity: usize,
    timeout: Duration,
}
impl<'a> Server {
    pub fn new() -> Self {
        Server {
            sessions: Slab::new(),
            capacity: std::usize::MAX,
            timeout: Duration::from_secs(60*60),
        }
    }
    pub fn with_capacity(mut self, capacity: usize) -> Self {
        self.capacity = capacity;
        self
    }
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }
    async fn on_message(&mut self, token: Token, message: WsMessage, event_sink: UnboundedSender<Result<Event, Error>>) {
        let message = message.into_data();
        let parsed_message = get_root_as_message(&message);
        //event_sink.unbounded_send();
    }
    pub async fn run(mut self, addr: &str) {
        let addr = addr.to_socket_addrs().await
            .expect("Not a valid address")
            .next()
            .expect("Not a socket address");
        let listener = TcpListener::bind(&addr).await.unwrap();
        info!("Listening on: {}", addr);

        let (event_sink, event_receiver) = unbounded();
        let conns = listener.incoming()
            .err_into::<Error>()
            .map_ok(|stream| Event::Connect(stream));
        let mut events = futures::stream::select(conns, event_receiver);
        //let mut events = futures::stream::select_all()
        loop {
            match events.next().await {
                Some(Ok(Event::Connect(stream))) => {
                    if self.capacity <= self.sessions.len() {
                        stream.shutdown(std::net::Shutdown::Both);
                        println!("shutdown due to no capacity");
                        continue;
                    }
                    let (drop_flag, drop) = oneshot::channel();
                    //let (writer, reader) = stream.split();
                    let token = self.sessions.insert(Session{
                        writer: None,
                        drop: Some(drop_flag),
                    });
                    task::spawn(Self::read(token, stream, event_sink.clone(), drop, self.timeout));
                }
                Some(Ok(Event::WebsocketUpgraded{token, writer})) => {
                    self.sessions[token].writer = Some(writer);
                }
                Some(Ok(Event::Die{token, reason})) => {
                    self.sessions.remove(token);
                }
                Some(Ok(Event::Message{token, message})) => {
                    self.on_message(token, message, event_sink.clone()).await;
                }
                Some(Err(err)) => {
                    println!("{:?}", err);
                }
                _ => panic!("tcp listener die")
            }
        }
    }
    async fn read(token: Token, mut stream: TcpStream, mut event_sink: UnboundedSender<Result<Event, Error>>, drop: oneshot::Receiver<()>, timeout: Duration) {
        let mut stream = accept_async(stream).await.unwrap();
        let (writer, mut reader) = stream.split();
        event_sink.unbounded_send(Ok(Event::WebsocketUpgraded{token, writer}));
        let mut drop = drop.fuse();
        //Self::on_open(token, &mut event_sink).await;
        loop {
            select! {
                _ = drop => {
                    event_sink.unbounded_send(Ok(Event::Die{token, reason: Some(Error::ServersideDrop)}));
                    break;
                }
                res = async_std::future::timeout(timeout, reader.next()).fuse() => match res{
                    Ok(Some(Ok(message))) if message.len() > 0 => {
                        event_sink.unbounded_send(Ok(Event::Message{token, message}));
                        //Self::codec(token, msg, &mut stream, &mut event_sink).await;
                    }
                    Ok(Some(Err(err))) => {
                        event_sink.unbounded_send(Ok(Event::Die{token, reason: Some(err.into())}));
                        break;
                    }
                    Err(err) => {
                        event_sink.unbounded_send(Ok(Event::Die{token, reason: Some(err.into())}));
                        break;
                    }
                    Ok(Some(Ok(_))) | Ok(None) => {
                        event_sink.unbounded_send(Ok(Event::Die{token, reason: None}));
                        break;
                    }
                }
            }
        }
    }
    /*async fn on_open(token: Token, event_sink: &mut UnboundedSender<Result<Event, Error>>) {
    }
    async fn on_message(token: Token, msg: WsMessage, stream: &mut WebSocket, event_sink: &mut UnboundedSender<Result<Event, Error>>) {
        //stream.send(msg).await;
        //stream.send();
        /*block(move ||{
            println!("{}: {}", token ,msg);
        }).await;*/
    }*/
}
