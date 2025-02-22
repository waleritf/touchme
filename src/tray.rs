use std::net::Ipv4Addr;

use image::ImageReader;
use tray_icon::{menu::{Menu, MenuEvent, MenuItem, PredefinedMenuItem}, Icon, TrayIconBuilder};

use crate::{local_ip, static_dir};

enum MenuItemEntity {
  Ip(Ipv4Addr),
}

pub struct TrayIcon {
  menu_items: Vec<(MenuItem, MenuItemEntity)>,
  _icon: tray_icon::TrayIcon,
}

impl TrayIcon {
  pub fn build() -> Self {
    let tray_menu = Menu::new();

    let ip_menu_items = local_ip::list()
      .into_iter().map(|(name, ip)| {
        let ip_menu_item = MenuItem::new(
          format!("{ip}:7127 ({name})"), true, None
        );
        tray_menu.append(&ip_menu_item).unwrap();

        (ip_menu_item, MenuItemEntity::Ip(ip))
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

    let tray_icon = TrayIconBuilder::new()
      .with_menu(Box::new(tray_menu))
      .with_tooltip("Touchme")
      .with_icon(Icon::from_rgba(rgba_data, width, height).unwrap())
      .build().unwrap();

    Self {
      _icon: tray_icon,
      menu_items: ip_menu_items,
    }
  }

  pub fn handle_event(&self) {
    if let Ok(event) = MenuEvent::receiver().try_recv() {
      for (menu_item, entity) in &self.menu_items {
        if event.id() == menu_item.id() {
          match entity {
            MenuItemEntity::Ip(ipv4_addr) => {
              if webbrowser::open(&format!("http://{ipv4_addr}:7127/qr")).is_err() {
                tracing::error!("QR Code cannot be opened");
              }
            },
          }
        }
      }
    }
  }
}
