# AHQ Store CLI

Read more about it [here](https://ahqstore.github.io)

This is the Official AHQ Store CLI and the CLI with the most number of ports.
The original CLI has been written in rust lang and we're quite excited to tell you how versatile this tool actually is. This tool is **OFFICIALLY** available and maintained for :

- [Crates.io (Original)](https://crates.io/crates/ahqstore_cli_rs)
- [npmjs (Port)](https://www.npmjs.com/package/@ahqstore/cli)
- [jsr (Port)](https://jsr.io/@ahqstore/cli)
- [PyPi (Port)](https://pypi.org/project/ahqstore-cli/)
- [Nuget (Port)](https://www.nuget.org/packages/AHQStoreCLI)
- [Pub.dev (Port)](https://pub.dev/packages/ahqstore_cli)
- [Golang (Port; See Installation Guide Below)](#golang)

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
deno install -A -f -g -n ahqstore https://jsr.io/@ahqstore/cli/0.11.4/js/cli.js
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
dart pub global deactivate ahqstore_cli
```

## .NET C#

```sh
dotnet tool install --global AHQStoreCLI
```
