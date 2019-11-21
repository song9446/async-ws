use std::collections::HashMap;
use std::io::{self, Read, Write};
use std::str::from_utf8;

use mio::event::Event;
use mio::net::{TcpListener, TcpStream};
use mio::{Events, Interests, Poll, Registry, Token};

use tungstenite::accept_hdr;
use tungstenite::WebSocket;
use tungstenite::handshake::server::Request;

use tungstenite::Message;

use slab::Slab;

const SERVER: Token = Token(std::usize::MAX);

fn ws_accept(stream: TcpStream) -> WebSocket<TcpStream> {
    let callback = |req: &Request| {
                println!("Received a new ws handshake");
                println!("The request's path is: {}", req.path);
                println!("The request's headers are:");
                for &(ref header, _ /* value */) in req.headers.iter() {
                    println!("* {}", header);
                }

                // Let's add an additional header to our response to the client.
                let extra_headers = vec![
                    (String::from("MyCustomHeader"), String::from(":)")),
                    (
                        String::from("SOME_TUNGSTENITE_HEADER"),
                        String::from("header_value"),
                    ),
                ];
                Ok(Some(extra_headers))
            };
    accept_hdr(stream, callback).unwrap()
}

enum Stream {
    RawStream(TcpStream),
    WebSocketStream(WebSocket<TcpStream>),
}
impl Stream {
    fn get_ref(&self) -> &TcpStream {
        match self {
            Stream::RawStream(s) => &s,
            Stream::WebSocketStream(s) => s.get_ref(),
        }
    }
    fn as_raw(self) -> TcpStream {
        match self {
            Stream::RawStream(s) => s,
            _ => panic!("Upgraded stream cannot converted to raw stream"),
        }
    }
    fn is_raw(&self) -> bool {
        match &self {
            Stream::RawStream(_) => true,
            _ => false,
        }
    }
    fn read_message(&mut self) -> Result<Message, tungstenite::error::Error> {
        match self {
            Stream::RawStream(_) => panic!("Upgraded stream cannot converted to raw stream"),
            Stream::WebSocketStream(s) => s.read_message(),
        }
    }
}

pub fn run(addr: &str) -> io::Result<()> {
    let mut poll = Poll::new()?;
    let mut events = Events::with_capacity(128);
    let mut conns = Slab::new();

    let server = TcpListener::bind(addr.parse().expect("Invalid addr representation"))?;

    poll.registry()
        .register(&server, SERVER, Interests::READABLE)?;

    loop {
        poll.poll(&mut events, None)?;
        for event in &events {
            match event.token() {
                SERVER => {
                    let (conn, addr) = server.accept()?;
                    println!("Accepted connection from: {}", addr);
                    let key = conns.insert(Stream::RawStream(conn));
                    poll.registry().register(
                        conns[key].get_ref(),
                        Token(key),
                        Interests::READABLE.add(Interests::WRITABLE),
                    )?;
                }
                token => {
                    if event.is_readable() {
                        if conns[token.0].is_raw() {
                            let ws = Stream::WebSocketStream(ws_accept(conns.remove(token.0).as_raw()));
                            let key = conns.insert(ws);
                            poll.registry().reregister(
                                conns[key].get_ref(), 
                                Token(key),
                                Interests::READABLE.add(Interests::WRITABLE),
                            )?;
                                
                        } else {
                            println!("msg: {}", conns[token.0].read_message().unwrap());
                        }
                    }
                }
            }
        }
    }

}
