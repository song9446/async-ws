use thiserror::Error;
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
    ConnectError(#[from] ConnectError)
}

/*pub enum Error {
}*/
#[derive(Error, Debug)]
pub enum ConnectError {
    #[error("tcp listen failed")]
    IoError(#[from] std::io::Error),
    #[error("websocket handshake failed")]
    WsError(#[from] WsError),
}
