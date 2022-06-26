use anyhow::Context;
use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use tower::util::Either;

use std::net::{Ipv6Addr, SocketAddr, SocketAddrV6, ToSocketAddrs};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = Router::new().route("/trace", post(wrap));

    // let addr = SocketAddr::from(([0, 0, 0, 0], 4000));
    let addr = "[::]:4000".parse::<std::net::SocketAddr>().unwrap();

    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Trace {
    resolvable: String,
}

async fn wrap(input: Json<Trace>) -> impl IntoResponse {
    match trace(input).await {
        Ok(res) => (StatusCode::OK, res),
        Err(e) => {
            eprintln!("{:?}", e);
            (StatusCode::BAD_REQUEST, String::new())
        }
    }
}

async fn trace(Json(Trace { resolvable }): Json<Trace>) -> anyhow::Result<String> {
    let resolved = (resolvable.as_str(), 80)
        .to_socket_addrs()?
        .next()
        .context("No resolution")?;
    let value = resolved.ip().to_string();
    let out = tokio::process::Command::new("mtr")
        .args(["-nbzTwC", &value])
        .output()
        .await?;
    Ok(String::from_utf8(out.stdout)?)
}
