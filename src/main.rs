mod lib;
use crate::lib::*;
use futures::StreamExt;
use futures::Future;
//use core::future::Future;

fn main() {
    env_logger::init();
	//let mut h = Handler::new();
    async_std::task::block_on(run("127.0.0.1:3030", |ws| async move {
    }));
}
