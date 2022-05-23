use crate::ODBCConnectionManager;
use crate::SharedPool;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

pub type Connection = odbc_api::force_send_sync::Send<odbc_api::Connection<'static>>;
const DEREF_ERR: &str = "Connection already released to pool";

pub struct ODBCConnection {
    pub connection: Option<Connection>,
    pub(crate) pool: Option<Arc<SharedPool>>,
}

impl ODBCConnection {
    pub(crate) fn new(pool: Arc<SharedPool>, connection: Connection) -> Self {
        Self {
            connection: Some(connection),
            pool: Some(pool),
        }
    }

    pub fn detach(mut self) -> Connection {
        self.connection
            .take()
            .expect("PoolConnection double-dropped")
    }
}

impl Drop for ODBCConnection {
    fn drop(&mut self) {
        if self.connection.is_some() && self.pool.is_some() {
            let shared = self.pool.take().unwrap();
            let connection = self.connection.take().unwrap();
            let pool = shared.pool.lock().unwrap();

            if pool.len() < pool.capacity() {
                // pool has space lets insert it back into the Pool
                match pool.push(connection) {
                    Ok(_) => {}
                    Err(_) => panic!("Queue was maxed out"),
                };
            }
        }
    }
}

impl Deref for ODBCConnection {
    type Target = Connection;

    fn deref(&self) -> &Self::Target {
        self.connection.as_ref().expect(DEREF_ERR)
    }
}

impl DerefMut for ODBCConnection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.connection.as_mut().expect(DEREF_ERR)
    }
}

impl AsRef<Connection> for ODBCConnection {
    fn as_ref(&self) -> &Connection {
        self
    }
}

impl AsMut<Connection> for ODBCConnection {
    fn as_mut(&mut self) -> &mut Connection {
        self
    }
}

pub trait AttachODBC {
    fn attach(self, manager: ODBCConnectionManager) -> ODBCConnection;
}

impl AttachODBC for Connection {
    fn attach(self, manager: ODBCConnectionManager) -> ODBCConnection {
        ODBCConnection::new(Arc::clone(&manager.shared), self)
    }
}
