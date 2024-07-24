# AHQ Store Cli

Edita el archivo config.json

El esquema se muestra a continuación

## config.json

```ts
type Platform =
  // Plataformas Windows + Soporte para Actualizador y Desinstalador
  | "WindowsZip"
  | "WindowsInstallerMsi"

  // No Implementado
  | "WindowsInstallerExe"
  | "WindowsUWPMsix"

  // Plataforma Linux + Soporte para Actualizador y Desinstalador
  | "LinuxAppImage"

  // Android (En Desarrollo)
  | "AndroidLinuxZip";

interface ConfigJSON {
  [key: string]: {
    appId: string; // ID de la aplicación proporcionado por los desarrolladores de AHQ Store
    appInstalledName: string; // Nombre de la entrada en el menú de inicio + acceso directo en el escritorio
    appDisplayName: string; // Nombre de la aplicación
    authorId: string; // Tu ID de usuario
    shortDesc: string; // Descripción corta (máximo 48 palabras)
    description: string; // Descripción de la aplicación en varias líneas
    repo: {
      author: string; // Tu nombre de usuario en GitHub
      repo: string; // URL del repositorio
    };
    finder: {
      [platform: ""]: {
        startsWith?: string; // ¿Con qué debe comenzar la aplicación empaquetada?
        contains?: string; // ¿Qué debe incluir la aplicación empaquetada?
        endsWith?: string; // ¿Con qué debe terminar la aplicación empaquetada?
      };
    };
    platform: {
      // Debe ser "WindowsZip"| "WindowsInstallerMsi" |"WindowsInstallerExe" | "WindowsUWPMsix"
      winAmd64Platform?: Platform; // ¿Qué tipo de binario proporciona tu aplicación a AHQ Store?
      winArm64Platform?: Platform; // <-- Igual que winAmd64Platform -->

      linuxAmd64Platform?: Platform; // Debe ser LinuxAppImage
      linuxArm64Platform?: Platform; // Debe ser LinuxAppImage
      linuxArm64Platform?: Platform; // Debe ser LinuxAppImage
      linuxArm32Platform?: Platform; // Debe ser LinuxAppImage

      androidUniversalPlatform?: Platform; // Debe ser AndroidApkZip

      winAmd64Options?: {
        zip_file_exec?: string; // Exe para enlazar a través de nuestro instalador (WindowsZIP)
        exe_installer_args?: string[]; // Argumentos para ejecutar tu instalador personalizado (WindowsInstallerExe)
      };

      winArm64Options?: {
        zip_file_exec?: string; // Exe para enlazar a través de nuestro instalador (WindowsZIP)
        exe_installer_args?: string[]; // Argumentos para ejecutar tu instalador personalizado (WindowsInstallerExe)
      };
    };
    site?: string; // Sitio web de tu aplicación
    source?: string; // Sitio web de tu aplicación (que contiene el código fuente)
    redistributed?: string; // No puedes establecer esto
    license_or_tos?: string; // Nombre de la licencia o sitio de los términos de servicio
  };
}
```

## images/<app-id>/icon.png

El icono de tu aplicación que se incluirá en el archivo de metadatos de la aplicación

## images/<app-id>/\*

Coloca cualquier imagen (hasta 10) que se colocará en el modal de la aplicación en AHQ Store
