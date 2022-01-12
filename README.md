# WASM Rust

A simple Web Assembly module written in Rust. 

After installing rust and cargo via [rustup](https://www.rust-lang.org/tools/install), add the wasm target and then compile the module:

    rustup target add wasm32-unknown-unknown
    cargo build --target wasm32-unknown-unknown

... acutally, just use npm:
```
$ npm run build
$ npm run start

```
