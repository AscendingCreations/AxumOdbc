use crate::pool::ODBCConnection;
use crate::OdbcError;
use crate::ENV;
use crossbeam_queue::ArrayQueue;
pub use odbc_api::*;
use std::sync::{Arc, Mutex, RwLock};
use tokio::task;

pub(crate) struct SharedPool {
    pub(crate) pool: Mutex<ArrayQueue<Connection<'static>>>,
    pub(crate) connection_string: RwLock<String>,
}

impl SharedPool {
    pub(crate) fn new_arc(connection_string: String, limit: u32) -> Arc<Self> {
        let pool = Self {
            pool: Mutex::new(ArrayQueue::new(limit as usize)),
            connection_string: RwLock::new(connection_string),
        };

        Arc::new(pool)
    }

    pub(crate) async fn aquire(self: Arc<Self>) -> Result<ODBCConnection, OdbcError> {
        task::spawn_blocking(move || {
            let pool = self
                .pool
                .lock()
                .map_err(|p| OdbcError::LockError { msg: p.to_string() })?;

            if let Some(connection) = pool.pop() {
                return Ok(ODBCConnection::new(Arc::clone(&self), connection));
            }

            let conn_str = self
                .connection_string
                .read()
                .map_err(|p| OdbcError::LockError { msg: p.to_string() })?;

            let env = &ENV;
            let conn = env
                .connect_with_connection_string(conn_str.as_str(), ConnectionOptions::default())?;

            Ok(ODBCConnection::new(Arc::clone(&self), conn))
        })
        .await?
    }
}
