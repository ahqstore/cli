use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct FileFinder<'a> {
  #[serde(borrow)]
  pub windowsFinder: Option<Finder<'a>>,
  #[serde(borrow)]
  pub linuxFinder: Option<Finder<'a>>,
}

#[derive(Debug, Serialize, Deserialize)]
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
      windowsFinder: Some(Finder {
        startsWith: Some("This-is"),
        contains: Some("an"),
        endsWith: Some(".example"),
      }),
      linuxFinder: None,
    }
  }
}
