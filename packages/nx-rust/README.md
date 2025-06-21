<div align="center">
  <img src="https://raw.githubusercontent.com/nrwl/nx/master/images/nx-logo.png" width="80" alt="Nx Logo">
  <h1>@goodiebag/nx-rust</h1>
</div>

<div align="center">

[![MIT](https://img.shields.io/badge/License-MIT-blue.svg?style=flat-square)](LICENSE)
[![npm version](https://img.shields.io/npm/v/@goodiebag/nx-rust.svg?style=flat-square)](https://www.npmjs.com/package/@goodiebag/nx-rust)
[![npm downloads](https://img.shields.io/npm/dm/@goodiebag/nx-rust.svg?style=flat-square)](https://www.npmjs.com/package/@goodiebag/nx-rust)
[![commitizen](https://img.shields.io/badge/commitizen-friendly-brightgreen.svg?style=flat-square)](http://commitizen.github.io/cz-cli/)
[![PRs](https://img.shields.io/badge/PRs-welcome-brightgreen.svg?style=flat-square)](CONTRIBUTING.md)
[![styled with prettier](https://img.shields.io/badge/styled_with-prettier-ff69b4.svg?style=flat-square)](https://github.com/prettier/prettier)

</div>

---

A modern Nx plugin that adds comprehensive support for Cargo and Rust in your Nx
workspace.

> **Built upon the excellent foundation of
> [@monodon/rust](https://github.com/monodon/monodon) by the Monodon team.**  
> This fork enhances the original plugin with additional features, improved
> reliability, and modern Nx compatibility while maintaining the core philosophy
> of seamless Rust integration.

## Compatibility Chart

| @goodiebag/nx-rust | nx       |
| ------------------ | -------- |
| >=3.0.0            | >=19.0.0 |

## Getting Started

### Prerequisites

The following tools need to be installed on your system to take full advantage
of `@goodiebag/nx-rust`

- Node (LTS)
- Rust / Cargo via [https://rustup.rs](https://rustup.rs)

### Installation

Install the plugin in your existing Nx workspace:

```shell
# Using pnpm (recommended)
pnpm add -D @goodiebag/nx-rust

# Using npm
npm install -D @goodiebag/nx-rust

# Using yarn
yarn add -D @goodiebag/nx-rust
```

#### Initialization

After installing, you can run any of the project generators (binary, library) to
have @goodiebag/nx-rust set up Cargo in your workspace.

## Generators

Use Nx Console to see the full list of options for each generator.

### `@goodiebag/nx-rust:binary`

Creates a Rust binary application to be run independently.

> Create a new binary:
>
> ```shell
> nx generate @goodiebag/nx-rust:binary my-rust-app
> ```

### `@goodiebag/nx-rust:library`

Creates a Rust library that can be used in binaries, or compiled to be used for
napi.

> Create a new library:
>
> ```shell
> nx generate @goodiebag/nx-rust:library my-rust-lib
> ```

> Create a new library with napi:
>
> ```shell
> nx generate @goodiebag/nx-rust:library my-rust-node-lib --napi
> ```

#### Napi

Generating a library with the `--napi` flag will set up the project to be built
with it.

## Executors

All the executors support these additional properties:

- toolchain: (e.g. `--toolchain='stable' | 'beta' | 'nightly'`);
  - Uses `stable` by default
- target (e.g. `--target=aarch64-apple-darwin`);
- profile (e.g. `--profile=dev`)
  - [Cargo profiles](https://doc.rust-lang.org/cargo/reference/profiles.html)
- release
- target-dir
- features (e.g. `--features=bmp`)
  - [Cargo features](https://doc.rust-lang.org/cargo/reference/features.html)
- all-features
- args
  - [Arguments forwarding](https://nx.dev/nx-api/nx/executors/run-commands#args)
    to the executor.

### `@goodiebag/nx-rust:build`

Runs cargo to build the project

> Not supported with napi

### `@goodiebag/nx-rust:lint`

Runs cargo clippy to lint the project

### `@goodiebag/nx-rust:napi`

Runs the napi cli to build the project

### `@goodiebag/nx-rust:run`

Runs `cargo run` for the project

> Not supported with napi

### `@goodiebag/nx-rust:test`

Runs `cargo test` for the project
