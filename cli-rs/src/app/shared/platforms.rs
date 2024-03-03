use ahqstore_types::{InstallType, InstallerFormat, UnixDeps, Win32Deps};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct IPlatform<'a> {
  pub installType: InstallType,
  pub win32Platform: Option<InstallerFormat>,
  pub linuxPlatform: Option<InstallerFormat>,
  #[serde(borrow)]
  pub win32Options: Option<IOWin32<'a>>,
  pub linuxOptions: Option<IOLinux>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IOWin32<'a> {
  pub deps: Vec<Win32Deps>,
  #[serde(borrow)]
  pub zip_file_exec: Option<&'a str>,
  #[serde(borrow)]
  pub exe_installer_args: Option<Vec<&'a str>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IOLinux {
  pub deps: Vec<UnixDeps>,
}

impl<'a> IPlatform<'a> {
  pub fn new(platforms: Vec<InstallerFormat>) -> Self {
    let win32 = own(
      platforms
        .iter()
        .find(|p| !matches!(&p, &&&InstallerFormat::LinuxAppImage)),
    );

    let linux = own(
      platforms
        .iter()
        .find(|p| matches!(&p, &&&InstallerFormat::LinuxAppImage)),
    );

    Self {
      installType: InstallType::Computer,
      win32Platform: win32,
      linuxPlatform: linux,
      win32Options: Some(IOWin32 {
        deps: vec![],
        exe_installer_args: Some(vec![]),
        zip_file_exec: None,
      }),
      linuxOptions: Some(IOLinux { deps: vec![] }),
    }
  }
}

fn own(val: Option<&InstallerFormat>) -> Option<InstallerFormat> {
  if let Some(v) = val {
    Some(match &v {
      &&InstallerFormat::LinuxAppImage => InstallerFormat::LinuxAppImage,
      &&InstallerFormat::WindowsInstallerExe => InstallerFormat::WindowsInstallerExe,
      &&InstallerFormat::WindowsZip => InstallerFormat::WindowsZip,
      &&InstallerFormat::WindowsInstallerMsi => InstallerFormat::WindowsInstallerMsi,
      &&InstallerFormat::WindowsUWPMsix => InstallerFormat::WindowsUWPMsix,
    })
  } else {
    None
  }
}
