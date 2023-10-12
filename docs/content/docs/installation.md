---
title: "Installing and running Flatlake"
nav_title: "Installing Flatlake"
nav_section: References
weight: 49
---

Flatlake is a static binary with no dynamic dependencies, so in most cases will be simple to install and run. Flatlake is currently supported on Windows, macOS, and Linux distributions.

## Running via npx

```bash
npx flatlake
```

Flatlake publishes a [wrapper package through npm](https://www.npmjs.com/package/flatlake), which is the easiest way to get started. This package will download the correct [binary of the latest release](https://github.com/CloudCannon/flatlake/releases) as an npm dependency for your platform and run it.

Specific versions can be run by passing a version tag:

```bash
npx flatlake@latest

npx flatlake@v0.1.0
```

## Downloading a precompiled binary

If you prefer to install Flatlake yourself, you can download a [precompiled release from GitHub](https://github.com/CloudCannon/flatlake/releases) and run the binary directly:

```bash
./flatlake
```

## Building from source

If you have [Rust and Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) installed, you can run `cargo install flatlake` to build from source.

```bash
cargo install flatlake
flatlake
```
