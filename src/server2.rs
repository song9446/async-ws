use std::time::{Duration, Instant};
use futures::{
    prelude::*,
    select,
    pin_mut,
    Future, 
    StreamExt, 
    SinkExt, 
    Stream, 
    stream::{SplitStream, SplitSink, SelectAll},
    io::ReadHalf,
    io::WriteHalf,
    channel::{
        oneshot, 
        mpsc::{unbounded, UnboundedSender, UnboundedReceiver}}};

use async_tungstenite::tungstenite::handshake::{headers::Headers, server::{Request, ErrorResponse}};
use async_tungstenite::tungstenite::protocol::Message;
use async_tungstenite::{accept_hdr_async, WebSocketStream, accept_async};
use async_tungstenite::tungstenite::error::Error as WsError;

use async_std::net::{SocketAddr, TcpListener, TcpStream, ToSocketAddrs};
use async_std::task;
use async_trait::async_trait;

use log::*;
use slab::Slab;

use crate::*;

use error::{Error};

enum Event {
    Connected(TcpStream),
    WebsocketUpgraded(Result<WebSocketStream, WsError>),
    Die(Token),
}

pub fn block<F, R>(f: F) -> impl Future<Output = Result<R, oneshot::Canceled>>
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
        actix_threadpool::run(f)
}

pub type WebSocket = WebSocketStream<TcpStream>;
type Token = usize;

pub async fn run(addr: &str, capacity: usize) {
    let (message_sender, message_receiver) = unbounded();
    let (event_sender, event_receiver) = unbounded();
    task::spawn(accept_loop(addr, event_sender.clone()));
    let sockets = Slab::with_capacity(capacity);
    while let Some(event) = event_receiver.read().await {
        match event {
            Event::Connected(stream) => {
                if sockets.len() >= capacity {
                    stream.shutdown(std::net::Shutdown::Both);
                } 
            }
            Event::WebsocketUpgraded(Ok(stream)) => {
                let (writer, reader) = stream.split();
                let token = sockets.insert(stream);
                listen(reader, event_sender, event_receiver, event_sender.clone())
            }
            Event::WebsocketUpgraded(Err(err)) => {
            }
        }
    }
}

pub async fn accept_loop(addr: &str, event_sender: UnboundedReceiver<Event>) {
    let addr = addr.to_socket_addrs().await
        .expect("Not a valid address")
        .next()
        .expect("Not a socket address");
    let listener = TcpListener::bind(&addr).await.unwrap();
    println!("Listening on: {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        task::spawn(websocket_upgrade(stream, event_sender.clone()));
    }
}

pub async fn websocket_upgrade(stream: TcpStream, event_sender: UnboundedReceiver<Event>) {
    event_sender.send(Event::WebsocketUpgraded(accept_async(stream).await));
}

//    stream::{SplitStream, SplitSink},
pub async fn listen(stream: SplitStream<Message>, event_sender: UnboundedReceiver<Event>) {
    while let Some(msg) = ws_stream.next().await {
        let msg = msg.expect("Failed to get request");
        if msg.is_text() || msg.is_binary() {
            ws_stream.send(msg).await.expect("Failed to send response");
            println!("0: {}", peer);
        }
    }
}

pub async fn runs() {
}
