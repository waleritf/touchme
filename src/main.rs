#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tao::{event::Event::UserEvent, event_loop::{ControlFlow, EventLoopBuilder}};

mod static_dir;
mod state;
mod local_ip;
mod touch;
mod routes;
mod tray;
mod server;

#[tokio::main]
async fn main() {
  tracing_subscriber::fmt::init();

  let event_loop = EventLoopBuilder::<touch::Touch>::with_user_event().build();
  let event_loop_proxy = event_loop.create_proxy();
  let mut enigo = init_enigo();
  let app_state = state::AppState::new(event_loop_proxy);
  let tray_icon = tray::TrayIcon::build();

  tokio::spawn(async { server::start(app_state).await });

  event_loop.run(move |event, _, control_flow| {
    *control_flow = ControlFlow::Wait;

    match event {
      UserEvent(touch) => touch::handle(&mut enigo, touch),
      _ => { tray_icon.handle_event(); },
    }
  });
}

fn init_enigo() -> enigo::Enigo {
  enigo::Enigo::new(
    &enigo::Settings {
      windows_subject_to_mouse_speed_and_acceleration_level: true,
      ..enigo::Settings::default()
    }
  ).unwrap()
}
