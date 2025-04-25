<h1 align="center">
    AxumODBC
</h1>
<div align="center">
    Library to Provide an <a href="https://github.com/pacman82/odbc-api">ODBC-API</a> layer.
</div>
<br />
<div align="center">
    <a href="https://crates.io/crates/axum_odbc"><img src="https://img.shields.io/crates/v/axum_odbc?style=plastic" alt="crates.io"></a>
    <a href="https://docs.rs/axum_odbc"><img src="https://docs.rs/axum_odbc/badge.svg" alt="docs.rs"></a>
    <img src="https://img.shields.io/badge/min%20rust-1.60-green.svg" alt="Minimum Rust Version">
</div>

## License

This project is licensed under either [Apache License, Version 2.0](LICENSE-APACHE), [zlib License](LICENSE-ZLIB), or [MIT License](LICENSE-MIT), at your option.

## Help

If you need help with this library or have suggestions please go to our [Discord Group](https://discord.gg/gVXNDwpS3Z)

## Install

Axum ODBC uses [`tokio`] runtime.

[`tokio`]: https://github.com/tokio-rs/tokio

```toml
# Cargo.toml
[dependencies]
axum_odbc = "0.10.0"
odbc-api = "12.0.1"
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
