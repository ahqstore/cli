const { writeFileSync, readFileSync } = require("fs");
const { join } = require("path");
const arch = process.env.arch || require("process").arch;

const packageJson = join(__dirname, "dist", "package.json");

let json = JSON.parse(readFileSync(packageJson));

json.name = `@ahqstore/cli-rs-${process.platform}`;

if (arch != "x64") {
  json.name = `@ahqstore/cli-rs-${process.platform}-${arch}`;
}

json.description = `AHQ Store CLI Binaries for ${process.platform}-${process.arch}`;
json.repository = {
  type: "git",
  url: "git+https://github.com/ahqstore/cli.git",
};
json.bugs = {
  url: "https://github.com/ahqstore/cli/issues",
};
json.homepage = "https://github.com/ahqstore/cli#readme";

writeFileSync(packageJson, JSON.stringify(json, null, 2));
