mod inquire;
use std::{fs, process};

use inquire::*;
use serde_json::to_string_pretty;

use super::{ERR, INFO, WARN};

pub fn create(force: bool) {
  let (config, platforms) = inquire();

  create_dir(force);

  let succ = (|| {
    let config_file = to_string_pretty(&config).ok()?;
    fs::write("./.ahqstore/config.json", config_file).ok()?;

    fs::create_dir("./.ahqstore/images").ok()?;

    let icon = include_bytes!("./icon.png");
    fs::write("./.ahqstore/icon.png", icon).ok()?;

    let readme = include_str!("./readme.md");
    fs::write("./.ahqstore/README.md", readme).ok()?;

    let plt = to_string_pretty(&platforms).ok()?;
    fs::write("./.ahqstore/platforms.json", plt).ok()
  })()
  .is_some();

  if !succ {
    ERR.println(&"Failed to populate .ahqstore");
    process::exit(1);
  } else {
    println!("🇩​​​​​🇴​​​​​🇳​​​​​🇪​​​​​");
    println!(
      r#"████████████████████████████████████████████████████████████████▀█
█─█─██▀▄─██▄─▄▄─█▄─▄▄─█▄─█─▄███─▄▄▄─█─▄▄─█▄─▄▄▀█▄─▄█▄─▀█▄─▄█─▄▄▄▄█
█─▄─██─▀─███─▄▄▄██─▄▄▄██▄─▄████─███▀█─██─██─██─██─███─█▄▀─██─██▄─█
▀▄▀▄▀▄▄▀▄▄▀▄▄▄▀▀▀▄▄▄▀▀▀▀▄▄▄▀▀▀▀▄▄▄▄▄▀▄▄▄▄▀▄▄▄▄▀▀▄▄▄▀▄▄▄▀▀▄▄▀▄▄▄▄▄▀
"#
    );
  }
}

pub fn create_dir(force: bool) {
  if let Err(_) = fs::create_dir("./.ahqstore") {
    if force {
      WARN.println(&"--force detected\nRemoving dir .ahqstore");

      let succ = (|| {
        fs::remove_dir_all("./.ahqstore").ok()?;
        fs::create_dir_all("./.ahqstore").ok()?;

        Some(())
      })()
      .is_some();

      if succ {
        INFO.println(&".ahqstore directory created, initializing data...");
      } else {
        ERR.println(&"Failed to create .ahqstore directory");
        process::exit(1);
      }
    } else {
      ERR.println(
        &"Failed to create .ahqstore directory\nHint: Use --force option to ignore this error",
      );
      process::exit(1);
    }
  } else {
    INFO.println(&"Created .ahqstore directory, initializing data...");
  }
}
