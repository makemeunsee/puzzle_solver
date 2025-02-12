# Dodecahedronic puzzle

## The puzzle

12+ pentagonal tiles, each exhibiting 5 different numbers, are to be assembled to form an icosaedron.
Each face of the icosaedron will show 3 different numbers, which sum must be e.g. `96`.

## Demo

to come

## How to run

### Native

```sh
RUST_LOG=info cargo run --release --bin native
```

### Generating the icosaedron-dodecahedron graph

```sh
cargo run --release --bin svg
```
