use std::net::Ipv4Addr;

use image::ImageReader;
use tao::event_loop::{ControlFlow, EventLoop};
use tray_icon::{menu::{Menu, MenuEvent, MenuItem, PredefinedMenuItem}, Icon, TrayIconBuilder};

use crate::{components::local_ip, static_dir};

pub fn init() -> Result<(), Box<dyn std::error::Error>> {
  let event_loop = EventLoop::new();
  let tray_menu = Menu::new();

  let ip_menu_items: Vec<(MenuItem, Ipv4Addr)> = local_ip::list().into_iter().map(|(name, ip)| {
    let ip_menu_item = MenuItem::new(format!("{ip}:50127 ({name})"), true, None);
    tray_menu.append(&ip_menu_item).unwrap();

    (ip_menu_item, ip)
  }).collect();

  tray_menu.append(&PredefinedMenuItem::separator()).unwrap();
  tray_menu.append(&PredefinedMenuItem::quit(Some("Exit"))).unwrap();

  let image = ImageReader::new(static_dir::favicon())
    .with_guessed_format()
    .expect("Failed to open ICO file")
    .decode()
    .expect("Failed to decode ICO file");

  let rgba_image = image.to_rgba8();
  let (width, height) = rgba_image.dimensions();
  let rgba_data = rgba_image.into_raw();


  let _tray_icon = TrayIconBuilder::new()
    .with_menu(Box::new(tray_menu))
    .with_tooltip("Touchme")
    .with_icon(Icon::from_rgba(rgba_data, width, height).unwrap())
    .build()?;

  event_loop.run(move |_, _, control_flow| {
    *control_flow = ControlFlow::Wait;

    if let Ok(event) = MenuEvent::receiver().try_recv() {
      for (menu_item, ip) in &ip_menu_items {
        if event.id() == menu_item.id() {
          if webbrowser::open(&format!("http://{ip}:50127/qr")).is_err() {
            tracing::error!("QR Code cannot be opened");
          }
        }
      }
    }
  });
}
