#![allow(dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "schemars", schemars::JsonSchema)]
#[allow(non_snake_case)]
/// All the elements of the Finder are case sensitive
///
/// This Finder structure assumes minimal presence of
/// artifact differentiators that are required for a smooth
/// artifact match.
/// If your project does not have the minimum presence, you
/// might need to reorganize your CI/CD Pipeline!
///
/// ## Note
/// The structure has been designed to allow robust pattern matching
/// with static information instead of utilizing a runtime scripting
/// language
pub struct Finder<'a> {
  #[serde(borrow)]
  /// A list of prefixes that the artifact name should start with
  pub startsWith: OrBlock<&'a str>,
  #[serde(borrow)]
  pub contains: AndBlock<OrBlock<&'a str>>,
  #[serde(borrow)]
  pub notcontains: OrBlock<&'a str>,
  #[serde(borrow)]
  pub endsWith: OrBlock<&'a str>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "schemars", schemars::JsonSchema)]
#[allow(non_snake_case)]
#[repr(transparent)]
pub struct AndBlock<T>(Vec<T>);

#[derive(Debug, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "schemars", schemars::JsonSchema)]
#[allow(non_snake_case)]
#[repr(transparent)]
pub struct OrBlock<T>(Vec<T>);

impl<'a> Finder<'a> {
  pub fn matches(&self, raw: &str) -> bool {
    let starts =
      self.startsWith.0.is_empty() || self.startsWith.0.iter().any(|x| raw.starts_with(x));
    let contains = self.contains.0.is_empty()
      || self
        .contains
        .0
        .iter()
        .all(|x| x.0.iter().any(|d| raw.contains(d)));
    let notcontains =
      self.notcontains.0.is_empty() || !self.notcontains.0.iter().any(|x| raw.contains(x));
    let ends = self.endsWith.0.is_empty() || self.endsWith.0.iter().any(|x| raw.ends_with(x));

    #[cfg(feature = "debug")]
    return dbg!(starts) && dbg!(contains) && dbg!(notcontains) && dbg!(ends);

    #[cfg(not(feature = "debug"))]
    return starts && contains && notcontains && ends;
  }
}
