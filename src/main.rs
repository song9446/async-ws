struct Test;
#[async_trait]
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
}
fn main() {
    use crate::run;
    env_logger::init();
    async_std::task::block_on(run());
}
