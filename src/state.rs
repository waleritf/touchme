use std::sync::Mutex;

use enigo::{Enigo, Settings};

pub struct AppState {
  pub enigo: Mutex<Enigo>,
}

impl AppState {
  pub fn init() -> Self {
    Self {
      enigo: Mutex::new(init_enigo())
    }
  }
}

fn init_enigo() -> Enigo {
  Enigo::new(
    &Settings {
      windows_subject_to_mouse_speed_and_acceleration_level: true,
      ..Settings::default()
    }
  ).unwrap()
}