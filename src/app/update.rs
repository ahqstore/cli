use std::{env::home_dir, fs, time::{SystemTime, UNIX_EPOCH}};

use semver::Version;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string};

use crate::app::build::CLIENT;

static URL: &'static str = "https://crates.io/api/v1/crates/ahqstore_cli_rs";

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Update {
  pub last: u64,
  pub to: String,
}

#[derive(Deserialize, Debug)]
pub struct Dependency {
  #[serde(rename = "crate")]
  pub pkg: Crate
}

#[derive(Deserialize, Debug)]
pub struct Crate {
  default_version: String
}

pub fn check_updates() {
  let time = SystemTime::now().duration_since(UNIX_EPOCH).expect("Impossible")
    .as_secs();

  let mut last_updated_file = None;
  let mut update_check = true;

  if let Some(mut x) = home_dir() {
    x.push(".ahqstore");

    if !fs::exists(&x).expect("Unknown IO error") {
      _ = fs::create_dir_all(&x);
    }

    x.push("last_updated");

    let last_updated: Update = from_str(&fs::read_to_string(&x).unwrap_or("0".to_string())).unwrap_or_default();

    // If there has been a gap of 1hr since last check, check again
    if time <= (last_updated.last + 60*60) {
      update_check = false;

      let Ok(manifest) = Version::parse(&last_updated.to) else {
        return;
      };

      let Ok(current) = Version::parse(env!("CARGO_PKG_VERSION")) else {
        return;
      };

      if manifest > current {
        display_update_message(&last_updated.to, env!("CARGO_PKG_VERSION"));
      }
    }

    last_updated_file = Some(x);
  }

  if update_check {
    let Ok(x) = CLIENT.get(URL)
      .send() else {
        return;
    };

    let Ok(x) = x.json::<Dependency>() else {
      return;
    };

    let x = x.pkg;

    let Ok(online_ver) = Version::parse(&x.default_version) else {
      return;
    };

    let Ok(current) = Version::parse(env!("CARGO_PKG_VERSION")) else {
      return;
    };

    let mut update = Update::default();

    if online_ver > current {
      update.to = format!("{}", &x.default_version);
      display_update_message(&x.default_version, env!("CARGO_PKG_VERSION"));
    }

    update.last = time;

    if let Some(file) = last_updated_file {
      // Ignore errors
      _ = fs::write(file, to_string(&update).expect("Impossible")).unwrap();
    }
  }
}

fn display_update_message(new_version: &str, old: &str) {
  let box_width = 72;
  let message_width = box_width - 4;

  let top_bar = format!("\u{2554}{}\u{2557}", "\u{2550}".repeat(box_width - 2));
  let bottom_bar = format!("\u{255A}{}\u{255D}", "\u{2550}".repeat(box_width - 2));
  let vertical_line = "\u{2551}";

  let message_1 = "A newer version of the CLI is available";
  let message_2_part1 = "Updating is strongly recommended to ensure";
  let message_2_part2 = "compatibility with future AHQ Store schema changes";

  println!("{}", top_bar);
  println!("{} {:<width$}{}", vertical_line, message_1, vertical_line, width = message_width + 1);
  println!("{} {:<width$}{}", vertical_line, message_2_part1, vertical_line, width = message_width + 1);
  println!("{} {:<width$}{}", vertical_line, message_2_part2, vertical_line, width = message_width + 1);
  println!("{} {:<width$}{}", vertical_line, "", vertical_line, width = message_width + 1);
  println!("{} ✅ Recommended update: {:<width$}{}", vertical_line, format!("{old} -> {new_version}"), vertical_line, width = message_width - 22);
  println!("{}", bottom_bar);
}
