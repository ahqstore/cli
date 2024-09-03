use std::env::args;

mod app;

fn main() {
  let mut arg = args().collect::<Vec<String>>();
  arg.remove(0);

  app::start(arg, std::env::var("GH_ACTION").is_ok());
}
