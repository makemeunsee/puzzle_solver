# Dodecahedronic puzzle

## The puzzle

12+ pentagonal tiles, each exhibiting 5 different numbers, are to be assembled to form an icosahedron.
Each face of the icosahedron will show 3 different numbers, which sum must be e.g. `99`.

## Demo

https://makemeunsee.github.io/puzzle_solver/dodeca.html

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

## Complexity

Rotate only: `5^11 = 48'828'135` unique configurations.  
Average move count to solve: `~200`.

Rotate and swap: `5^11 * 11! = 1'949'062'500'000'000` unique configurations.  
Average move count to solve: `~2'200'000`.

For comparison, the [original puzzle](https://github.com/makemeunsee/puzzle_solver/) has `2'252'341'248` unique configurations and takes `~10'400'000` moves to solve.

Notes:

* 'move' = placing, rotating, removing a tile.
* counted by non-exhaustive depth-first search, see `native::pentas_on_ico`.

## TODOs

* [ ] impossible mode: 13rd tile and/or 1 tile has 2 sides
