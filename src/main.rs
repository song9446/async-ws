/*
struct Test;

impl Handler for Test {
    async fn on_open(&mut self, ws: &mut Socket) {
        info!("New WebSocket connection");
    }
    async fn on_message(&mut self, ws: &mut Socket, msg: Message) {
        ws.send(msg).await.expect("Failed to send response");
    }
    async fn on_close(&mut self) {
        info!("WebSocket closed");
    }
}*/
mod lib;
use crate::lib::run;
fn main() {
    env_logger::init();
    run("127.0.0.1:3000");
}
