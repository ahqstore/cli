mod app;
use node_bindgen::derive::node_bindgen;
use tslink::tslink;

#[tslink]
#[node_bindgen]
pub fn node_entrypoint(args: Vec<String>) {
  app::start(args);
}
