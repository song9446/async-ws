use core::pin::Pin;
use std::time::{Duration, Instant};
use futures::{
    prelude::*,
    select,
    pin_mut,
    Future, 
    StreamExt, 
    SinkExt, 
    Stream, 
    stream::{pending, SplitStream, SplitSink, SelectAll, futures_unordered::FuturesUnordered},
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

pub type WebSocket = WebSocketStream<TcpStream>;
type Token = usize;

pub async fn run(addr: &str, capacity: usize) {
    let addr = addr.to_socket_addrs().await.expect("Not a valid address").next().expect("Not a socket address");
    let listener = TcpListener::bind(&addr).await.unwrap();
    info!("Listening on: {}", addr);

    let (socket_sender, socket_receiver) = unbounded();
    task::spawn(event_loop(socket_receiver, capacity));
    while let Ok((stream, _)) = listener.accept().await {
        task::spawn(websocket_upgrade(stream, socket_sender.clone()));
    }
}
pub async fn websocket_upgrade(stream: TcpStream, mut socket_sender: UnboundedSender<WebSocket>) {
    if let Ok(ws) = accept_async(stream).await {
        println!("connection established");
        socket_sender.send(ws);
    } else {
        println!("handshake fail");
    }
}

struct Session {
    writer: SplitSink<WebSocket, Message>,
}
pub async fn event_loop(mut socket_receiver: UnboundedReceiver<WebSocket>, capacity: usize) {
    let mut sessions = Slab::new();
    //let mut readers = SelectAll::new();
    //let events = FuturesUnordered::new();
    //readers.push(pending());
    let (message_stream, message_sink) = unbounded();
    loop {
        select! {
            res = socket_receiver.next().fuse() => match res {
                Some(ws) => {
                    println!("connection come to evloop");
                    let (writer, reader) = ws.split();
                    let token = sessions.insert(Session{
                        writer,
                    });
                    //let reader = reader.map(move |msg| (token, msg));
                    task::spawn(read_message(token, reader, message_sink));
                },
                None => panic!("Tcp listen exited"),
            },
            res = message_stream.next().fuse() => match res {
                Some((token, Ok(msg))) if msg.len() > 0 => {
                    println!("{}: {}", token, msg);
                },
                Some((token, Ok(_))) => {
                    println!("{}: might quit?", token);
                },
                Some((token, Err(err))) => {
                    println!("{}: {:?}", token, err);
                },
                None => continue, //println!("no readers available"),
            }
        }
    }
}

pub async fn read_message(token: Token, mut reader: SplitSink<WebSocket, Message>, mut writer: UnboundedSender<(Token, Message)>) {
    loop {
        match async_std::future::timeout(Duration::from_secs(3600), reader.next()).await {
            Ok(Some(Ok(msg))) if msg.len() > 0 => {
                reader.send((token, msg)).await;
            }
            Ok(Some(Err(err))) => {
                writer.send((token, Message::close())).await;
                return;
            }
            Err(err) => {
                writer.send((token, Message::close())).await;
                return;
            }
            Ok(Some(Ok(_))) | Ok(None) => {
                print!("h4");
                return;
            }
        }}
}

pub fn block<F, R>(f: F) -> impl Future<Output = Result<R, oneshot::Canceled>>
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
        actix_threadpool::run(f)
}
