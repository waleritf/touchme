use std::sync::Arc;

use axum::{routing::{any, get}, Router};

use crate::{routes, state::AppState, static_dir::{assets, index}};

pub async fn start(app_state: AppState) {
  let app = Router::new()
    .route("/", get(index))
    .route("/qr", get(index))
    .route("/assets/{*path}", get(assets))
    .route("/ws", any(routes::ws::handler))
    .with_state(Arc::new(app_state));

  let lis = tokio::net::TcpListener::bind("0.0.0.0:7127").await.unwrap();

  tracing::info!("Listening on {}", lis.local_addr().unwrap());
  axum::serve(lis, app).await.unwrap();
}
