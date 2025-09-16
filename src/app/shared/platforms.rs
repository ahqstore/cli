use ahqstore_types::{InstallerFormat, WindowsInstallScope, AndroidAbi};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
/// # Format to be used in
/// - winAmd64Platform: Windows X64
/// - winArm64Platform : Windows Arm64
/// - linuxAmd64Platform: Linux X64
/// - linuxArm64Platform: Linux Arm64
/// - linuxArm32Platform: Linux Arm32
/// - androidUniversal: Android
/// 
/// ## is as follows:
/// (any value from the list)
/// - "WindowsZip"
/// - "WindowsInstallerMsi"
/// - "WindowsInstallerExe"
/// - "WindowsUWPMsix"
/// - "WindowsAHQDB"
/// - "LinuxAppImage"
/// - "AndroidApkZip"
pub struct IPlatform<'a> {
  pub winAmd64Platform: Option<InstallerFormat>,
  pub winArm64Platform: Option<InstallerFormat>,
  pub linuxAmd64Platform: Option<InstallerFormat>,
  pub linuxArm64Platform: Option<InstallerFormat>,
  pub linuxArm32Platform: Option<InstallerFormat>,
  pub androidUniversal: Option<InstallerFormat>,

  /// Click on IOAndroid for documentation
  pub androidOptions: Option<IOAndroid>,
  #[serde(borrow)]
  /// Click on IOWin<'a> for documentation
  pub winAmd64Options: Option<IOWin<'a>>,
  #[serde(borrow)]
  /// Click on IOWin<'a> for documentation
  pub winArm64Options: Option<IOWin<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(non_snake_case)]
/// Android Information Specification
/// # minSdk
/// - Must be a valid Android SDK
/// 
/// # abi
/// An array of the following values
/// - "Aarch64" **Arm64 Phone**
/// - "Armv7" **Arm32 Phone**
/// - "X86" **Intel/AMD 32bit Phone**
/// - "X64" **Intel/AMD 64bit Phone**
pub struct IOAndroid {
  pub minSdk: u32,
  pub abi: Vec<AndroidAbi>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
/// # zip_file_exec
/// - Executable to be linked to the Start Menu
/// - For **WindowsZip** only
/// 
/// # exe_installer_args
/// - An array of args passed to the exe installer
/// - For **WindowsInstallerExe** only
/// 
/// **The array is internally joined with " "**
/// 
/// # scope
/// - Scope of the Windows Installer
/// - For **WindowsInstallerExe** or **WindowsZip** apps only
/// - Is required for **WindowsInstallerExe**
/// - For **WindowsZip** or **WindowsInstallerExe**, keeping it empty means that it can be installed both as user or system application
/// 
/// One of the two values:
/// - "User"
/// - "Machine"
pub struct IOWin<'a> {
  #[serde(borrow)]
  pub zip_file_exec: Option<&'a str>,
  #[serde(borrow)]
  pub exe_installer_args: Option<Vec<&'a str>>,
  pub scope: Option<WindowsInstallScope>
}

impl<'a> IPlatform<'a> {
  pub fn new() -> Self {
    let io_win = IOWin {
      exe_installer_args: Some(vec![]),
      zip_file_exec: None,
      scope: None
    };

    let io_android = IOAndroid {
      minSdk: 29,
      abi: vec![AndroidAbi::Aarch64, AndroidAbi::Armv7, AndroidAbi::X86, AndroidAbi::X64]
    };

    Self {
      winAmd64Platform: None,
      winArm64Platform: None,
      linuxAmd64Platform: None,
      linuxArm32Platform: None,
      linuxArm64Platform: None,
      androidUniversal: None,
      androidOptions: Some(io_android),
      winAmd64Options: Some(io_win.clone()),
      winArm64Options: Some(io_win)
    }
  }
}