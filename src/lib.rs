#![doc = include_str!("../README.md")]
#![allow(dead_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]

mod errors;
mod layer;
mod manager;
mod pool;
mod service;

pub(crate) use manager::ENV;
pub(crate) use pool::SharedPool;
pub(crate) use service::OdbcManagerService;

pub use errors::OdbcError;
pub use layer::OdbcManagerLayer;
pub use manager::ODBCConnectionManager;
pub use pool::ODBCConnection;
pub use tokio::{self, task};

/// Block non async closure or functions so it can run within async.
///
/// # Examples
/// ```rust no_run
///
/// let connection = manager.aquire().await?;
///
/// let _ = blocking!(connection.execute("DROP TABLE IF EXISTS TEST", ()))?;
/// ```
///
#[macro_export]
macro_rules! blocking {
    ($($expr:tt)*) => {
        $crate::tokio::task::spawn_blocking(move || { $($expr)* })
            .await.expect("Blocking task failed to complete.")
    };
}
