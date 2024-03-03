# AHQ Store Cli

Do edit the config.json

The schema has been shown below

## config.json

```ts
type Platform =
  // Windows Platforms + Updater & UnInstaller Support
  | "WindowsZip"

  // Windows Platforms + No Updater Support
  | "WindowsInstallerExe"
  | "WindowsInstallerMsi"
  | "WindowsUWPMsix"

  // Linux Platform + Updater & UnInstaller Support
  | "LinuxAppImage";
type Win32Deps = "VisualCpp" | "AHQStoreAPI" | "Node21" | "Node18";
type UnixDeps = "AHQStoreAPI" | "Node21" | "Node18";

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
      repo: string; //Repo Name
    };
    finder: {
      windowsFinder?: {
        startsWith?: string; // The Windows Bundled app should startWith?
        contains?: string; // The Windows Bundled app should include?
        endsWith?: string; // The Windows Bundled app should endWith?
      };
      linuxFinder?: {
        startsWith?: string; // The Linux Bundled app should startWith?
        contains?: string; // The Linux Bundled app should include?
        endsWith?: string; // The Linux Bundled app should endWith?
      };
    };
    platform: {
      win32Platform?: Platform; // What type of binary does your app provide to AHQ Store
      linuxPlatform?: Platform; // <-- Same as win32Platform -->
      win32Options?: {
        deps: Win32Deps[]; // Win32 Custom Deps
        zip_file_exec?: string; // Exe to Link via our installer (WindowsZIP)
        exe_installer_args?: string[]; // Args to run to your custom installer (WindowsInstallerExe)
      };
      linuxOptions?: {
        deps: UnixDeps[]; // Linux Custom Deps
      };
    };
  };
}
```

## icon.png

Your application icon that'll be bundled in the app metadata file

## images/<app-id>/\*

Place any image(s) [upto 10] that will be placed in the app modal in AHQ Store
