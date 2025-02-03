#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod static_dir;
mod state;
mod components;
mod routes;
mod tray;
mod server;

#[tokio::main]
async fn main() {
  tracing_subscriber::fmt::init();
  tokio::spawn(async { server::start().await });
  tray::init().unwrap();
}
