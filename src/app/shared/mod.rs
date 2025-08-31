use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use ahqstore_types::AppRepo;

pub type Str = String;
pub type Config<'a> = HashMap<String, ConfigValue<'a>>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ConfigValue<'a> {
  Schema(String),
  #[serde(borrow)]
  Meta(IMetadata<'a>)
}

mod file_sorter;
mod platforms;
pub use file_sorter::*;
pub use platforms::*;

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
/// # MUST EDIT FIELDS
/// - platform
/// - finder
/// 
/// ## MAY EDIT
/// - site
/// - license_or_tos
pub struct IMetadata<'a> {
  /// Application ID: **Auto set by the cli**
  pub appId: Str,
  /// Application Name (as it appears in start menu): **Auto set by the cli**
  pub appShortcutName: Str,
  /// Application Display Name (as it appears in app): **Auto set by the cli**
  pub appDisplayName: Str,
  /// Author ID: **Auto set by the cli**
  pub authorId: Str,
  /// Application Description: **Auto set by the cli**
  pub description: Str,
  /// Application Repository Information: **Auto set by the cli**
  pub repo: AppRepo,
  #[serde[borrow]]
  /// Platform Information **MUST EDIT**
  pub platform: IPlatform<'a>,
  #[serde[borrow]]
  /// Binary Finder Information **MUST EDIT**
  pub finder: FileFinder<'a>,
  /// Your Application Site: **MAY EDIT**
  pub site: Option<Str>,
  /// DO NOT TOUCH THIS
  /// THIS IS FOR INTERNAL USAGE
  pub redistributed: Option<Str>,
  /// Specify your license or preferably a url to the app TOS & LICENSE
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
    platform: IPlatform<'a>
  ) -> Config<'a> {
    let mut config = Config::new();

    config.insert("$schema".to_string(), ConfigValue::Schema(format!("./spec.schema.json")));

    config.insert(
      appId.clone(),
      ConfigValue::Meta(Self {
        // schema: format!("./spec.schema.json"),
        appId,
        appShortcutName,
        appDisplayName,
        authorId,
        description,
        repo,
        platform,
        finder: FileFinder::new(),
        site: None,
        redistributed: None,
        license_or_tos: None,
      }),
    );

    config
  }
}
