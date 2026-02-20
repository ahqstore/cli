#!/usr/bin/env -S cargo +nightly -Zscript
---cargo
[package]
edition = "2024"

[dependencies]
ahqstore_cli_rs = { path = "..", features = ["schemars"] }
---

fn main() {
  println!("Hello World!");
}
