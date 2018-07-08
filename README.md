# Kompressor
!!! WORK IN PROGRESS not much is working yet !!!

Local file compression powered by WebAssembly and Rust.

## Development setup
* [install Rust](https://www.rust-lang.org/en-US/install.html) 
* ``rustup target add wasm32-unknown-unknown``
* ``cargo install cargo-web``

## Build 
* ``cargo web build --target=wasm32-unknown-unknown``

## Develop
* ``cargo web start --target=wasm32-unknown-unknown``

## Deploy
* ``cargo web deploy --target=wasm32-unknown-unknown``
* ``docker build -t kompressor-nginx``
* ``docker run --name kompressor-nginx -d -p 8080:80 kompressor-nginx``
