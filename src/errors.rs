use thiserror::Error;

#[derive(Error, Debug)]
pub enum OdbcError {
    #[error(transparent)]
    IO(#[from] std::io::Error),
    #[error(transparent)]
    OdbcApi(#[from] odbc_api::Error),
    #[error("Lock is poisoned {msg} ")]
    LockError { msg: String },
    #[error(transparent)]
    SerdeJson(#[from] serde_json::error::Error),
    #[error(transparent)]
    HTTP(#[from] http::Error),
    #[error(transparent)]
    TokioJoin(#[from] tokio::task::JoinError),
    #[error("unknown error")]
    Unknown,
}
