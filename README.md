# AxumODBC

Library to Provide an ODBC-Api layer.

[![https://crates.io/crates/axum_odbc](https://img.shields.io/crates/v/axum_odbc?style=plastic)](https://crates.io/crates/axum_odbc)
[![Docs](https://docs.rs/axum_odbc/badge.svg)](https://docs.rs/axum_odbc)

## Help

If you need help with this library or have suggestions please go to our [Discord Group](https://discord.gg/xKkm7UhM36)

## Install

Axum ODBC uses [`tokio`] runtime.

[`tokio`]: https://github.com/tokio-rs/tokio

```toml
# Cargo.toml
[dependencies]
axum_odbc = "0.3.1"
```

#### Cargo Feature Flags
`iodbc`: Sets odbc-api to use iodbc connection manager.

# Example

```rust no_run
use axum_odbc::{ODBCConnectionManager, blocking};
use axum::{
    Router,
    routing::get,
};
use std::time::Duration;

#[tokio::main]
async fn main() {

    let manager = ODBCConnectionManager::new("Driver={ODBC Driver 17 for SQL Server};Server=localhost;UID=SA;PWD=My@Test@Password1;", 5);

    // build our application with some routes
    let app = Router::with_state(manager)
        .route("/drop", get(drop_table));

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn drop_table(manager: ODBCConnectionManager) -> String {
    let mut connection = manager.aquire().await.unwrap();
    let sleep = || tokio::time::sleep(Duration::from_millis(50));

    let _ = connection.execute_polling("DROP TABLE IF EXISTS TEST", (), sleep).await.unwrap();

    "compeleted".to_string()
}
```
