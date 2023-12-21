## rk

reprogrammable (or rust) kernel

### Setup

You need rustup, cargo.

```bash
rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu  # or windows equivalent
cargo install bootimage
rustup component add llvm-tools
```

### Run

```bash
cargo build
cargo run
```

