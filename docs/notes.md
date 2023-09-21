---
title: Notes
---
## Issues
### 1. rust-analyzer issue:
#### description:
`` error: 'rust-analyzer' is not installed for the toolchain 'stable-aarch64-apple-darwin'``
#### solution:
```
rm -rf ~/.cargo ~/.rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
rustup uninstall stable && rustup update nightly && rustup update stable
```