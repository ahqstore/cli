use std::{fs, process};

use serde_json::from_str;

use crate::app::{
  shared::{FileFinder, Finder, IMetadata, IPlatform},
  ERR,
};

use super::{GHAsset, GHRelease};

pub fn get_configs<'a>() -> (IMetadata, IPlatform<'a>, FileFinder<'a>) {
  let Ok(config) = fs::read_to_string("./.ahqstore/config.json") else {
    ERR.println(&"Unable to read config file!");
    process::exit(1);
  };
  let Ok(config) = from_str::<IMetadata>(&config) else {
    ERR.println(&"Unable to read config file!");
    process::exit(1);
  };

  let Ok(platforms) = fs::read_to_string("./.ahqstore/config.json") else {
    ERR.println(&"Unable to read platforms file!");
    process::exit(1);
  };
  let platforms = platforms.leak();
  let Ok(platforms) = from_str::<'a, IPlatform>(platforms) else {
    ERR.println(&"Unable to read platforms file!");
    process::exit(1);
  };

  let Ok(finder) = fs::read_to_string("./.ahqstore/finder.json") else {
    ERR.println(&"Unable to read finder file!");
    process::exit(1);
  };
  let finder = finder.leak();
  let Ok(finder) = from_str::<'a, FileFinder>(finder) else {
    ERR.println(&"Unable to read finder file!");
    process::exit(1);
  };

  (config, platforms, finder)
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
