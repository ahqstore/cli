mod app;

pub use app::shared;

#[cfg(feature = "node")]
#[macro_use]
extern crate napi_derive;

#[cfg_attr(feature = "node", napi)]
pub fn node_entrypoint(args: Vec<String>, gh: bool) {
  app::start(args, gh);
}
