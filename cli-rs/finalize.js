const { writeFileSync, readFileSync } = require("fs");
const { join } = require("path");

const packageJson = join(__dirname, "dist", "package.json");

let json = JSON.parse(readFileSync(packageJson));
json.name = "@ahqstore/cli-rs";

writeFileSync(packageJson, JSON.stringify(json, null, 2));
