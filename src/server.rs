use axum::{routing::{any, get}, Router};
use enigo::Enigo;
use tokio::sync::mpsc::UnboundedSender;

use crate::{components::touch::Touch, routes, state::AppState, static_dir::{assets, index}};

pub async fn start(ch_tx: UnboundedSender<Touch>) {
  let app = Router::new()
    .route("/", get(index))
    .route("/qr", get(index))
    .route("/assets/{*path}", get(assets))
    .route("/ws", any(routes::ws::handler))
    .with_state(AppState { ch_tx: ch_tx }.into());

  let lis = tokio::net::TcpListener::bind("0.0.0.0:7127").await.unwrap();

  tracing::info!("Listening on {}", lis.local_addr().unwrap());
  axum::serve(lis, app).await.unwrap();
}
