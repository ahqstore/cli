const { writeFileSync, readFileSync } = require("fs");
const { join } = require("path");

const packageJson = join(__dirname, "dist", "package.json");

let json = JSON.parse(readFileSync(packageJson));

if (process.platform == "win32") {
  json.name = "@ahqstore/cli-rs-win32";
} else if (process.platform == "linux") {
  json.name = "@ahqstore/cli-rs-linux";
} else if (process.platform == "darwin") {
  json.name = "@ahqstore/cli-rs-darwin";
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
