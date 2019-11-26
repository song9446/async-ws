use thiserror::Error;
use async_std::future::TimeoutError;
use async_tungstenite::tungstenite::error::Error as WsError;

#[derive(Error, Debug)]
pub enum Error {
    //#[error("exceed connection capacity: {0}")]
    //ExceedConnectionCapacity(i32),
    //#[error("exceed connection Capacity")]
    //UserSessionExpired,
    //UserLoginFailed,
    //SessionConflict,
    #[error("connection failed")]
    IoError(#[from] std::io::Error),
    #[error("websocket handshake failed")]
    WebsocketError(#[from] WsError),
    #[error("a client not react for a long time")]
    TimeoutError(#[from] TimeoutError),
    #[error("server drop the client")]
    ServersideDrop,
}
