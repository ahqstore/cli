#[cfg(feature = "node")]
mod app;

#[cfg(feature = "node")]
#[macro_use]
extern crate napi_derive;

#[cfg(feature = "node")]
#[napi]
pub fn node_entrypoint(args: Vec<String>, gh: bool) {
  app::start(args, gh);
}
