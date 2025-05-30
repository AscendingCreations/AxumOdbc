#![doc = include_str!("../README.md")]
#![allow(dead_code)]

mod errors;
mod manager;
mod pool;

pub(crate) use manager::ENV;
pub(crate) use pool::SharedPool;

pub use errors::OdbcError;
pub use manager::ODBCConnectionManager;
pub use pool::ODBCConnection;
pub use tokio::{self, task};

pub use odbc_api as odbc;

/// Block non async closure or functions so it can run within async.
///
/// # Examples
/// ```rust no_run
///
/// let connection = manager.aquire().await?;
///
/// let _ = blocking!(connection.execute("DROP TABLE IF EXISTS TEST", (), None))?;
/// ```
///
#[macro_export]
macro_rules! blocking {
    ($($expr:tt)*) => {
        $crate::tokio::task::spawn_blocking(move || { $($expr)* })
            .await.expect("Blocking task failed to complete.")
    };
}
