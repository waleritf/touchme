fn main() {
  if cfg!(target_os = "windows") {
    let mut res = winres::WindowsResource::new();
    res.set_icon("./frontend/dist/favicon.ico");
    res.compile().unwrap();
  }
}