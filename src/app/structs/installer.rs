use ahqstore_types::FileIntent;
use serde::{Deserialize, Serialize};

use crate::app::structs::finder::Finder;

#[derive(Debug, Serialize, Deserialize, Default, schemars::JsonSchema)]
#[allow(non_snake_case)]
pub struct InstallerOptions<'a> {
  #[serde(borrow)]
  pub winX64: Option<DesktopPlatform<'a>>,
  #[serde(borrow)]
  pub winArm64: Option<DesktopPlatform<'a>>,
  #[serde(borrow)]
  pub linuxX64: Option<DesktopPlatform<'a>>,
  #[serde(borrow)]
  pub linuxArm64: Option<DesktopPlatform<'a>>,
  #[serde(borrow)]
  pub android: Option<AndroidPlatform<'a>>,
}

#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[allow(non_snake_case)]
pub struct DesktopPlatform<'a> {
  #[serde(borrow)]
  pub finder: Finder<'a>,
  pub intent: FileIntent,
}

#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[allow(non_snake_case)]
#[serde(tag = "installType")]
pub enum AndroidPlatform<'a> {
  #[serde(rename = "universal")]
  Universal {
    #[serde(borrow)]
    finder: Finder<'a>,
    intent: FileIntent,
  },
  #[serde(rename = "split")]
  SplitApk {
    #[serde(borrow)]
    finder: Finder<'a>,
    x64: FileIntent,
    x86: FileIntent,
    arm64: FileIntent,
    arm32: FileIntent,
  },
}
