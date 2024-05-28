mod inquire;
use std::{fs, process};

use inquire::*;
use serde_json::to_string_pretty;

use super::{ERR, INFO, WARN};

pub fn create(force: bool) {
  let (id, config) = inquire();

  create_dir(force);

  let succ = (|| {
    let config_file = to_string_pretty(&config).ok()?;
    fs::write("./.ahqstore/config.json", config_file).ok()?;

    let base_img = format!("./.ahqstore/images/{}", &id);

    fs::create_dir_all(&base_img).ok()?;

    let icon = include_bytes!("./icon.png");
    fs::write(format!("{}/icon.png", &base_img), icon).ok()?;

    let readme = include_str!("./readme.md");
    fs::write("./.ahqstore/README.md", readme).ok()
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
    INFO.println(&"Do not forget to edit config.json and finder.json\nMore details about all the files is present in README.md");
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