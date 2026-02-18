use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
#[allow(non_snake_case)]
/// # Self Explanatory
pub struct FileFinder<'a> {
  #[serde(borrow)]
  pub windowsAmd64Finder: Option<Finder<'a>>,
  #[serde(borrow)]
  pub windowsArm64Finder: Option<Finder<'a>>,
  #[serde(borrow)]
  pub linuxAmd64Finder: Option<Finder<'a>>,
  #[serde(borrow)]
  pub linuxArm64Finder: Option<Finder<'a>>,
  #[serde(borrow)]
  pub linuxArm32Finder: Option<Finder<'a>>,
  #[serde(borrow)]
  pub androidUniversalFinder: Option<Finder<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[allow(non_snake_case)]
pub struct Finder<'a> {
  #[serde(borrow)]
  pub startsWith: Option<&'a str>,
  #[serde(borrow)]
  pub contains: Option<&'a str>,
  #[serde(borrow)]
  pub endsWith: Option<&'a str>,
}

impl<'a> FileFinder<'a> {
  pub fn new() -> Self {
    Self {
      windowsAmd64Finder: Some(Finder {
        startsWith: Some("This-is"),
        contains: Some("an"),
        endsWith: Some(".example"),
      }),
      ..Default::default()
    }
  }
}
