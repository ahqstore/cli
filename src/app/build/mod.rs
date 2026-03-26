use indicatif::{ProgressBar, ProgressStyle};
use serde::{Deserialize, Serialize};

pub type AssetMap = Vec<Asset>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Asset {
  pub name: String,
  pub url: String,
  pub assetid: u8,
}

pub fn bundle() {
  let mut prog = ProgressBar::new(0)
    .with_style(
      ProgressStyle::with_template("{msg>18.cyan/bold} {wide_bar} {pos:>7}/{len:7}")
        .expect("Unable to create progressstyle"),
    )
    .with_message("Pending...");
}
