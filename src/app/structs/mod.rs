use serde::{Deserialize, Serialize};

pub mod finder;
pub mod installer;

use installer::InstallerOptions;

#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[allow(non_snake_case)]
pub struct IMetadata<'a> {
  #[serde(borrow)]
  #[serde(rename = "$schema")]
  pub schema: &'a str,

  /// The ID of the application
  #[serde(borrow)]
  pub appId: &'a str,

  /// The name of the shortcut of the app
  #[serde(borrow)]
  pub appShortcutName: &'a str,

  /// The name that'll be displayed in the AHQ Store
  #[serde(borrow)]
  pub appDisplayName: &'a str,

  /// Short description
  #[serde(borrow)]
  pub shortdesc: &'a str,

  /// Unique ID of the author
  #[serde(borrow)]
  pub authorId: &'a str,

  /// Install options
  #[serde(borrow)]
  pub install: InstallerOptions<'a>,

  /// The Repository associated
  #[serde(borrow)]
  pub repo: ApplicationRepository<'a>,

  /// A version provided by the user, like AHQ Store CLI
  #[serde(borrow)]
  pub usrVersion: Option<&'a str>,

  /// The Site to your app
  #[serde(borrow)]
  pub site: Option<&'a str>,

  /// License type or Terms of Service Page
  #[serde(borrow)]
  pub licenseOrTos: Option<&'a str>,
}

#[derive(Debug, Serialize, Deserialize, Default, schemars::JsonSchema)]
#[allow(non_snake_case)]
pub struct ApplicationRepository<'a> {
  pub source: GitProvider,

  #[serde(borrow)]
  pub author: &'a str,
  #[serde(borrow)]
  pub repository: &'a str,
}

#[derive(Debug, Serialize, Deserialize, Default, schemars::JsonSchema)]
#[allow(non_snake_case)]
pub enum GitProvider {
  #[default]
  GitHub,
  GitLab,
}

impl ToString for GitProvider {
  fn to_string(&self) -> String {
    match self {
      Self::GitHub => "GitHub (github.com)",
      Self::GitLab => "GitLab (gitlab.com)",
    }
    .into()
  }
}
