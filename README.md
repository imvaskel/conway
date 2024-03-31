# Conway

![Demo](.github/demo.gif)

An implementation of [Conway's Game of Life](https://en.wikipedia.org/wiki/Conway's_Game_of_Life) in rust, for fun.

It comes with some features, namely preset patterns that are cool to look at from the wikipedia page, the ability to provide cells as the basis, and the ability to seed the rng that is used for generating the initial cells.

## Installation

Install conway with `cargo install`

```bash
  cargo install --git https://github.com/imvaskel/conway
```

## Usage/Examples

```bash
> conway --help
Usage: conway [OPTIONS] [WIDTH] [HEIGHT]

Arguments:
  [WIDTH]   The width of the Conway board
  [HEIGHT]  The height of the Conway board

Options:
  -c, --cells [<CELLS>...]     A space seperated set of coordinate pairs in the form x,y
  -n, --num-cells <NUM_CELLS>  The number of cells to generate. If not provided, the default is a 50% chance per cell
  -p, --pattern <PATTERN>      The pattern to use [possible values: block, blinker, beehive, toad, loaf, beacon, tub]
  -s, --seed <SEED>            The seed to use for generation of the initial random cells. This can only be used with num_cells
  -h, --help                   Print help
  -V, --version                Print version
```

## License

[MIT](https://choosealicense.com/licenses/mit/)
