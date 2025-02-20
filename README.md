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

### Generating the icosahedron-dodecahedron graph

```sh
cargo run --release --bin svg
```

## TODOs

* [ ] fix click capture on web
* [x] facet swapping
* [x] numbers follow facets
* [x] triangle shines if correct
* [x] better font
* [x] swap anim
* [x] unmoveable facet
* [x] -> toggleable
* [ ] different fonts
* [x] list unused numbers
* [ ] cheat mode: no swap
* [ ] easy mode: no swap, no highlight
* [ ] hard mode: no highlight
* [ ] impossible mode: 13rd facet
* [ ] random puzzle button
* [ ] seed input, +check has unique sol
* [ ] win animation (breaking/fading out, game controls locked, reveal dodeca?)
* [ ] estimate difficulty vs puzzle100
* [ ] perf issues?
* [x] darker facet back / shadows again?
* [ ] fix anchor tile bad rot
