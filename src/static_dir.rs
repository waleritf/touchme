use std::io::Cursor;

use axum::{body::Body, extract::Path, http::StatusCode, response::{Html, IntoResponse, Response}};
use include_dir::{include_dir, Dir};

const ASSETS_DIR: Dir<'_> = include_dir!("frontend/dist");

pub fn favicon() -> Cursor<&'static [u8]> {
  Cursor::new(ASSETS_DIR.get_file("favicon.ico").unwrap().contents())
}

pub async fn index() -> impl IntoResponse {
  tracing::debug!("Serving index.html");

  let Some(index_html) = ASSETS_DIR.get_file("index.html") else {
    panic!("Index file not found");
  };

  let content = index_html.contents_utf8().unwrap();
  Html(content)
}

pub async fn assets(Path(path): Path<String>) -> Response {
  tracing::debug!("Serving assets");
  tracing::debug!("{path:?}");

  let file = ASSETS_DIR.get_file(format!("assets/{path}")).unwrap();

  Response::builder()
    .status(StatusCode::OK)
    .header("Content-Type", resolve_mime_type(&path))
    .body(Body::from(file.contents_utf8().unwrap()))
    .unwrap()
}

fn resolve_mime_type(path: &str) -> &'static str {
  match path.split('.').last().unwrap_or("") {
      "html" => "text/html",
      "css" => "text/css",
      "js" => "application/javascript",
      "json" => "application/json",
      "png" => "image/png",
      "jpg" | "jpeg" => "image/jpeg",
      "gif" => "image/gif",
      "svg" => "image/svg+xml",
      "ico" => "image/vnd.microsoft.icon",
      "txt" => "text/plain",
      "wasm" => "application/wasm",
      _ => "application/octet-stream",
  }
}