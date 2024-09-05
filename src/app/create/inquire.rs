use std::process;

use ahqstore_types::AppRepo;
use inquire::{
  validator::{ErrorMessage, Validation},
  Editor, Text,
};
use serde::{Deserialize, Serialize};

use crate::app::{
  shared::{Config, IMetadata, IPlatform},
  ERR, INFO,
};

#[derive(Serialize, Deserialize)]
struct ServerUserResp {
  pub linked_acc: Vec<String>,
}

pub fn inquire<'a>() -> (String, Config<'a>) {
  INFO.println(&"Generating a random Application ID");
  let Ok(app_id) = Text::new("Application ID:")
    .with_default(&gen_appid())
    .prompt()
  else {
    ERR.println(&"Must Enter an ID");
    process::exit(1);
  };

  let Ok(app_name) = Text::new("Start menu entry name:")
    .with_default("Application")
    .prompt()
  else {
    ERR.println(&"Must Enter a name");
    process::exit(1);
  };

  let Ok(display_name) = Text::new("Application Display Name:")
    .with_default("My Cool App")
    .prompt()
  else {
    ERR.println(&"Must Enter a name");
    process::exit(1);
  };

  let Ok(user_id) = Text::new("Your AHQ Store Author ID:").prompt() else {
    ERR.println(&"Must Enter an ID");
    process::exit(1);
  };

  let Ok(desc) = Editor::new("Enter your app description").prompt() else {
    ERR.println(&"Must Enter a description");
    process::exit(1);
  };

  let Ok(repo) = Text::new("Enter your app repository:")
    .with_default("owner/repoName")
    .with_validator(|string: &str| {
      if string.split("/").collect::<Vec<_>>().len() == 2 {
        Ok(Validation::Valid)
      } else {
        Ok(Validation::Invalid(ErrorMessage::Custom(
          "Must be in the format owner/repoName".into(),
        )))
      }
    })
    .prompt()
  else {
    ERR.println(&"Must Enter a repository");
    process::exit(1);
  };

  let [owner, repo] = repo.split("/").collect::<Vec<_>>()[..] else {
    panic!("Repo Parsing Failed")
  };

  INFO.println(&"Validating author id & repo");
  
  (
    app_id.clone(),
    IMetadata::new(
      app_id,
      app_name,
      display_name,
      user_id,
      desc,
      AppRepo {
        author: owner.into(),
        repo: repo.into(),
      },
      IPlatform::new()
    ),
  )
}

fn gen_appid() -> String {
  let mut string = String::new();

  use rand::seq::SliceRandom;

  let val = vec![
    "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s",
    "t", "u", "v", "w", "s", "y", "z", "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L",
    "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "S", "Y", "Z", "0", "1", "2", "3", "4",
    "5", "6", "7", "8", "9"
  ];

  for _ in 0..40 {
    let val = val.choose(&mut rand::thread_rng()).unwrap();
    string.push_str(val);
  }

  string
}
