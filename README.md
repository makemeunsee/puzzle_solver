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

* [x] seed input, randomize button
* [x] different fonts
* [x] align unused nums
* [ ] win animation (breaking/fading out, game controls locked, reveal dodeca?)
* [x] estimate difficulty: vs puzzle100
* [x] estimate difficulty: shuffling triangles or not => same
* [x] estimate difficulty: generating other triangles => same
* [ ] ~~generate triangles +check has unique sol~~ done but no point in keeping it 
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
