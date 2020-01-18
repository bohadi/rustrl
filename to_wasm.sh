#! /usr/bin/sh

cargo build --release --target wasm32-unknown-unknown && \
wasm-bindgen target/wasm32-unknown-unknown/release/rustrl.wasm --out-dir ./wasm/ --no-modules --no-typescript && \
python -m http.server
