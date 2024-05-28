#[cfg(feature = "node")]
extern crate napi_build;

fn main() {
  // Ensures that users can download it too
  #[cfg(feature = "node")]
  napi_build::setup();
}
