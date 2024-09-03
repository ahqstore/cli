use std::{env, fs, process};

use serde_json::from_str;

use crate::app::{
  shared::{Config, Finder, IMetadata},
  ERR,
};

use super::{GHAsset, GHRelease};

pub fn get_config<'a>() -> IMetadata<'a> {
  let Ok(config) = fs::read_to_string("./.ahqstore/config.json") else {
    ERR.println(&"Unable to read config file!");
    process::exit(1);
  };
  let config = config.leak();
  let Ok(mut config) = from_str::<'a, Config>(config) else {
    ERR.println(&"Unable to read config file!");
    process::exit(1);
  };

  if let Ok(app_id) = env::var("APP_ID") {
    config.remove(&app_id).expect("Key not present in JSON")
  } else {
    config.into_values().nth(0).expect("No Key present in JSON")
  }
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
