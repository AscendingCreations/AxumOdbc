use crate::{ODBCConnectionManager, OdbcManagerService};
use tower_layer::Layer;

/// Sessions Layer used with Axum to activate the Service.
///
/// # Examples
/// ```
/// use axum_odbc::{OdbcManagerLayer, ODBCConnectionManager};
///
/// let odbc_manager = ODBCConnectionManager::new("");
/// let layer = OdbcManagerLayer::new(odbc_manager);
/// ```
///
#[derive(Clone)]
pub struct OdbcManagerLayer {
    odbc_manager: ODBCConnectionManager,
}

impl OdbcManagerLayer {
    /// Constructs a OdbcManagerLayer used with Axum to activate the Service.
    ///
    /// # Examples
    /// ```rust
    /// use axum_odbc::{OdbcManagerLayer, ODBCConnectionManager};
    ///
    /// let odbc_manager = ODBCConnectionManager::new("");
    /// let layer = OdbcManagerLayer::new(odbc_manager);
    /// ```
    ///
    pub fn new(odbc_manager: ODBCConnectionManager) -> Self {
        OdbcManagerLayer { odbc_manager }
    }
}

impl<S> Layer<S> for OdbcManagerLayer {
    type Service = OdbcManagerService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        OdbcManagerService {
            odbc_manager: self.odbc_manager.clone(),
            inner,
        }
    }
}
