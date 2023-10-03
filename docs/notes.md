---
title: Notes
---
## Issues
### 1. rust-analyzer issue:
#### description:
`` error: 'rust-analyzer' is not installed for the toolchain 'stable-aarch64-apple-darwin'``
#### solution:
```bash
rm -rf ~/.cargo ~/.rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
rustup uninstall stable && rustup update nightly && rustup update stable
```
## Commands
### Rust configuration check
To see what Rust toolchain you are presently using, run:

```bash
rustup show
```

To ensure your Rust compiler is always up to date, you should run:

```bash
rustup update
rustup update nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
```