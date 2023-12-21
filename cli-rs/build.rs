fn main() {
  #[cfg(feature = "node")]
  node_bindgen::build::configure();
}
