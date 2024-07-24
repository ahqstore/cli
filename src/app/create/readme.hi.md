# AHQ Store Cli

config.json को संपादित करें

नीचे स्कीमा दिखाया गया है

## config.json

```ts
type Platform =
  // विंडोज प्लेटफॉर्म + अपडेटर और अनइंस्टॉलर समर्थन
  | "WindowsZip"
  | "WindowsInstallerMsi"

  // लागू नहीं किया गया
  | "WindowsInstallerExe"
  | "WindowsUWPMsix"

  // लिनक्स प्लेटफॉर्म + अपडेटर और अनइंस्टॉलर समर्थन
  | "LinuxAppImage"

  // एंड्रॉइड (विकासाधीन)
  | "AndroidLinuxZip";

interface ConfigJSON {
  [key: string]: {
    appId: string; // AHQ स्टोर DEVS द्वारा प्रदान किया गया एप्लिकेशन आईडी
    appInstalledName: string; // ऐप स्टार्ट मेनू एंट्री + डेस्कटॉप शॉर्टकट नाम
    appDisplayName: string; // ऐप डिस्प्ले नाम
    authorId: string; // आपका यूजर आईडी
    shortDesc: string; // संक्षिप्त विवरण (अधिकतम 48 शब्द)
    description: string; // बहु-लाइन ऐप विवरण
    repo: {
      author: string; // आपका GitHub उपयोगकर्ता नाम
      repo: string; // रिपो URL
    };
    finder: {
      [platform: ""]: {
        startsWith?: string; // बंडल किया गया ऐप किससे शुरू होना चाहिए?
        contains?: string; // बंडल किया गया ऐप क्या शामिल होना चाहिए?
        endsWith?: string; // बंडल किया गया ऐप किससे समाप्त होना चाहिए?
      };
    };
    platform: {
      // "WindowsZip"| "WindowsInstallerMsi" |"WindowsInstallerExe" | "WindowsUWPMsix" होना चाहिए
      winAmd64Platform?: Platform; // आपका ऐप AHQ स्टोर को किस प्रकार का बाइनरी प्रदान करता है
      winArm64Platform?: Platform; // <-- winAmd64Platform के समान -->

      linuxAmd64Platform?: Platform; // LinuxAppImage होना चाहिए
      linuxArm64Platform?: Platform; // LinuxAppImage होना चाहिए
      linuxArm64Platform?: Platform; // LinuxAppImage होना चाहिए
      linuxArm32Platform?: Platform; // LinuxAppImage होना चाहिए

      androidUniversalPlatform?: Platform; // AndroidApkZip होना चाहिए

      winAmd64Options?: {
        zip_file_exec?: string; // हमारे इंस्टॉलर के माध्यम से लिंक करने के लिए Exe (WindowsZIP)
        exe_installer_args?: string[]; // आपके कस्टम इंस्टॉलर को चलाने के लिए Args (WindowsInstallerExe)
      };

      winArm64Options?: {
        zip_file_exec?: string; // हमारे इंस्टॉलर के माध्यम से लिंक करने के लिए Exe (WindowsZIP)
        exe_installer_args?: string[]; // आपके कस्टम इंस्टॉलर को चलाने के लिए Args (WindowsInstallerExe)
      };
    };
    site?: string; // आपके ऐप की वेबसाइट
    source?: string; // आपके ऐप की वेबसाइट (जिसमें स्रोत कोड शामिल है)
    redistributed?: string; // आप इसे सेट नहीं कर सकते
    license_or_tos?: string; // लाइसेंस का नाम या TOS की साइट
  };
}
```

## images/<app-id>/icon.png

आपका एप्लिकेशन आइकन जो ऐप मेटाडेटा फ़ाइल में बंडल किया जाएगा

## images/<app-id>/\*

कोई भी छवि (अधिकतम 10) रखें जो AHQ स्टोर में ऐप मोडल में रखी जाएगी
