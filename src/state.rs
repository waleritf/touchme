pub struct AppState {
  pub event_loop_proxy: tao::event_loop::EventLoopProxy<crate::touch::Touch>,
}

impl AppState {
  pub fn new(event_loop_proxy: tao::event_loop::EventLoopProxy<crate::touch::Touch>) -> Self {
    Self { event_loop_proxy }
  }
}