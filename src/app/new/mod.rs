use std::fs;

use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};

use rand::{rng, seq::IndexedRandom};
use schemars::schema_for;
use serde_json::{to_string, to_string_pretty};

use crate::app::structs::{
  installer::InstallerOptions, ApplicationRepository, GitProvider, IMetadata,
};

pub async fn generate(force: bool) {
  let theme = ColorfulTheme::default();

  if fs::exists("./.ahqstore").expect("Unable to lookup .ahqstore") {
    if !force
      && !Confirm::with_theme(&theme)
        .with_prompt("Delete existing .ahqstore directory?")
        .show_default(false)
        .interact()
        .unwrap_or(false)
    {
      panic!("Unable to continue : `.ahqstore` directory exists!");
    }

    // Clear it all out
    fs::remove_dir_all("./.ahqstore").expect("Unable to delete `.ahqstore` directory");
  }

  let appid = Input::with_theme(&theme)
    .allow_empty(false)
    .default(gen_appid())
    .with_prompt("Application ID")
    .validate_with(|input: &String| {
      if input.len() != 64 {
        return Err("The ID must have a length of 64");
      }
      if input.chars().any(|c| !c.is_ascii_alphanumeric()) {
        return Err("You must use exactly Base 62 (A-Z, a-z, 0-9)");
      }
      Ok(())
    })
    .interact()
    .expect("Application ID is required!");

  let shortcut = Input::with_theme(&theme)
    .allow_empty(false)
    .with_prompt("Application Name")
    .validate_with(|input: &String| {
      if input.len() >= 64 {
        return Err("The display name cannot be longer than 64 characters");
      }
      if input.len() <= 3 {
        return Err("The display name must be longer than 3 characters");
      }
      if input
        .chars()
        .any(|c| !c.is_ascii_alphanumeric() && ![' ', '-', ':', '.'].iter().any(|x| &c == x))
      {
        return Err("You must use exactly (A-Z, a-z, 0-9, -, :, .)");
      }
      Ok(())
    })
    .interact()
    .expect("This field is required");

  let storename = Input::with_theme(&theme)
    .allow_empty(false)
    .with_prompt("App Display Name")
    .validate_with(|input: &String| {
      if input.len() > 64 {
        return Err("The display name cannot be longer than 64 characters");
      }
      if input.len() < 3 {
        return Err("The display name must be longer than 3 characters");
      }
      if input
        .chars()
        .any(|c| !c.is_ascii_alphanumeric() && ![' ', '-', ':', '.'].iter().any(|x| &c == x))
      {
        return Err("You must use exactly (A-Z, a-z, 0-9, -, :, .)");
      }
      Ok(())
    })
    .interact()
    .expect("This field is required");

  let shortdesc = Input::with_theme(&theme)
    .allow_empty(false)
    .with_prompt("Short Description")
    .validate_with(|input: &String| {
      if input.len() >= 64 {
        return Err("The description name cannot be longer than 64 characters");
      }
      if input.len() <= 3 {
        return Err("The description name must be longer than 3 characters");
      }
      if !input.is_ascii() {
        return Err("You must use exactly ASCII characters");
      }
      Ok(())
    })
    .interact()
    .expect("This field is required");

  let devid = Input::with_theme(&theme)
    .allow_empty(false)
    .with_prompt("AHQStore Developer ID")
    .validate_with(|input: &String| {
      if input.len() <= 3 {
        return Err("The display name must be longer than 3 characters");
      }

      Ok(())
    })
    .interact()
    .expect("This field is required");

  let provider = match Select::with_theme(&theme)
    .item(GitProvider::GitHub)
    .item(GitProvider::GitLab)
    .default(0)
    .with_prompt("Please select your Git Provider")
    .interact()
    .expect("This is required!")
  {
    0 => GitProvider::GitHub,
    _ => unreachable!(),
  };

  let repo = Input::with_theme(&theme)
    .with_prompt("Git Repository (owner/repo)")
    .allow_empty(false)
    .validate_with(|x: &String| {
      let Some((owner, repo)) = x.split_once("/") else {
        return Err("Must follow the owner/repo pattern");
      };

      if !owner.is_ascii() || !repo.is_ascii() {
        return Err("Please ensure that you're using ASCII Complaint text");
      }

      Ok(())
    })
    .interact()
    .expect("This field is required");
  let (owner, repo) = repo.split_once("/").expect("Unable to parse owner");

  fs::create_dir_all("./.ahqstore/schemas").expect("Unable to create `.ahqstore` directory");
  fs::create_dir_all("./.ahqstore/config").expect("Unable to create `.ahqstore` directory");
  fs::create_dir_all("./.ahqstore/artifacts").expect("Unable to create `.ahqstore` directory");
  fs::create_dir_all("./.ahqstore/bundle").expect("Unable to create `.ahqstore` directory");

  let metadata = to_string_pretty(&IMetadata {
    appId: &appid,
    appDisplayName: &storename,
    appShortcutName: &shortcut,
    shortdesc: &shortdesc,
    authorId: &devid,
    install: InstallerOptions::default(),
    licenseOrTos: None,
    repo: ApplicationRepository {
      source: provider,
      author: owner,
      repository: repo,
    },
    schema: SCHEMAVER,
    site: None,
    usrVersion: None,
  })
  .expect("Unable to build initial manifest");

  fs::write("./.ahqstore/config/metadata.json", metadata).expect("Unable to write metadata");

  let schema = to_string(&schema_for!(IMetadata)).expect("Unable to Serialize schema");

  fs::write(
    concat!(
      "./.ahqstore/schemas/schema_",
      env!("CARGO_PKG_VERSION"),
      ".schema.json"
    ),
    schema,
  )
  .expect("Unable to write metadata");
}

const SCHEMAVER: &str = concat!(
  "../schemas/schema_",
  env!("CARGO_PKG_VERSION"),
  ".schema.json"
);

const VALS: [char; 62] = [
  'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
  't', 'u', 'v', 'w', 'x', 'y', 'z', // Lowercase
  'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
  'T', 'U', 'V', 'W', 'X', 'Y', 'Z', // Uppercase
  '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', // Digits
];

fn gen_appid() -> String {
  let mut string = String::with_capacity(3 * 64 / 2);

  for _ in 0..64 {
    let val = VALS.choose(&mut rng()).unwrap();
    string.push(*val);
  }

  string
}
