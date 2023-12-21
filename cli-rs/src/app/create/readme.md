# AHQ Store Cli

## config.json

```ts
interface ConfigJSON {
  [key: string]: {
    appId: string; //application id provided by AHQ Store DEVS
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

## platforms.json

```ts
// Possible Platform Values

// Windows
"64-bit Windows (ZIP)";
"64-bit Windows Installer (MSI; Not Supported Right Now)";
"64-bit Windows Installer (EXE; Not Supported Right Now)";
"UWP Windows Installer (Msix; Not Supported Right Now)";

// Linux
"64-bit Linux (AppImage; Under Development )";
```

## icon.png

Your application icon that'll be bundled in the app metadata file

## images/\*

Place any image(s) [upto 10] that will be placed in the app modal in AHQ Store
