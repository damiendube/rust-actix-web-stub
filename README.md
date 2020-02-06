# README

## Installing Deps

```bash
# Install Rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# Install stable toolchain
rustup install stable
# Install none-default targets
rustup target add wasm32-wasi
rustup target add x86_64-unknown-linux-musl
```

## Building

### Same Arch as system

cargo build --release
cargo run --release

### WASM build

cargo build --release --target wasm32-wasi
wasmer run target/wasm32-wasi/release/rust-actix-web-stub.wasm

### Linux buid with static musl

CC_x86_64_unknown_linux_musl="x86_64-linux-musl-gcc" cargo build --release --target=x86_64-unknown-linux-musl

### Linux musl Docker build

docker build -t rust-actix-web-stub .
docker run --rm -it rust-actix-web-stub

## Size Differences

docker images | grep rust-actix && ls -lh target/release/rust-actix-web-stub target/x86_64-unknown-linux-musl/release/rust-actix-web-stub target/wasm32-wasi/release/rust-actix-web-stub.wasm
