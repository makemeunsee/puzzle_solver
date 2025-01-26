# Solver

## How to run

### Native

```sh
cargo run --release
```

### Web

```sh
wasm-pack build --target web --out-name web --out-dir web/pkg
```

Requires a webserver running from `./web` e.g.:

```sh
python3 -m http.server --bind :: 8080
```
