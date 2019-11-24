use async_trait::async_trait;
use futures::StreamExt;
use futures::Future;
use futures::channel::oneshot::Canceled;
use log::*;
use slab::Slab;
//use async_trait::async_trait;
pub use async_std::net::SocketAddr;
pub use async_tungstenite::tungstenite::handshake::{headers::Headers, server::{Request, ErrorResponse}};
pub use async_tungstenite::tungstenite::protocol::Message;
pub use async_tungstenite::{accept_hdr_async, WebSocketStream, accept_async};
pub use async_std::net::{TcpListener, TcpStream, ToSocketAddrs};
use async_std::sync::Arc;
use futures::lock::{Mutex, MutexGuard};



/*
 * copied from actix_web::block
 */
pub type WebSocket = WebSocketStream<TcpStream>;

pub fn block<F, R>(f: F) -> impl Future<Output = Result<R, Canceled>>
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
        actix_threadpool::run(f)
}

pub struct Handler {
	conns: Arc<Slab<WebSocket>>,
}
impl Handler {
	pub fn new() -> Self {
		Handler{ 
			conns: Arc::new(Slab::new()) 
		}
	}
	pub async fn run(&self, addr: &str){
		let addr = addr.to_socket_addrs().await
			.expect("Not a valid address")
			.next()
			.expect("Not a socket address");
		let listener = TcpListener::bind(&addr).await.unwrap();
		info!("Listening on: {}", addr);
		while let Ok((stream, _)) = listener.accept().await {
			let peer = stream
				.peer_addr()
				.expect("connected streams should have a peer address");
			let evloop = async move {
				let mut ws = accept_hdr_async(stream, |req: &Request|{
					Ok(None)
				}).await.expect("handshake error");
				while let Some(msg) = ws.next().await {
					let msg = msg.expect("Failed to get request");
					if msg.is_text() || msg.is_binary() {
						//self.on_message(msg).await
						ws.send(msg).await.expect("Failed to send response");
					}
				}
			};
			async_std::task::spawn(evloop);
		}
	}
}

#[async_trait]
pub trait Handler2 {
    async fn on_message<'e>(&self, msg: Message, mut ws: MutexGuard<'e, WebSocket>) {
    }
    async fn on_message2(&self) {
    }
    async fn run(&self, addr: &str) {
		let addr = addr.to_socket_addrs().await
			.expect("Not a valid address")
			.next()
			.expect("Not a socket address");
		let listener = TcpListener::bind(&addr).await.unwrap();
		info!("Listening on: {}", addr);
        let evloops = futures::stream::FuturesUnordred::new();
		while let Ok((stream, _)) = listener.accept().await {
			let peer = stream
				.peer_addr()
				.expect("connected streams should have a peer address");
			let evloop = async move {
				let mut ws = accept_hdr_async(stream, |req: &Request|{
					Ok(None)
				}).await.expect("handshake error");
                let mut ws = Arc::new(Mutex::new(ws));
				while let Some(msg) = ws.lock().await.next().await {
					let msg = msg.expect("Failed to get request");
                    //self.on_message(msg, ws.clone().lock().await).await
                    //self.on_message2().await
					/*if msg.is_text() || msg.is_binary() {
						//self.on_message(msg).await
						//ws.send(msg).await.expect("Failed to send response");
					}*/
				}
			};
            evloops.push(evloop);
			//async_std::task::spawn(evloop);
		}
	}
}

/*pub async fn run<H, F, R>(addr: &str, handler: H)
where H: Fn(TcpStream) -> F,
      F: Future<Output=R> + Send + 'static,
      R: Send + 'static,*/
      //C: Fn(&Request) -> Result<Option<Vec<(String, String)>>, ErrorResponse> + Sync + Send + Unpin + 'static,
/*
pub async fn run<H, F, R>(addr: &str, handler: H)
where H: Fn(TcpStream) -> F,
      F: Future<Output=R> + Send + 'static,
      R: Send + 'static,*/
