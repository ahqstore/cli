macro_rules! f {
    ($($x:tt)*) => {
        format!($($x)*)
    };
}

macro_rules! s {
    ($($x:tt)*) => {
        stringify!($($x)*)
    };
}

#[macro_export]
macro_rules! windowsPlatform {
  ($num: ident, $win: ident, $config: ident, $gh_r: ident, $final_config: ident, $platform: ident, $options: ident, $finder: ident) => {
    $num += 1;
    if let Some(platform) = $config.platform.$platform {
      if matches!(&platform, &InstallerFormat::LinuxAppImage | &InstallerFormat::AndroidApkZip) {
        ERR.println(&"Invalid File Format, expected a valid windows format");
        process::exit(1);
      } 

      let Some(options) = $config.platform.$options else {
        ERR.println(&f!("{} Options not found!", s!($win)));
        process::exit(1);
      };
      let Some(finder) = $config.finder.$finder else {
        ERR.println(&f!("{} Finder config not found!", s!($win)));
        process::exit(1);
      };
  
      let assets = crate::app::build::find_assets(&$gh_r, &finder);
  
      if assets.len() > 1 {
        ERR.println(&f!("Multiple assets found while parsing {}", s!($win)));
        process::exit(1);
      }
      if assets.len() == 0 {
        ERR.println(&f!("No assets found while parsing {}", s!($win)));
        process::exit(1);
      }
  
      $final_config.downloadUrls.insert(
        $num,
        DownloadUrl {
          installerType: platform,
          asset: assets[0].name.clone(),
          url: "".into(),
        },
      );
  
      $final_config.install.$win = Some(InstallerOptionsWindows {
        assetId: $num,
        exec: options.zip_file_exec.map_or(None, |a| Some(a.to_string())),
        installerArgs: options
          .exe_installer_args
          .map_or(None, |a| Some(a.iter().map(|x| x.to_string()).collect())),
      });
    }
  };
}

#[macro_export]
macro_rules! linuxPlatform {
    ($num: ident, $linux: ident, $config: ident, $gh_r: ident, $final_config: ident, $platform: ident, $finder: ident) => {
      $num += 1;
      if let Some(platform) = $config.platform.$platform {
        if !matches!(&platform, &InstallerFormat::LinuxAppImage) {
          ERR.println(&"Invalid File Format, expected LinuxAppImage");
        }

        let Some(finder) = $config.finder.$finder else {
          ERR.println(&f!("{} Finder Config not found!", s!($linux)));
          process::exit(1);
        };
    
        let assets = find_assets(&$gh_r, &finder);
    
        if assets.len() > 1 {
          ERR.println(&f!("Multiple assets found while parsing {}", s!($linux)));
          process::exit(1);
        }
    
        $final_config.downloadUrls.insert(
          $num,
          DownloadUrl {
            installerType: platform,
            asset: assets[0].name.clone(),
            url: "".into()
          },
        );
    
        $final_config.install.$linux = Some(InstallerOptionsLinux {
          assetId: $num
        });
      }
    };  
}