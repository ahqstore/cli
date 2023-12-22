import { Chalk } from "chalk";
const chalk = new Chalk();

const platform = process.platform;

const sep = "---------------------------------";

console.log(sep);
console.log(`         AHQ Store CLI`);
console.log(sep);

const info = chalk.blue(chalk.bold("INFO:"));
const warn = chalk.yellow(chalk.bold("WARN:"));
const errr = chalk.red(chalk.bold("ERRR:"));
const success = chalk.green(chalk.bold("PASS:"));

console.log(`${warn} Checking Operating System`);
console.log(`${info} ${platform}`);

if (platform === "win32" || platform === "linux" || platform == "darwin") {
  console.log(`${success} OS Valid!`);
} else {
  console.log(`${errr} OS Invalid!`);
  console.log("NOT OK");
  throw new Error(`Invalid Platform ${platform}`);
}

const arch = process.arch;

console.log(`${warn} Checking Architecture`);
console.log(`${info} ${arch}`);

if (arch == "x64" || (platform != "win32" && arch == "arm64")) {
  console.log(`${success} Arch Validated`);
} else {
  console.log(`${errr} Arch Invalid`);
  console.log("NOT OK");
  console.log("Install using CARGO\ncargo install ahqstore_cli_rs");
  throw new Error(`Invalid Architecture ${arch}`);
}

console.log(sep);
console.log("Postinstall Script Successful");
console.log(sep);
