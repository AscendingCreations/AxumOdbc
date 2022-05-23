use crate::pool::{ODBCConnection, SharedPool};
use async_trait::async_trait;
use axum_core::extract::{FromRequest, RequestParts};
use http::{self, StatusCode};
use lazy_static::lazy_static;
use odbc_api::Environment;
use std::fmt;
use std::sync::Arc;

#[derive(Clone)]
pub struct ODBCConnectionManager {
    pub(crate) shared: Arc<SharedPool>,
}

lazy_static! {
    pub static ref ENV: Environment = Environment::new().unwrap();
}

impl ODBCConnectionManager {
    /// Creates a new `ODBCConnectionManager`.
    pub fn new<S: Into<String>>(connection_string: S, limit: u32) -> ODBCConnectionManager {
        ODBCConnectionManager {
            shared: SharedPool::new_arc(connection_string.into(), limit),
        }
    }

    pub async fn aquire(&self) -> ODBCConnection {
        self.shared.aquire().await.unwrap()
    }
}

#[async_trait]
impl<B> FromRequest<B> for ODBCConnectionManager
where
    B: Send,
{
    type Rejection = (http::StatusCode, &'static str);

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        req.extensions()
            .get::<ODBCConnectionManager>()
            .cloned()
            .ok_or((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Can't extract ODBCConnectionManager. Is `ODBCManagerLayer` enabled?",
            ))
    }
}

impl fmt::Debug for ODBCConnectionManager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ODBCConnectionManager").finish()
    }
}
