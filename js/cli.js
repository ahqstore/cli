#! /usr/bin/env node
// @ts-check

import { join } from "node:path";
import { existsSync, mkdirSync, rmSync } from "node:fs";
import { argv, env, platform } from "node:process";

import { downloadModuleWithProgress } from "./download.js";

import c from "ansi-colors";

import koffi from "koffi";

import pkg from "../package.json" with { type: "json" };

export function getPrefixSuffix() {
  let prefix = "";
  let suffix = "";

  switch (platform) {
    case "win32":
      suffix = ".dll";
      break;
    case "darwin":
      prefix = "lib";
      suffix = ".dylib";
      break;
    case "linux":
      prefix = "lib";
      suffix = ".so";
      break;
    default:
      prefix = "lib";
      suffix = ".so";
      console.warn(c.yellow("We're guessing a UNIX compatible system."));
  }

  return { prefix, suffix };
}

/**
 * @param {string} name
 */
function getLibraryFilename(name) {
  const { prefix, suffix } = getPrefixSuffix();

  return `${prefix}${name}${suffix}`;
}

const dylibDir = join(import.meta.dirname, "lib");
const dylib = join(dylibDir, getLibraryFilename("ahqstore_cli_rs"));

if (!(existsSync(dylibDir) && existsSync(dylib))) {
  console.warn(
    c.red.redBright(
      "Binary not found, downloading AHQ Store CLI Binary for this version",
    ),
  );

  try {
    rmSync(dylib);
  } catch(_) {}
  try {
    mkdirSync(dylibDir);
  } catch (_) {}

  await downloadModuleWithProgress(dylib);
}

let dlib;

try {
  dlib = koffi.load(dylib);
} catch (_) {
  console.warn(
    c.red.redBright(
      "Binary is corrupted, downloading AHQ Store CLI Binary for this version",
    ),
  );

    try {
    rmSync(dylib);
  } catch(_) {}
  try {
    mkdirSync(dylibDir);
  } catch (_) {}

  await downloadModuleWithProgress(dylib);

  dlib = koffi.load(dylib);
}

const ver = dlib.func("get_ver", "str", []);

/**
 * Note that we've leaked some memory here
 * @type {string}
 */
const output = ver();

if (output != pkg.version) {
  console.warn(c.red.yellowBright("We need to update binaries..."));

  // Unload current one
  dlib.unload();

  try {
    rmSync(dylib);
  } catch(_) {}
  try {
    mkdirSync(dylibDir);
  } catch (_) {}

  /// Download
  await downloadModuleWithProgress(dylib);

  // Load newer
  dlib = koffi.load(dylib);
}

dlib.func("init_args", "void", [])();

const pushArg = dlib.func("add_arg", "void", ["str"]);

argv.slice(2).forEach((a) => {
  pushArg(a);
});

// Clear the console output
console.clear();

dlib.func("node_entrypoint", "void", ["bool"])(env["CI"] == "true");
