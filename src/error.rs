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
    ConnectError(#[from] ConnectError),
    #[error("websocket read failed")]
    ReadError(#[from] ReadError),
}

#[derive(Error, Debug)]
pub enum ReadError {
    #[error("websocket read failed")]
    WsError(#[from] WsError),
    #[error("timeout no response from client")]
    TimeoutError(#[from] TimeoutError),
}

#[derive(Error, Debug)]
pub enum ConnectError {
    #[error("tcp listen failed")]
    IoError(#[from] std::io::Error),
    #[error("websocket handshake failed")]
    WsError(#[from] WsError),
}
