# AHQ Store Cli

Do edit the config.json

The schema has been shown below

## config.json

```ts
type Platform =
  // Windows Platforms + Updater & Uninstaller Support
  | "WindowsZip"
  | "WindowsInstallerMsi"

  // Not Implemented
  | "WindowsInstallerExe"
  | "WindowsUWPMsix"

  // Linux Platform + Updater & Uninstaller Support
  | "LinuxAppImage"

  // Android (Under Development)
  | "AndroidLinuxZip";

interface ConfigJSON {
  [key: string]: {
    appId: string; //application id provided by AHQ Store DEVS
    appInstalledName: string; //App Start Menu Entry + Desktop Shortcut Name
    appDisplayName: string; //App display name
    authorId: string; //Your User ID
    shortDesc: string; //Short Description (max 48words)
    description: string; //MultiLine App Description
    repo: {
      author: string; //Your GitHub username
      repo: string; //Repo URL
    };
    finder: {
      [platform: ""]: {
        startsWith?: string; // The Bundled app should startWith?
        contains?: string; // The Bundled app should include?
        endsWith?: string; // The Bundled app should endWith?
      };
    };
    platform: {
      // Must be "WindowsZip"| "WindowsInstallerMsi" |"WindowsInstallerExe" | "WindowsUWPMsix"
      winAmd64Platform?: Platform; // What type of binary does your app provide to AHQ Store
      winArm64Platform?: Platform; // <-- Same as winAmd64Platform -->

      linuxAmd64Platform?: Platform; // Must be LinuxAppImage
      linuxArm64Platform?: Platform; // Must be LinuxAppImage
      linuxArm64Platform?: Platform; // Must be LinuxAppImage
      linuxArm32Platform?: Platform; // Must be LinuxAppImage

      androidUniversalPlatform?: Platform; // Must be AndroidApkZip

      winAmd64Options?: {
        zip_file_exec?: string; // Exe to Link via our installer (WindowsZIP)
        exe_installer_args?: string[]; // Args to run to your custom installer (WindowsInstallerExe)
      };

      winArm64Options?: {
        zip_file_exec?: string; // Exe to Link via our installer (WindowsZIP)
        exe_installer_args?: string[]; // Args to run to your custom installer (WindowsInstallerExe)
      };
    };
    site?: string; // Your app's website
    source?: string; // Your app's website (that contains source code)
    redistributed?: string; // You just **cannot** set this
    license_or_tos?: string; // Name of License or site of TOS
  };
}
```

## images/<app-id>/icon.png

Your application icon that'll be bundled in the app metadata file

## images/<app-id>/\*

Place any image(s) [upto 10] that will be placed in the app modal in AHQ Store
