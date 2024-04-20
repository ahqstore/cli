const { writeFileSync, readFileSync } = require("fs");
const { join } = require("path");
let arch = process.env.arch || require("process").arch;

if (arch == "arm64" && process.platform == "win32") {
  arch = "ia32";
}

const packageJson = join(__dirname, "dist", "package.json");

let json = JSON.parse(readFileSync(packageJson));

json.name = `@ahqstore/cli-rs-${process.platform}-${arch}`;

json.description = `AHQ Store CLI Binaries for ${process.platform}-${arch}`;
json.repository = {
  type: "git",
  url: "git+https://github.com/ahqstore/cli.git",
};
json.bugs = {
  url: "https://github.com/ahqstore/cli/issues",
};
json.homepage = "https://github.com/ahqstore/cli#readme";

writeFileSync(packageJson, JSON.stringify(json, null, 2));
