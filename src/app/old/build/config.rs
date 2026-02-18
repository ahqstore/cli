use std::{env, fs, process};

use serde_json::from_str;

use crate::app::{
  ERR, WARN, shared::{Config, ConfigValue, Finder, IMetadata}
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
    if let ConfigValue::Meta(x) = config.remove(&app_id).expect("Key not present in JSON") {
      x
    } else {
      panic!("No key present in JSON");
    }
  } else {
    WARN.println(&"[WARN] Automatically selecting the 1st key");

    let ConfigValue::Meta(x) = config.into_values().filter(|x| match x {
      ConfigValue::Meta(_) => true,
      _ => false
    }).nth(0).expect("No Key present in JSON") else {
      unreachable!()
    };

    x
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
