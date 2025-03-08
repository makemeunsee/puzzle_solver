# Icosahedronic puzzle

## Demo

TODO

## How to run

### Native

```sh
RUST_LOG=info cargo run --release --bin native
```

### Web

Build command:

```sh
cd web/
RUSTFLAGS='--cfg getrandom_backend="wasm_js"'  wasm-pack build --target web --out-name web --out-dir pkg
```

Requires a webserver running from `web/` e.g.:

```sh
python3 -m http.server --bind :: 8080
```

Then the web app is accessible at `localhost:8080`.

### Generating the icosahedron-dodecahedron graph

```sh
cargo run --release --bin svg
```

## TODOs

* [ ] fix picking with variable projection
