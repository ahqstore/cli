/**
 * Maps the Node.js platform and architecture to a Rust target triple.
 * @returns {string} The Rust target triple (e.g., 'x86_64-pc-windows-msvc').
 * @throws {Error} If the platform/architecture combination is not supported.
 */
export function getRustTarget() {
  const os = process.platform;
  const arch = process.arch;

  if (os === "win32") {
    if (arch === "ia32") {
      return "i686-pc-windows-msvc";
    } else if (arch === "x64") {
      return "x86_64-pc-windows-msvc";
    } else if (arch === "arm64") {
      return "aarch64-pc-windows-msvc";
    }
  } else if (os === "darwin") {
    if (arch === "x64") {
      return "x86_64-apple-darwin";
    } else if (arch === "arm64") {
      return "aarch64-apple-darwin";
    }
  } else if (os === "linux") {
    if (arch === "ia32") {
      return "i686-unknown-linux-gnu";
    } else if (arch === "x64") {
      return "x86_64-unknown-linux-gnu";
    } else if (arch === "arm") {
      return "armv7-unknown-linux-gnueabihf";
    } else if (arch === "arm64") {
      return "aarch64-unknown-linux-gnu";
    }
  }

  throw new Error(`Unsupported platform: ${os} ${arch}`);
}
