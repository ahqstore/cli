// @ts-check

import { SingleBar } from "cli-progress";
import c from "ansi-colors";

import pkg from "../package.json" with { type: "json" };
import { getPrefixSuffix } from "./cli.js";
import { getRustTarget } from "./rust.js";

const bar = new SingleBar({
  format:
    "Downloading |" + c.cyan("{bar}") + "| {percentage}% || {value}/{total} KB",
  barCompleteChar: "\u2588",
  barIncompleteChar: "\u2591",
  hideCursor: true,
});

bar.start(300, 0);

bar.increment();
bar.update(20);

async function getDownload() {
  const { prefix, suffix } = getPrefixSuffix();

  return `https://github.com/ahqstore/cli/releases/download/${pkg.version}/${prefix}ahqstore_cli_rs-${getRustTarget()}.${suffix}`;
}

export async function downloadModuleWithProgress() {}
