use std::process;

use ahqstore_types::AppRepo;
use inquire::{
  list_option::ListOption,
  validator::{ErrorMessage, Validation},
  Editor, MultiSelect, Text,
};

use crate::app::{
  shared::{Config, IMetadata},
  ERR,
};

pub fn inquire() -> (Config, Vec<&'static str>) {
  let Ok(app_id) = Text::new("Application ID:")
    .with_default("8LmFjl3xtm5tAzcdHFvW")
    .prompt()
  else {
    ERR.println(&"Must Enter an ID");
    process::exit(1);
  };

  let Ok(display_name) = Text::new("Application Display Name:")
    .with_default("My Cool App")
    .prompt()
  else {
    ERR.println(&"Must Enter a name");
    process::exit(1);
  };

  let Ok(user_id) = Text::new("Your AHQ Store Author ID:")
    .with_default("9MXn6dqO0qaUiqYDiaPoV5k3owd2")
    .prompt()
  else {
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

  let validator = |input: &[ListOption<&&str>]| {
    if input.len() == 0 {
      return Ok(Validation::Invalid(
        "You must select at least one target".into(),
      ));
    }
    if input.len() > 2 {
      return Ok(Validation::Invalid(
        "You can only select two targets".into(),
      ));
    }

    let flagged = vec![0, 1, 2, 3];
    if input
      .iter()
      .filter(|a| flagged.contains(&a.index))
      .collect::<Vec<_>>()
      .len()
      > 1
    {
      return Ok(Validation::Invalid(
        "You can only select one bundle target for a platform".into(),
      ));
    }

    Ok(Validation::Valid)
  };

  let Ok(platforms) = MultiSelect::new(
    "Which platforms do you intend to support?",
    vec![
      "64-bit Windows (ZIP)",
      "64-bit Windows Installer (MSI; Not Supported Right Now)",
      "64-bit Windows Installer (EXE; Not Supported Right Now)",
      "UWP Windows Installer (Msix; Not Supported Right Now)",
      "64-bit Linux (AppImage; Under Development )",
    ],
  )
  .with_default(&[0])
  .with_validator(validator)
  .prompt() else {
    ERR.println(&"Must Select a platform");
    process::exit(1);
  };

  (
    IMetadata::new(
      app_id,
      display_name,
      user_id,
      desc,
      AppRepo {
        author: owner.into(),
        repo: repo.into(),
      },
    ),
    platforms,
  )
}
