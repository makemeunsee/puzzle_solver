# Solver

## The puzzle

The puzzle is made of 9 blocks and an empty transparent box.
On each face of each block shows a unique number, from `1` to `54`.
The puzzle is complete when:

* the box is packed with all blocks
* the sum of the numbers of the block faces visible through each side of the box is `100`

## How to run

### Native

```sh
RUST_LOG=info cargo run --release
```

### Web

```sh
wasm-pack build --target web --out-name web --out-dir web/pkg
```

Requires a webserver running from `./web` e.g.:

```sh
python3 -m http.server --bind :: 8080
```

Then the web app is accessible at `localhost:8080`.

## Stats

The original puzzle (see [specs in the code](src/common.rs)) has:

* `1074` packing solutions when not considering the face values, i.e. as if the blocks were blank
* `1074 * 8 * 4**9 = ` packing solutions in total when considering the face values; factors detail:
    * `8`: `3` blocks have a square base and have one rotation which becomes significant
    * `4**9`: from all `9` blocks, `4` rotations become significant
* `2` full solutions

Note: the stats here discount the solutions equivalent rotations of the whole puzzle (`4`).
The volumic solver still counts them.

Note: the stats are 'measured' by running the solvers and may be incorrect due to bugs or design flaws.
