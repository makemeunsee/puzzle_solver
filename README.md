# Solver

## The puzzle

The puzzle is made of 9 blocks and an empty transparent box.
On each face of each block shows a unique number, from `1` to `54`.

![picture of the real puzzle, solved](https://makemeunsee.github.io/puzzle_solver/the_puzzle.jpg)

The puzzle is complete when:

* the box is packed with all blocks
* the sum of the numbers of the block faces visible through each side of the box is `100`

See [the code](solvers/src/common.rs) for the full specs of the puzzle.

## Demo

https://makemeunsee.github.io/puzzle_solver/  

## How to run

### Native

```sh
RUST_LOG=info cargo run --release
```

### Web

```sh
cd web/
wasm-pack build --target web --out-name web --out-dir pkg
```

Requires a webserver running from `web/` e.g.:

```sh
python3 -m http.server --bind :: 8080
```

Then the web app is accessible at `localhost:8080`.

## Stats

As counted by the solvers, the puzzle has:

* `1074` packing solutions when not considering the face values, i.e. as if the blocks were blank
* `1074 * 2^3 * 4^9 = 2'252'341'248` packing solutions in total when considering the face values; factors detail:
    * `2^3`: `3` blocks have a square base and have one more rotation which becomes significant
    * `4^9`: from all `9` blocks, all `4` rotations of the block shape become significant
* `2` fully distinct solutions; noting each solution has a very close twin, obtained by flipping a single block, of which only face is showing.

Note: the stats here discount the solutions equivalent rotations of the whole puzzle (factor `4`).
The 'shape only' volume solver still counts them.
