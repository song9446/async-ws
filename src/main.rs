use futures::StreamExt;
use futures::Future;

mod error;
mod server;
//use crate::server::*;
//use core::future::Future;

fn main() {
    env_logger::init();
	//let mut h = Handler::new();
    async_std::task::block_on(server::run("127.0.0.1:3030"));
}
