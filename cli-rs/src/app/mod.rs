use chalk_rs::Chalk;
use lazy_static::lazy_static;

use crate::app::help::not_found;

use self::help::main_help;

mod help;

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

pub fn start(args: Vec<String>) {
  if args.len() == 1 {
    if args[0] == "help" {
      println!("{}", main_help());
    } else {
      println!("{}", not_found(&args[0]));
    }
  } else {
    println!("{}", main_help());
  }
}
