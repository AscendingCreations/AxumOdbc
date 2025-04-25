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

Axum ODBC uses [`tokio`] runtime and uses odbc-api = "12.0.1" internally.

[`tokio`]: https://github.com/tokio-rs/tokio

```toml
# Cargo.toml
[dependencies]
axum_odbc = "0.10.0"
```

#### Cargo Feature Flags
`iodbc`: Sets odbc-api to use iodbc connection manager.

# Example

```rust no_run
use axum::response::IntoResponse;
use axum::{routing::get, Router};
use axum_odbc::{blocking, ODBCConnectionManager};
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {

    let manager = ODBCConnectionManager::new("Driver={ODBC Driver 17 for SQL Server};Server=localhost;UID=SA;PWD=My@Test@Password1;", 5);

    
    // build our application with some routes
    let app = Router::new()
        .route("/drop", get(drop_table))
        .route("/create", get(create_table))
        .with_state(manager);

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}

async fn drop_table(manager: ODBCConnectionManager) -> impl IntoResponse {
    let connection = manager.aquire().await.unwrap();

    blocking!(
        let _ = connection.execute("DROP TABLE IF EXISTS testy", (), None).unwrap();
    );

    "compeleted".to_string()
}

async fn create_table(manager: ODBCConnectionManager) -> impl IntoResponse {
    let connection = manager.aquire().await.unwrap();

    blocking!(
            let _ = connection.execute(
            "IF NOT EXISTS (SELECT * FROM sysobjects WHERE name='testy' AND xtype='U')
            CREATE TABLE testy (
            id INT PRIMARY KEY,
            name VARCHAR(100) NOT NULL
            );",
            (),
            None,
        ).unwrap();
    );

    "compeleted".to_string()
}
```
