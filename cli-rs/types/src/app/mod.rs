use crate::Str;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod install;
mod other_fields;

pub use install::*;
pub use other_fields::*;

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct AHQStoreApplication {
  pub appId: Option<Str>,
  pub appDisplayName: Option<Str>,
  pub authorId: Option<Str>,
  pub downloadUrls: Option<HashMap<u8, DownloadUrl>>,
  pub install: Option<InstallerOptions>,
  pub description: Str,
  pub icon: Str,
  pub repo: AppRepo,
  pub version: Str,
}
