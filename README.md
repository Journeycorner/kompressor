# Kompressor
Local file compression powered by WebAssembly and Rust using the [Yew framework](https://github.com/DenisKolodin/yew).

## Development setup
* [install Rust](https://www.rust-lang.org/en-US/install.html) 
* ``rustup target add wasm32-unknown-unknown``
* ``cargo install cargo-web``

## Build 
* ``cargo web build --target=wasm32-unknown-unknown``

## Run
* ``cargo web start --target=wasm32-unknown-unknown``
