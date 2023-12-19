const { writeFileSync, readFileSync } = require("fs");
const { join } = require("path");

const packageJson = join(__dirname, "dist", "package.json");

let json = JSON.parse(readFileSync(packageJson));

if (process.platform == "win32") {
  json.name = "@ahqstore/cli-rs-win32";
} else {
  json.name = "@ahqstore/cli-rs-linux";
}

writeFileSync(packageJson, JSON.stringify(json, null, 2));
