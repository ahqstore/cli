#! /usr/bin/env node
const platform = process.platform;
const args = process.argv.slice(2) || [];

(async () => {
  if (platform == "win32") {
    (await import("@ahqstore/cli-rs-win32")).nodeEntrypoint(args);
  } else {
    (await import("@ahqstore/cli-rs-linux")).nodeEntrypoint(args);
  }
})();
