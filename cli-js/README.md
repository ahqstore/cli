# AHQ Store CLI

The CLI for devs making AHQ Store Apps

This version: NPMJS-PREBUILT-BINARIES

Install

```sh
$ npm i -g @ahqstore/cli
$ yarn global add @ahqstore/cli
$ pnpm add -g @ahqstore/cli
```

## Help Command

The help command shows all available commands

```sh
$ ahqstore --help
```

## Create .ahqstore config

Creates the .ahqstore/\* config files to make your app ready to be built

```sh
$ ahqstore --create {--force}
```

## Build a project

Builds your app based on the config files and environment variables
ENV:

> `APP_ID` (Optional) Application Id (only required if your config has more than 1 appIds)
> `RELEASE_ID` GitHub Release Id
> `GH_TOKEN` GitHub Personal Access Token / GitHub Actions Token
> `GITHUB_REPOSITORY` GitHub owner & repo name, eg ahqstore/app

```sh
$ ahqstore build
```
