#! /usr/bin/env node
import { Chalk } from "chalk";

const platform = process.platform;
const arch = process.platform;
const args = process.argv.slice(2) || [];

(async () => {
  try {
    (await import(`@ahqstore/cli-rs-${platform}-${arch}`)).nodeEntrypoint(args);
  } catch (e) {
    const chalk = new Chalk();
    console.log(chalk.red(e));
    console.log(
      chalk.red(
        chalk.bold(
          "Error running cli, consider using cargo version\ncargo install ahqstore_cli_rs"
        )
      )
    );
  }
})();
