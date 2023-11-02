use std::env;
use std::net::SocketAddr;

use anyhow::{Context, Error as Anyhow};
use axum::{extract::ConnectInfo, routing::get, Router};
use tracing_subscriber::fmt::time::UtcTime;

fn main() -> anyhow::Result<()> {
    let addr: SocketAddr = env::var("ADDR")
        .map_err(Anyhow::from)
        .and_then(|s| s.parse().map_err(Anyhow::from))
        .context("Unable to determine bind address from environment")?;

    tracing_subscriber::fmt()
        .json()
        .with_file(false)
        .with_line_number(false)
        .with_target(false)
        .with_current_span(false)
        .with_span_list(true)
        .with_timer(UtcTime::rfc_3339())
        .init();

    let routes = Router::new()
        .route("/", get(report_ip))
        .into_make_service_with_connect_info::<SocketAddr>();

    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async move {
            axum::Server::bind(&addr)
                .serve(routes)
                .await
                .map_err(Into::into)
        })
}

async fn report_ip(ConnectInfo(addr): ConnectInfo<SocketAddr>) -> String {
    tracing::info!(client_addr = %addr, "got connection");
    format!("{}\n", addr.ip())
}
