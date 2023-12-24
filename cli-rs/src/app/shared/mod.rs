use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use ahqstore_types::AppRepo;

pub type Str = String;
pub type Config = HashMap<String, IMetadata>;

mod file_sorter;
mod platforms;
pub use file_sorter::*;
pub use platforms::*;

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct IMetadata {
  pub appId: Str,
  pub appShortcutName: Str,
  pub appDisplayName: Str,
  pub authorId: Str,
  pub description: Str,
  pub repo: AppRepo,
}

impl IMetadata {
  #[allow(non_snake_case)]
  pub fn new(
    appId: Str,
    appShortcutName: Str,
    appDisplayName: Str,
    authorId: Str,
    description: Str,
    repo: AppRepo,
  ) -> Config {
    let mut config = Config::new();

    config.insert(
      appId.clone(),
      Self {
        appId,
        appShortcutName,
        appDisplayName,
        authorId,
        description,
        repo,
      },
    );

    config
  }
}
