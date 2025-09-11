# AHQ Store CLI

[![crates.io](https://img.shields.io/crates/v/ahqstore_cli_rs)](https://crates.io/crates/ahqstore_cli_rs)
[![npm](https://img.shields.io/npm/v/@ahqstore/cli)](https://www.npmjs.com/package/@ahqstore/cli)
[![JSR Version](https://img.shields.io/jsr/v/@ahqstore/cli)](https://jsr.io/@ahqstore/cli)
[![PyPI](https://img.shields.io/pypi/v/ahqstore-cli)](https://pypi.org/project/ahqstore-cli/)
[![NuGet](https://img.shields.io/nuget/v/AHQStoreCLI)](https://www.nuget.org/packages/AHQStoreCLI)
[![pub.dev](https://img.shields.io/pub/v/ahqstore_cli)](https://pub.dev/packages/ahqstore_cli)
[![Build](https://github.com/ahqstore/cli/actions/workflows/publish.yml/badge.svg)](https://github.com/ahqstore/cli/actions/workflows/publish.yml)

---

Read more about it [here](https://ahqstore.github.io)

The **official cross-platform AHQ Store CLI**, built in Rust and exported everywhere via C ABI.  
One codebase → many runtimes:

The original CLI has been written in rust lang and we're quite excited to tell you how versatile this tool actually is. This tool is **OFFICIALLY** available and maintained for :

- [Crates.io (Original)](https://crates.io/crates/ahqstore_cli_rs)
- [npmjs (Port)](https://www.npmjs.com/package/@ahqstore/cli)
- [jsr (Port)](https://jsr.io/@ahqstore/cli)
- [PyPi (Port)](https://pypi.org/project/ahqstore-cli/)
- [Nuget (Port)](https://www.nuget.org/packages/AHQStoreCLI)
- [Pub.dev (Port)](https://pub.dev/packages/ahqstore_cli)
- [Golang (Port; See Installation Guide Below)](#golang)

Prebuilt binaries available for Linux, macOS, and Windows (x64, arm64, i686, armv7).

All the platforms use the same codebase (the rust codebase). We're making use of the C-Abi to
make the CLI compatible to the following languages/runtimes:

- Cargo
- NodeJS
- Deno
- Bun
- Python
- .NET C#
- Golang
- Dart

# Usage

Head over to https://ahqstore.github.io/guide/cli/ for usage references!

# Installation

## Rust

There are two ways to install in Rust Lang, `cargo install` and `cargo binstall`

## cargo install (official)

```sh
cargo install ahqstore_cli_rs
```

### cargo binstall

```sh
cargo binstall ahqstore_cli_rs
```

## NodeJS

```sh
npm i -g @ahqstore/cli
```

## Deno

### Using npmjs

```sh
deno install -g npm:@ahqstore/cli
```

### Using JSR

```sh
deno install -A -f -g -n ahqstore https://jsr.io/@ahqstore/cli/0.12.1/js/cli.js
```

## Bun

```sh
bun install -g @ahqstore/cli
```

## Python

```sh
pip install ahqstore-cli
```

## GoLang

Since GoLang mainly works with repositories. We've set up a mirror repo so that it works as expected. Here's the install command :-

```sh
go install github.com/ahqstore/cli-go/ahqstore@latest
```

## Dart

```sh
dart pub global activate ahqstore_cli
```

## .NET C#

```sh
dotnet tool install --global AHQStoreCLI
```
