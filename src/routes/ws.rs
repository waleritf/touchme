use std::sync::Arc;
use axum::{extract::{ws::{Message, WebSocket}, State, WebSocketUpgrade}, response::IntoResponse};
use crate::state::AppState;

pub async fn handler(
  ws: WebSocketUpgrade,
  State(state): State<Arc<AppState>>
) -> impl IntoResponse {
  tracing::info!("Client connected");
  ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(mut socket: WebSocket, state: Arc<AppState>) {
  while let Some(msg) = socket.recv().await {
    match msg {
      Err(err) => { tracing::error!("{err:?}") },
      Ok(msg) => { handle_message(msg, &state) }
    }
  }
}

fn handle_message(msg: Message, state: &Arc<AppState>) {
  match msg {
    Message::Text(text) => {
      match serde_json::from_str(&text) {
        Err(err) => tracing::error!("{err:?}"),
        Ok(msg) => state.event_loop_proxy.send_event(msg).unwrap(),
      }
    },
    Message::Close(_) => tracing::info!("Client disconnected"),
    msg => tracing::info!("Ignore message {msg:?}"),
  }
}