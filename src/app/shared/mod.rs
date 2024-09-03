use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use ahqstore_types::AppRepo;

pub type Str = String;
pub type Config<'a> = HashMap<String, IMetadata<'a>>;

mod file_sorter;
mod platforms;
pub use file_sorter::*;
pub use platforms::*;

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct IMetadata<'a> {
  pub appId: Str,
  pub appShortcutName: Str,
  pub appDisplayName: Str,
  pub authorId: Str,
  pub description: Str,
  pub repo: AppRepo,
  #[serde[borrow]]
  pub platform: IPlatform<'a>,
  #[serde[borrow]]
  pub finder: FileFinder<'a>,
  pub site: Option<Str>,
  pub source: Option<Str>,
  pub redistributed: Option<Str>,
  pub license_or_tos: Option<Str>,
}

impl<'a> IMetadata<'a> {
  #[allow(non_snake_case)]
  pub fn new(
    appId: Str,
    appShortcutName: Str,
    appDisplayName: Str,
    authorId: Str,
    description: Str,
    repo: AppRepo,
    platform: IPlatform<'a>,
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
        platform,
        finder: FileFinder::new(),
        site: None,
        source: None,
        redistributed: None,
        license_or_tos: None
      },
    );

    config
  }
}
