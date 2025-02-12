use tokio::sync::mpsc::UnboundedSender;

use crate::components::touch::Touch;

pub struct AppState {
  pub ch_tx: UnboundedSender<Touch>,
}