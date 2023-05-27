fn main() {
  tauri_build::build();
  println!("cargo:rustc-link-search=native=proprietary_lib")
}
