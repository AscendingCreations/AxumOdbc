use crate::{
    pool::{ODBCConnection, SharedPool},
    OdbcError,
};
use axum_core::extract::{FromRef, FromRequestParts};
use http::request::Parts;
use odbc_api::Environment;
use once_cell::sync::Lazy;
use std::{convert::Infallible, fmt, sync::Arc};

#[derive(Clone)]
pub struct ODBCConnectionManager {
    pub(crate) shared: Arc<SharedPool>,
}

pub(crate) static ENV: Lazy<Environment> = Lazy::new(|| Environment::new().unwrap());

impl ODBCConnectionManager {
    /// Constructs a ODBCConnectionManager.
    ///
    /// limit is the max size of how many connections we will allow to exist within the pool.
    /// All other connections will get dropped after use if the pool is full.
    ///
    /// # Examples
    /// ```rust no_run
    /// use axum_odbc::{OdbcManagerLayer, ODBCConnectionManager};
    ///
    /// let odbc_manager = ODBCConnectionManager::new("DSN=PostgreSQL", 5);
    /// ```
    ///
    pub fn new<S: Into<String>>(connection_string: S, limit: u32) -> ODBCConnectionManager {
        ODBCConnectionManager {
            shared: SharedPool::new_arc(connection_string.into(), limit),
        }
    }

    /// Aquires an ODBCConnection.
    ///
    /// # Examples
    /// ```rust no_run
    /// use axum_odbc::{OdbcManagerLayer, ODBCConnectionManager};
    ///
    /// let odbc_manager = odbc_manager.aquire().await.unwrap();
    /// ```
    ///
    pub async fn aquire(&self) -> Result<ODBCConnection, OdbcError> {
        let shared = Arc::clone(&self.shared);
        shared.aquire().await
    }
}

impl<S> FromRequestParts<S> for ODBCConnectionManager
where
    S: Send + Sync,
    ODBCConnectionManager: FromRef<S>,
{
    type Rejection = Infallible;

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        Ok(ODBCConnectionManager::from_ref(state))
    }
}

impl fmt::Debug for ODBCConnectionManager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ODBCConnectionManager").finish()
    }
}
