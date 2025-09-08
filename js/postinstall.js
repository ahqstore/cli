import { getRustTarget } from "./rust.js";

import { platform, arch } from "node:process";

import c from "ansi-colors";

try {
  const target = getRustTarget();

  console.log(c.green(`Supported Target found: `) + c.yellow(`${target}`));
  console.warn(
    c.yellow(
      "The cli will download the supported binaries the first time it loads."
    )
  );
} catch (_) {
  console.error(
    c.redBright(
      `ERROR: Your OS is not supported for the cli: ${platform}-${arch} has no binaries available right now`
    )
  );
  console.error(
    c.yellowBright(
      `Want to get support for your target? Head over to https://github.com/ahqstore/cli/issues`
    )
  );

  process.exit(1);
}
