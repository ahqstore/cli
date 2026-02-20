use std::fs;

use dialoguer::{theme::ColorfulTheme, Confirm, Input};

use rand::{rng, seq::IndexedRandom};

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

  let _appid = Input::with_theme(&theme)
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

  let _shortcut = Input::with_theme(&theme)
    .allow_empty(false)
    .with_prompt("Application Name")
    .validate_with(|input: &String| {
      if input.len() >= 64 {
        return Err("The display name cannot be longer than 64 characters");
      }
      if input.len() <= 3 {
        return Err("The display name cannot be longer than 3 characters");
      }
      if input
        .chars()
        .any(|c| !c.is_ascii_alphanumeric() && !['-', ':', '.'].iter().any(|x| &c == x))
      {
        return Err("You must use exactly (A-Z, a-z, 0-9, -, :, .)");
      }
      Ok(())
    })
    .interact()
    .expect("This field is required");

  let _storename = Input::with_theme(&theme)
    .allow_empty(false)
    .with_prompt("App Display Name")
    .validate_with(|input: &String| {
      if input.len() >= 64 {
        return Err("The display name cannot be longer than 64 characters");
      }
      if input.len() <= 3 {
        return Err("The display name cannot be longer than 3 characters");
      }
      if input
        .chars()
        .any(|c| !c.is_ascii_alphanumeric() && !['-', ':', '.'].iter().any(|x| &c == x))
      {
        return Err("You must use exactly (A-Z, a-z, 0-9, -, :, .)");
      }
      Ok(())
    })
    .interact()
    .expect("This field is required");

  let (_owner, _repo) = Input::with_theme(&theme)
    .with_prompt("GitHub Repository (owner/repo)")
    .allow_empty(false)
    .validate_with(|x: &String| {
      let Some((owner, repo)) = x.split_once("/") else {
        return Err("Must follow the owner/repo pattern");
      };

      if owner.is_ascii() || repo.is_ascii() {
        return Err("Please ensure that you're using ASCII Complaint text");
      }

      Ok(())
    })
    .interact()
    .expect("This field is required")
    .split_once("/")
    .expect("Unable to parse owner");
}

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
