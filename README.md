# Dodecahedronic puzzle

## The puzzle

12+ pentagonal tiles, each exhibiting 5 different numbers, are to be assembled to form an icosahedron.
Each face of the icosahedron will show 3 different numbers, which sum must be e.g. `96`.

## Demo

to come

## How to run

### Native

```sh
RUST_LOG=info cargo run --release --bin native -- $SEED
```

where `SEED` is a number, used to seed rnd.

### Web

```sh
RUSTFLAGS='--cfg getrandom_backend="wasm_js"'  wasm-pack build --target web --out-name web --out-dir pkg
```

### Generating the icosahedron-dodecahedron graph

```sh
cargo run --release --bin svg
```

## TODOs

* [x] seed input, randomize button
* [x] different fonts
* [ ] align unused nums
* [ ] win animation (breaking/fading out, game controls locked, reveal dodeca?)
* [ ] estimate difficulty: vs puzzle100, shuffling triangles or not
* [ ] generate triangles +check has unique sol
* [x] fix click capture on web
* [x] facet swapping
* [x] numbers follow facets
* [x] triangle shines if correct
* [x] better font
* [x] swap anim
* [x] unmoveable facet
* [x] -> toggleable
* [x] list unused numbers
* [ ] cheat mode: no swap
* [ ] easy mode: no swap, no highlight
* [ ] hard mode: no highlight
* [ ] impossible mode: 13rd facet
* [ ] perf issues?
* [x] darker facet back / shadows again?
