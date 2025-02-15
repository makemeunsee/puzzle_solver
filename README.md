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
* [ ] facet swapping
* [ ] triangle shines if correct
