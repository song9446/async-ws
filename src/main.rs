#![recursion_limit="256"]

use futures::StreamExt;
use futures::Future;

mod error;
mod server;
#[allow(dead_code, unused_imports)]
mod message_generated;

fn main() {
    env_logger::init();
    let mut server = server::Server::new();
    async_std::task::block_on(server.run("127.0.0.1:3030"));
}
