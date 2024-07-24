use ahqstore_types::InstallerFormat;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct IPlatform<'a> {
  pub winAmd64Platform: Option<InstallerFormat>,
  pub winArm64Platform: Option<InstallerFormat>,
  pub linuxAmd64Platform: Option<InstallerFormat>,
  pub linuxArm64Platform: Option<InstallerFormat>,
  pub linuxArm32Platform: Option<InstallerFormat>,
  pub androidUniversal: Option<InstallerFormat>,
  #[serde(borrow)]
  pub winAmd64Options: Option<IOWin<'a>>,
  #[serde(borrow)]
  pub winArm64Options: Option<IOWin<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IOWin<'a> {
  #[serde(borrow)]
  pub zip_file_exec: Option<&'a str>,
  #[serde(borrow)]
  pub exe_installer_args: Option<Vec<&'a str>>,
}

impl<'a> IPlatform<'a> {
  pub fn new() -> Self {
    let io_win = IOWin {
      exe_installer_args: Some(vec![]),
      zip_file_exec: None,
    };

    Self {
      winAmd64Platform: None,
      winArm64Platform: None,
      linuxAmd64Platform: None,
      linuxArm32Platform: None,
      linuxArm64Platform: None,
      androidUniversal: None,
      winAmd64Options: Some(io_win.clone()),
      winArm64Options: Some(io_win)
    }
  }
}