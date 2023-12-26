use std::{fs, process};

use serde_json::from_str;

use crate::app::{
  shared::{Finder, IMetadata},
  ERR,
};

use super::{GHAsset, GHRelease};

pub fn get_config<'a>() -> IMetadata<'a> {
  let Ok(config) = fs::read_to_string("./.ahqstore/config.json") else {
    ERR.println(&"Unable to read config file!");
    process::exit(1);
  };
  let config = config.leak();
  let Ok(config) = from_str::<'a, IMetadata>(config) else {
    ERR.println(&"Unable to read config file!");
    process::exit(1);
  };

  config
}

pub fn find_assets<'a>(gh_r: &'a GHRelease, finder: &'a Finder) -> Vec<&'a GHAsset> {
  gh_r
    .assets
    .iter()
    .filter(|a| {
      if let Some(x) = finder.startsWith {
        if !a.name.starts_with(&x) {
          return false;
        }
      }
      if let Some(x) = finder.contains {
        if !a.name.contains(&x) {
          return false;
        }
      }
      if let Some(x) = finder.endsWith {
        if !a.name.ends_with(&x) {
          return false;
        }
      }

      true
    })
    .collect::<Vec<_>>()
}
