use async_std::net::{SocketAddr, TcpListener, TcpStream, ToSocketAddrs};
use async_std::task;
use async_tungstenite::accept_async;
use async_tungstenite::connect_async;
use async_tungstenite::tungstenite::protocol::Message;
use futures::StreamExt;
use async_tungstenite::WebSocketStream;
use async_trait::async_trait;
use log::info;

pub type Socket = WebSocketStream<TcpStream>;

#[async_trait]
pub trait Handler: Sized + Clone + Send {
    async fn on_open(&mut self, ws: &mut Socket); 
    async fn on_message(&mut self, ws: &mut Socket, msg: Message);
    async fn on_close(&mut self);
}


async fn accept_connection<H: Handler>(peer: SocketAddr, stream: TcpStream, mut handler: H) {
    let mut ws_stream = accept_async(stream).await.expect("Failed to accept");

    handler.on_open(&mut ws_stream).await;


    while let Some(msg) = ws_stream.next().await {
        let msg = msg.expect("Failed to get request");
        if msg.is_text() || msg.is_binary() {
            //handler.on_message(ws_stream, msg).await;
            //ws_stream.send(msg).await.expect("Failed to send response");
        }
    }

    //task::spawn(handler.on_close());

    info!("New WebSocket connection: {}", peer);
}

pub async fn run<H: Handler>(handler: H) {
    let addr = "127.0.0.1:9002"
        .to_socket_addrs()
        .await
        .expect("Not a valid address")
        .next()
        .expect("Not a socket address");
    let listener = TcpListener::bind(&addr).await.unwrap();
    info!("Listening on: {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        let peer = stream
            .peer_addr()
            .expect("connected streams should have a peer address");
        info!("Peer address: {}", peer);

        async_std::task::spawn(accept_connection(peer, stream, handler.clone()));
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use crate::run;
        env_logger::init();
        async_std::task::block_on(run());
        assert_eq!(2 + 2, 4);
    }
}
