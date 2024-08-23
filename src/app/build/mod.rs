use ahqstore_types::{
  AHQStoreApplication, DownloadUrl, InstallerFormat, InstallerOptions, InstallerOptionsAndroid, InstallerOptionsLinux, InstallerOptionsWindows, Str
};
use lazy_static::lazy_static;
use reqwest::blocking::{Client, ClientBuilder};
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string, to_string_pretty};
use std::{collections::HashMap, env, fs, process};

use crate::app::ERR;

use super::INFO;

mod config;
mod icon;
mod release;
use config::*;
use icon::*;
use release::*;

#[macro_use]
mod macros;

lazy_static! {
  pub static ref CLIENT: Client = ClientBuilder::new()
    .user_agent("AHQ Store / App Builder")
    .build()
    .unwrap();
}

#[derive(Debug, Deserialize, Serialize)]
struct GHRelease {
  pub tag_name: String,
  pub upload_url: String,
  pub assets: Vec<GHAsset>,
}

#[derive(Debug, Deserialize, Serialize)]
struct GHAsset {
  pub name: String,
  pub browser_download_url: String,
}
pub fn build_config(upload: bool, gh_action: bool) {
  let Some(_) = fs::read_dir("./.ahqstore").ok() else {
    ERR.println(&".ahqstore dir couldn't be accessed!");
    process::exit(1);
  };
  if !gh_action {
    INFO.print(&"INFO ");
    println!("Checking .ahqstore");
  }

  let config = get_config();

  let repo = env::var("GITHUB_REPOSITORY").unwrap_or("%NUL".into());

  if &repo == "%NUL" {
    ERR.println(&"GITHUB_REPOSITORY not set");
    process::exit(1);
  }

  let r_id = env::var("RELEASE_ID").unwrap_or("latest".into());

  if &r_id == "latest" && upload {
    ERR.println(&"RELEASE_ID variable not present");
    process::exit(1);
  };
  if &r_id == "latest" {
    INFO.print(&"INFO ");
    println!("Getting latest release");
  }

  let gh_token = env::var("GH_TOKEN").unwrap_or("".into());

  if &gh_token == "" && upload {
    ERR.println(&"GH_TOKEN variable not present");
    process::exit(1);
  };

  let (version, gh_r) = fetch_release(&repo, &r_id, &gh_token);

  let icon = get_icon(&config.appId);
  let dspl_images = get_images(&config.appId);

  let mut resources = HashMap::new();
  resources.insert(0, icon);

  #[allow(non_snake_case)]
  let displayImages = dspl_images.into_iter().enumerate().map(|(uid, icon)| {
    resources.insert(uid as u8 + 1u8, icon);

    uid as u8
  }).collect();

  let app_id = config.appId.clone();

  let mut final_config: AHQStoreApplication = AHQStoreApplication {
    releaseTagName: gh_r.tag_name.clone(),
    appDisplayName: config.appDisplayName,
    appId: config.appId,
    appShortcutName: config.appShortcutName,
    authorId: config.authorId,
    description: config.description,
    downloadUrls: HashMap::default(),
    displayImages,
    resources: Some(resources),
    app_page: config.source,
    license_or_tos: config.license_or_tos,
    install: InstallerOptions {
      linux: None,
      android: None,
      linuxArm64: None,
      linuxArm7: None,
      winarm: None,
      win32: None,
    },
    repo: config.repo,
    version,
    site: config.site,
    source: config.redistributed,
  };

  let mut num = 0;
  // Win x86_64
  windowsPlatform!(num, win32, config, gh_r, final_config, winAmd64Platform, winAmd64Options, windowsAmd64Finder);

  // Win Arm64
  windowsPlatform!(num, winarm, config, gh_r, final_config, winArm64Platform, winArm64Options, windowsArm64Finder);

  // Linux x86_64
  linuxPlatform!(num, linux, config, gh_r, final_config, linuxAmd64Platform, linuxAmd64Finder);

  // Linux Arm64
  linuxPlatform!(num, linuxArm64, config, gh_r, final_config, linuxArm64Platform, linuxArm64Finder);

  // Linux Armv7
  linuxPlatform!(num, linuxArm7, config, gh_r, final_config, linuxArm32Platform, linuxArm32Finder);

  num += 1;

  // Android Universal
  if let Some(platform) = config.platform.androidUniversal {
    if !matches!(platform, InstallerFormat::AndroidApkZip) {
      ERR.println(&"Invalid File Format, expected AndroidApkZip");
    }

    let Some(finder) = config.finder.androidUniversalFinder else {
      ERR.println(&"Android Finder Config not found!");
      process::exit(1);
    };

    let assets = find_assets(&gh_r, &finder);
    
    if assets.len() > 1 {
      ERR.println(&"Multiple assets found while parsing android");
      process::exit(1);
    }

    final_config.downloadUrls.insert(
      num,
      DownloadUrl {
        installerType: platform,
        asset: assets[0].name.clone(),
        url: "".into()
      },
    );

    final_config.install.android = Some(InstallerOptionsAndroid {
      assetId: num
    });
  }

  // if let Some(platform) = config.platform.linuxArm64Platform {
  //   let Some(finder) = config.finder.linuxFinder else {
  //     ERR.println(&"Linux Finder Config not found!");
  //     process::exit(1);
  //   };

  //   let assets = find_assets(&gh_r, &finder);

  //   if assets.len() > 1 {
  //     ERR.println(&"Multiple assets found");
  //     process::exit(1);
  //   }

  //   final_config.downloadUrls.insert(
  //     2,
  //     DownloadUrl {
  //       installerType: platform,
  //       url: assets[0].browser_download_url.clone(),
  //     },
  //   );

  //   final_config.install.linux = Some(InstallerOptionsLinux {
  //     assetId: 2
  //   });
  // }

  let config_file = to_string_pretty(&final_config).unwrap();
  let config_file = to_string(config_file.as_bytes()).unwrap();

  if !gh_action {
    println!("Bytes: ahqstore.json");
    println!("{}", &config_file);
  }

  if upload {
    let uup = gh_r
      .upload_url
      .replace("{?name,label}", &format!("?name={app_id}.txt"));

    let resp = CLIENT
      .post(uup)
      .header("Content-Length", config_file.len())
      .header("Content-Type", "text/plain")
      .header("Accept", "application/json")
      .body(config_file)
      .bearer_auth(&gh_token)
      .send()
      .unwrap()
      .text()
      .unwrap();

    if gh_action {
      let val: GHAsset = from_str(&resp).unwrap();

      println!("AHQ_STORE_FILE_URL={}", &val.browser_download_url);
    } else {
      INFO.println(&"GitHub Response");
      println!("{resp}");
    }
  }
}
