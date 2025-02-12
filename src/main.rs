#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use components::touch::{self, Touch};
use enigo::{Enigo, Settings};
use tokio::sync::mpsc;

mod static_dir;
mod state;
mod components;
mod routes;
mod tray;
mod server;

#[tokio::main]
async fn main() {
  tracing_subscriber::fmt::init();

  let mut enigo = Enigo::new(
    &Settings {
      windows_subject_to_mouse_speed_and_acceleration_level: true,
      ..Settings::default()
    }
  ).unwrap();

  let (ch_tx, mut ch_rx) = mpsc::unbounded_channel::<Touch>();
  
  tokio::spawn(async { server::start(ch_tx).await });
  tokio::spawn(async move {
    while let Some(touch) = ch_rx.recv().await {
      touch::handle(touch, &mut enigo);
    }
  });
  tray::init().unwrap();
}
