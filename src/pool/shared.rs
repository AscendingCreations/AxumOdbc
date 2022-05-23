use crate::pool::ODBCConnection;
use crate::OdbcError;
use crate::ENV;
use crossbeam_queue::ArrayQueue;
pub use odbc_api::*;
use std::sync::{Arc, Mutex, RwLock};
use tokio::task;

pub(crate) struct SharedPool {
    pub(crate) pool: Mutex<ArrayQueue<odbc_api::force_send_sync::Send<Connection<'static>>>>,
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

    pub(crate) async fn aquire(self: &Arc<Self>) -> Result<ODBCConnection, OdbcError> {
        let manager = Arc::clone(self);

        task::spawn_blocking(move || {
            let pool = manager
                .pool
                .lock()
                .map_err(|p| OdbcError::LockError { msg: p.to_string() })?;

            if let Some(connection) = pool.pop() {
                return Ok(ODBCConnection::new(Arc::clone(&manager), connection));
            }

            let conn_str = manager
                .connection_string
                .read()
                .map_err(|p| OdbcError::LockError { msg: p.to_string() })?;

            let env = &ENV;
            let conn = env.connect_with_connection_string(conn_str.as_str())?;
            // Promoting a connection to send is unsafe, since not every ODBC driver is thread safe.
            // Actual thread safety for unixODBC may also depend on the threading level defined for the
            // ODBC driver. Here we assume that the user conciously checked the safety of the
            // application and checked into sending connection then the ODBConnectionManager has been
            // constructed.
            // we will attempt to bypass this by handling the pool ourselves.
            let connection = unsafe { conn.promote_to_send() };

            Ok(ODBCConnection::new(Arc::clone(&manager), connection))
        })
        .await?
    }
}
