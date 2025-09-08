use std::env::args;

mod app;

fn main() {
  let mut arg = args().collect::<Vec<String>>();
  arg.remove(0);

  app::start(arg, std::env::var("CI").unwrap_or("false".to_string()) == "true");
}
