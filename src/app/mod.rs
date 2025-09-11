use chalk_rs::Chalk;
use lazy_static::lazy_static;

mod build;
mod create;
mod help;
pub mod shared;

mod update;

lazy_static! {
  static ref INFO: Chalk = {
    let mut chalk = Chalk::new();
    chalk.blue().bold();
    chalk
  };
  static ref WARN: Chalk = {
    let mut chalk = Chalk::new();
    chalk.yellow().bold();
    chalk
  };
  static ref ERR: Chalk = {
    let mut chalk = Chalk::new();
    chalk.red().bold();
    chalk
  };
}

pub fn start(args: Vec<String>, gh: bool) {
  update::check_updates();

  if args.len() >= 1 {
    match args[0].as_str() {
      "create" => create::create(args.len() > 1 && (&args[1] == "--force" || &args[1] == "-f")),
      "build" => build::build_config(false, gh),
      "upload" => build::build_config(true, gh),
      "help" => println!("{}", help::main_help()),
      a => println!("{}", help::not_found(a)),
    }
  } else {
    println!("{}", help::main_help());
  }
}
