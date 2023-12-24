# AHQ Store Cli

Edit the following files

- config.json
- finder.json
- platforms.json

The schema has been shown below

## config.json

```ts
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
  };
}
```

## finder.json

```ts
interface FinderJSON {
  windowsFinder?: {
    startsWith?: string;
    contains?: string;
    endsWith?: string;
  };
  linuxFinder?: {
    startsWith?: string;
    contains?: string;
    endsWith?: string;
  };
}
```

## platforms.json

Each Platform Definition

- WindowsZip: 64-bit Windows ZIP
- WindowsInstallerExe: 64-bit Windows Installer Exe
- WindowsInstallerMsi: 64-bit Windows Installer Msi
- WindowsUWPMsix: UWP Windows Msix Package
- LinuxAppImage: 64-bit Linux AppImage

```ts
type Platform = "WindowsZip" | "WindowsInstallerExe" | "WindowsInstallerMsi" | "WindowsUWPMsix" | "LinuxAppImage";
type Win32Deps = "VisualCpp" | "AHQStoreAPI" | "Node21" |"Node18";
type UnixDeps = "AHQStoreAPI" | "Node21" | "Node18";

interface type PlatformJSON {
  win32Platform?: Platform;
  linuxPlatform?: Platform;
  win32Options?: {
    deps: Win32Deps[];
    zip_file_exec?: string;
    exe_installer_args?: string[];
  };
  linuxOptions?: {
    deps: UnixDeps[];
  };
}
```

## icon.png

Your application icon that'll be bundled in the app metadata file

## images/\*

Place any image(s) [upto 10] that will be placed in the app modal in AHQ Store
