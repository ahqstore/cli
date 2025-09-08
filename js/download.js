// @ts-check

import { SingleBar } from "cli-progress";
import { getPrefixSuffix } from "./cli.js";
import { getRustTarget } from "./rust.js";
import { createWriteStream } from "node:fs";

import c from "ansi-colors";
import pkg from "../package.json" with { type: "json" };

const bar = new SingleBar({
  format:
    "Downloading |" + c.cyan("{bar}") + "| {percentage}% | {value}/{total} KB",
  barCompleteChar: "\u2588",
  barIncompleteChar: "\u2591",
  hideCursor: true,
});

function getDownload() {
  const { prefix, suffix } = getPrefixSuffix();

  return `https://github.com/ahqstore/cli/releases/download/${pkg.version}/${prefix}ahqstore_cli_rs-${getRustTarget()}${suffix}`;
}

/**
 * 
 * @param {number} ms 
 * @returns {Promise<undefined>}
 */
const delay = (ms) => new Promise((resolve) => setTimeout(() => resolve(undefined), ms));

/**
 * 
 * @param {string} file 
 * @returns {Promise<undefined>}
 */
export async function downloadModuleWithProgress(file) {
  const download = getDownload();

  const stream = createWriteStream(file, {
    autoClose: true,
    flush: true
  });

  const res = await fetch(
    download,
    {
      headers: {
        "user-agent": "Downloader"
      }
    }
  );

  const bytes = parseInt(res.headers.get("content-length") || "0");
  const kb = Math.round(bytes / 1024);

  bar.start(kb, 0);

  const reader = res.body?.getReader()

  let curr = 0;

  while (true) {
    const buf = await reader?.read();

    if (!buf || !buf.value) {
      bar.update(kb);
      bar.stop();

      await new Promise((resolve) => {
        stream.on('close', () => resolve(undefined));
        stream.end();
      });
      break;
    }

    const toAdd = buf.value.length || 0;

    curr += Math.round(toAdd / 1024);

    bar.update(curr);

    stream.write(buf.value);

    await delay(1);
  }
}
