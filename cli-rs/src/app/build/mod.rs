use ahqstore_types::{
  AHQStoreApplication, DownloadUrl, InstallerOptions, InstallerOptionsLinux, InstallerOptionsWin32,
  Str,
};
use lazy_static::lazy_static;
use reqwest::blocking::{Client, ClientBuilder};
use serde::{Deserialize, Serialize};
use serde_json::to_string_pretty;
use std::{collections::HashMap, env, fs, process};

use crate::app::ERR;

use super::INFO;

mod config;
mod icon;
mod release;
use config::*;
use icon::*;
use release::*;

lazy_static! {
  pub static ref CLIENT: Client = ClientBuilder::new()
    .user_agent("AHQ Store / App Builder")
    .build()
    .unwrap();
}

#[derive(Debug, Deserialize, Serialize)]
struct GHRelease {
  pub upload_url: String,
  pub assets: Vec<GHAsset>,
}

#[derive(Debug, Deserialize, Serialize)]
struct GHAsset {
  pub name: String,
  pub browser_download_url: String,
}
pub fn build_config() {
  let Some(_) = fs::read_dir("./.ahqstore").ok() else {
    ERR.println(&".ahqstore dir couldn't be accessed!");
    process::exit(1);
  };
  INFO.print(&"INFO ");
  println!("Checking .ahqstore");

  let config = get_config();

  let Some(repo) = env::var("GITHUB_REPOSITORY").ok() else {
    ERR.println(&"GITHUB_REPOSITORY variable not present");
    process::exit(1);
  };

  let Some(r_id) = env::var("RELEASE_ID").ok() else {
    ERR.println(&"RELEASE_ID variable not present");
    process::exit(1);
  };

  let Some(gh_token) = env::var("GH_TOKEN").ok() else {
    ERR.println(&"GH_TOKEN variable not present");
    process::exit(1);
  };

  let (version, gh_r) = fetch_release(&repo, &r_id, &gh_token);

  let icon = get_icon(&config.appId);
  #[allow(non_snake_case)]
  let displayImages = get_images(&config.appId);

  let app_id = config.appId.clone();

  let mut final_config = AHQStoreApplication {
    appDisplayName: config.appDisplayName,
    appId: config.appId,
    appShortcutName: config.appShortcutName,
    authorId: config.authorId,
    description: config.description,
    downloadUrls: HashMap::default(),
    icon,
    displayImages,
    install: InstallerOptions {
      linux: None,
      win32: None,
    },
    repo: config.repo,
    version,
  };

  if let Some(platform) = config.platform.win32Platform {
    let Some(options) = config.platform.win32Options else {
      ERR.println(&"Win32 Options not found!");
      process::exit(1);
    };
    let Some(finder) = config.finder.windowsFinder else {
      ERR.println(&"Win32 Finder Config not found!");
      process::exit(1);
    };

    let assets = find_assets(&gh_r, &finder);

    if assets.len() > 1 {
      ERR.println(&"Multiple assets found");
      process::exit(1);
    }

    final_config.downloadUrls.insert(
      1,
      DownloadUrl {
        installerType: platform,
        url: assets[0].browser_download_url.clone(),
      },
    );

    final_config.install.win32 = Some(InstallerOptionsWin32 {
      assetId: 1,
      deps: Some(options.deps),
      exec: options.zip_file_exec.map_or(None, |a| Some(a.to_string())),
      installerArgs: options
        .exe_installer_args
        .map_or(None, |a| Some(a.iter().map(|x| x.to_string()).collect())),
    });
  }

  if let Some(platform) = config.platform.linuxPlatform {
    let Some(options) = config.platform.linuxOptions else {
      ERR.println(&"Linux Options not found!");
      process::exit(1);
    };
    let Some(finder) = config.finder.linuxFinder else {
      ERR.println(&"Linux Finder Config not found!");
      process::exit(1);
    };

    let assets = find_assets(&gh_r, &finder);

    if assets.len() > 1 {
      ERR.println(&"Multiple assets found");
      process::exit(1);
    }

    final_config.downloadUrls.insert(
      2,
      DownloadUrl {
        installerType: platform,
        url: assets[0].browser_download_url.clone(),
      },
    );

    final_config.install.linux = Some(InstallerOptionsLinux {
      assetId: 2,
      deps: Some(options.deps),
    });
  }

  let config_file = to_string_pretty(&final_config).unwrap();

  println!("ahqstore.json");
  println!("{}", &config_file);

  let uup = gh_r
    .upload_url
    .replace("{?name,label}", &format!("?name={app_id}.json"));

  let resp = CLIENT
    .post(uup)
    .header("Content-Length", config_file.len())
    .header("Content-Type", "application/json")
    .header("Accept", "application/json")
    .body(config_file)
    .bearer_auth(&gh_token)
    .send()
    .unwrap()
    .text()
    .unwrap();

  INFO.println(&"GitHub Response");
  println!("{resp}");
}
