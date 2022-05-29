# Dominator-example

Counter app using [`dominator-rs`][dominator] and [`xtask-wasm`][xtask-wasm].

## Usage

* Generate the distributed package:

```
cargo xtask dist
```

The distributed package can be optimized via [`wasm-opt`][wasm-opt]:

```
cargo xtask dist --optimize
```

* Launch `cargo check` and relaunch if changes in project's files is detected:

```
cargo xtask watch
```

* Serve the distributed package at http://127.0.0.1:8000 and watch for changes in the project:

```
cargo xtask start
```

[dominator]: https://github.com/Pauan/rust-dominator
[xtask-wasm]: https://github.com/rustminded/xtask-wasm
[wasm-opt]: https://github.com/WebAssembly/binaryen#tools
