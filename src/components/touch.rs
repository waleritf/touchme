use std::sync::Mutex;

use enigo::{Enigo, Keyboard, Mouse};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum MouseAction {
  Move { x: f32, y: f32 },
  Click,
  Press,
  Release,
}

#[derive(Debug, Deserialize)]
pub enum Button {
  Up,
  Down,
  Right,
  Left,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum KeyAction {
  Click { button: Button },
}

#[derive(Debug, Deserialize)]
#[serde(tag = "input", content = "action")]
pub enum Touch {
  Mouse(MouseAction),
  Key(KeyAction),
}

pub fn handle(touch: Touch, enigo: &Mutex<Enigo>) {
  let mut enigo = enigo.lock().unwrap();

  match touch {
    Touch::Mouse(action) => {
      match action {
        MouseAction::Move { x, y } => {
          tracing::debug!("Move {x} {y}");

          let a = 1.0;
          let ax = (x * a).round() as i32;
          let ay = (y * a).round() as i32;

          if let Err(err) = enigo.move_mouse(
            ax, ay, enigo::Coordinate::Rel
          ) {
            tracing::error!("{err:?}");
          }
        },
        MouseAction::Click => {
          tracing::debug!("Mouse Click");
          if let Err(err) = enigo.button(
            enigo::Button::Left, enigo::Direction::Click
          ) {
            tracing::error!("{err:?}");
          }
        },
        MouseAction::Press => {
          tracing::debug!("Mouse Press");
          if let Err(err) = enigo.button(
            enigo::Button::Left, enigo::Direction::Press
          ) {
            tracing::error!("{err:?}");
          }
        },
        MouseAction::Release => {
          tracing::debug!("Mouse Release");
          if let Err(err) = enigo.button(
            enigo::Button::Left, enigo::Direction::Release
          ) {
            tracing::error!("{err:?}");
          }
        },
      }
    },
    Touch::Key(action) => {
      match action {
        KeyAction::Click { button } => {
          match button {
            Button::Up => {
              tracing::debug!("Key Click Up");
              if let Err(err) = enigo.key(
                enigo::Key::UpArrow, enigo::Direction::Click
              ) {
                tracing::error!("{err:?}");
              }
            },
            Button::Down => {
              tracing::debug!("Key Click Down");
              if let Err(err) = enigo.key(
                enigo::Key::DownArrow, enigo::Direction::Click
              ) {
                tracing::error!("{err:?}");
              }
            },
            Button::Right => {
              tracing::debug!("Key Click Right");
              if let Err(err) = enigo.key(
                enigo::Key::RightArrow, enigo::Direction::Click
              ) {
                tracing::error!("{err:?}");
              }
            },
            Button::Left => {
              tracing::debug!("Key Click Left");
              if let Err(err) = enigo.key(
                enigo::Key::LeftArrow, enigo::Direction::Click
              ) {
                tracing::error!("{err:?}");
              }
            },
          }
        },
      }
    },
  }
}
