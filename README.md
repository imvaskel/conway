
# Conway

An implementation of [Conway's Game of Life](https://en.wikipedia.org/wiki/Conway's_Game_of_Life) in rust, for fun.

It comes with some features, namely preset patterns that are cool to look at from the wikipedia page, the ability to provide cells as the basis, and the ability to seed the rng that is used for generating the initial cells.




## Installation

Install conway with ``cargo install``

```bash
  cargo install https://github.com/imvaskel/conway
```

## Usage/Examples

```bash
> conway --help
Usage: conway [OPTIONS] <X> <Y>

Arguments:
  <X>  The x size of the conway game
  <Y>  The y size of the conway game

Options:
  -c, --cells [<CELLS>...]     A space seperated set of coordinate pairs in the form x,y
  -n, --num-cells <NUM_CELLS>  The number of cells to generate
  -p, --pattern <PATTERN>      The pattern to use. Note: due to the way clap parses args, you still need to provide x and y, though they will be ignored [possible values: block, blinker, beehive, toad, loaf, beacon, tub]
  -s, --seed <SEED>            The seed to use for generation of the initial random cells. This can only be used with num_cells
  -h, --help                   Print help
  -V, --version                Print version
```


## License

[MIT](https://choosealicense.com/licenses/mit/)

