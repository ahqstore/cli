use chalk_rs::Chalk;
use lazy_static::lazy_static;

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
  INFO.println(&"Hello World");

  println!("Hi {}", &args[0]);
}
