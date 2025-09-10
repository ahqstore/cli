# AHQ Store CLI

Read more about it [here](https://ahqstore.github.io)

This is the Official AHQ Store CLI and the CLI with the most number of ports.
The original CLI has been written in rust lang and we're quite excited to tell you how versatile this tool actually is. This tool is **OFFICIALLY** available and maintained for :

- [Crates.io (Original)](https://crates.io/crates/ahqstore_cli_rs)
- [npmjs (Port)](https://www.npmjs.com/package/@ahqstore/cli)
- [jsr (Port)](https://jsr.io/@ahqstore/cli)
- [PyPi (Port)](https://pypi.org/project/ahqstore-cli/)
- [Nuget (Port)](https://www.nuget.org/packages/AHQStoreCLI)
- [Golang (Port; Not Tested)](https://github.com/ahqstore/cli/tree/main/go)

All the platforms use the same codebase (the rust codebase). We're making use of the C-Abi to
make the CLI compatible to the following languages/runtimes:

- Cargo
- NodeJS
- Deno
- Bun
- Python
- .NET C#
- Golang

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
deno install -A -f -g -n ahqstore https://jsr.io/@ahqstore/cli/0.11.0/js/cli.js
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

To be yet figured out

## .NET C#

```sh
dotnet tool install --global AHQStoreCLI
```
