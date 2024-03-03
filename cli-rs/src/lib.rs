#[cfg(feature = "node")]
mod app;
#[cfg(feature = "node")]
use node_bindgen::derive::node_bindgen;
#[cfg(feature = "node")]
use tslink::tslink;

#[cfg(feature = "node")]
#[tslink]
#[node_bindgen]
pub fn node_entrypoint(args: Vec<String>, gh: bool) {
  app::start(args, gh);
}
