use core::fmt;
use std::{process::exit, thread, time::Duration, vec};

use clap::{command, error::ErrorKind, CommandFactory, Parser, ValueEnum};
use rand::{rngs::StdRng, thread_rng, Rng, SeedableRng};

fn main() {
    ctrlc::set_handler(move || {
        // Restore the cursor if our program is ctrl-c'd
        println!("\x1b[?25h");
        exit(0);
    })
    .expect("Error setting ctrl-c handler");

    let cli = Cli::parse();

    if let Some((w, h)) = term_size::dimensions() {
        if w < cli.x || h < cli.y {
            println!("Warning: Your terminal is not big enough for the size of this board.");
            println!(
                "Your board is {}x{} but your terminal is only {w}x{h}",
                cli.x, cli.y
            );
            let mut buffer = String::new();
            println!("Press any button to continue: ");
            std::io::stdin()
                .read_line(&mut buffer)
                .expect("Unable to get stdin.");
        }
    }

    let rng = if cli.seed.is_some() {
        StdRng::seed_from_u64(cli.seed.unwrap())
    } else {
        StdRng::from_rng(thread_rng()).expect("RNG generation managed to fail?")
    };

    let mut conway;
    if let Some(pattern) = cli.pattern {
        println!("Found a pattern argument, using it. ({})", pattern);
        let (x, y) = pattern.size();
        conway = Conway::new(x, y, rng);
        for (coord_x, coord_y) in pattern.coordinates() {
            conway.revive_cell(coord_x, coord_y);
        }
    } else {
        conway = Conway::new(cli.x, cli.y, rng);

        if let Some(cells) = cli.cells {
            println!(
                "Found cells as an argument, using them instead of RNG. (total: {})",
                cells.len()
            );
            for cell in cells {
                let (x, y) = cell;
                conway.revive_cell(x - 1, y - 1);
            }
        } else {
            match cli.num_cells {
                Some(n) => conway.generate_board(n),
                None => Cli::command().error(ErrorKind::Io, "No coordinate pairs were specified, but neither was the amount of random cells.").exit()
            }
        }
    }

    conway.game_loop();
}

fn clear() {
    println!("\x1b[J");
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// The x size of the conway game.
    x: usize,

    /// The y size of the conway game.
    y: usize,

    #[arg(short, long, conflicts_with_all = ["pattern", "num_cells", "seed"], value_parser = parse_coordinate_pair, num_args=0..)]
    /// A space seperated set of coordinate pairs in the form x,y
    cells: Option<Vec<(usize, usize)>>,

    #[arg(short, long, conflicts_with_all=["pattern", "cells"])]
    /// The number of cells to generate
    num_cells: Option<usize>,

    #[arg(short, long, conflicts_with_all = ["cells", "num_cells", "seed"])]
    /// The pattern to use. Note: due to the way clap parses args, you still need to provide x and y, though they will be ignored.
    pattern: Option<Pattern>,

    #[arg(short, long, conflicts_with_all = ["cells", "pattern"])]
    /// The seed to use for generation of the initial random cells. This can only be used with num_cells.
    seed: Option<u64>,
}

/// Contains vectors of coordinate setups that make cool patterns.
/// https://en.wikipedia.org/wiki/Conway's_Game_of_Life
/// Call ``coordinates`` to get the coordinate sets.
/// Call ``size`` to get the preferred size for these patterns.
#[derive(Clone, Copy, ValueEnum)]
enum Pattern {
    Block,
    Blinker,
    Beehive,
    Toad,
    Loaf,
    Beacon,
    Tub,
}

impl fmt::Display for Pattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let pattern: &str = match self {
            Self::Block => "Block",
            Self::Blinker => "Blinker",
            Self::Beehive => "Beehive",
            Self::Loaf => "Loaf",
            Self::Toad => "Toad",
            Self::Beacon => "Beacon",
            Self::Tub => "Tub",
        };
        write!(f, "{}", pattern)
    }
}

impl Pattern {
    fn coordinates(&self) -> Vec<(usize, usize)> {
        match self {
            Self::Block => vec![(2, 2), (3, 2), (2, 3), (3, 3)],
            Self::Blinker => vec![(2, 3), (3, 3), (4, 3)],
            Self::Beehive => vec![(3, 2), (4, 2), (2, 3), (4, 3), (3, 4), (4, 4)],
            Self::Loaf => vec![(3, 2), (4, 2), (2, 3), (4, 3), (3, 4), (4, 4), (4, 5)],
            Self::Toad => vec![(4, 2), (2, 3), (5, 3), (2, 4), (5, 4), (3, 5)],
            Self::Beacon => vec![(2, 2), (3, 2), (2, 3), (5, 4), (4, 5), (5, 5)],
            Self::Tub => vec![(3, 2), (2, 3), (4, 2), (3, 4)],
        }
    }

    fn size(&self) -> (usize, usize) {
        match self {
            Self::Block => (4, 4),
            Self::Blinker | Self::Tub => (5, 5),
            Self::Beehive => (6, 5),
            Self::Loaf | Self::Toad | Self::Beacon => (6, 6),
        }
    }
}

fn parse_coordinate_pair(s: &str) -> Result<(usize, usize), String> {
    match s.split(",").collect::<Vec<&str>>()[..] {
        [x, y] => match (x.parse::<usize>(), y.parse::<usize>()) {
            (Ok(x), Ok(y)) => return Ok((x, y)),
            _ => return Err("Unable to parse coordinate pair.".to_owned()),
        },
        _ => return Err("Encountered invalid coordinate set when parsing coordinates".to_owned()),
    }
}

/// Represents the current state of a cell, either alive or dead.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CellState {
    Alive,
    Dead,
}

/// Representation of a Conway's game of life board.
struct Conway {
    cells: Vec<CellState>,
    rng: StdRng,
    width: usize,
    height: usize,
}

/// Represents coordinates of neighbors in the form of offset of x, y
const NEIGHBOR_COORDINATES: [(i32, i32); 8] = [
    (-1, -1), // Top Left
    (0, -1),  // Above
    (1, -1),  // Top Right
    (-1, 0),  // Left
    (1, 0),   // Right
    (-1, 1),  // Bottom Left
    (0, 1),   // Below
    (1, 1),   // Bottom Right
];

const RESET: &'static str = "\x1B[0m";
const COLOR_GREEN: &'static str = "\x1b[31;32m";

impl Conway {
    /// Returns a Conway's board with the size of x, y
    fn new(width: usize, height: usize, rng: StdRng) -> Self {
        return Self {
            cells: vec![CellState::Dead; width * height],
            rng: rng,
            width,
            height,
        };
    }

    fn revive_cell(&mut self, x: usize, y: usize) {
        let Some(cell) = self.cells.get(x + y * self.width) else {
            Cli::command()
                .error(
                    ErrorKind::InvalidValue,
                    format!("The coordinate pair {x},{y} was out of bounds."),
                )
                .exit();
        };
        if matches!(cell, CellState::Alive) {
            println!(
                "The cell with coordinates {}, {} was already alive, skipping...",
                x, y
            );
        } else {
            if let Err(s) = self.set_cell(x, y, CellState::Alive) {
                eprintln!("{}", s);
                exit(1);
            }
        }
    }

    fn game_loop(&mut self) {
        // This is a nonstandard ansi code to make the cursor invisible.
        print!("\x1b[?25l");
        while self.tick() {
            clear();
            self.print();
            // Move the cursor to the home position (0,0)
            print!("\x1b[H");
            println!();
            thread::sleep(Duration::from_millis(500));
        }
        // print the last board before it stopped ticking.
        self.print();
        println!("\x1b[?25h");
    }

    fn print(&self) {
        for row in self.cells.chunks(self.width) {
            for cell in row {
                match cell {
                    CellState::Alive => print!("{COLOR_GREEN}#"),
                    CellState::Dead => print!(" "),
                }
            }
            println!("{RESET}");
        }
    }

    /// Randomly generates a board with a given amount of cells.
    fn generate_board(&mut self, cells: usize) {
        for _ in 0..cells {
            loop {
                let x = self.rng.gen_range(0..self.width);
                let y = self.rng.gen_range(0..self.height);
                if let Some(cell) = self.get_cell(x, y) {
                    // if the cell is not already alive, then make it so
                    match cell {
                        CellState::Alive => continue,
                        CellState::Dead => {
                            if let Err(s) = self.set_cell(x, y, CellState::Alive) {
                                eprintln!("{}", s);
                                exit(1);
                            }
                            break;
                        }
                    }
                }
            }
        }
    }

    /// Returns the amount of neighbors that a cell has that are currently alive.
    fn neighbors(&self, x: usize, y: usize) -> usize {
        if self.get_cell(x, y).is_none() {
            return 0;
        }
        let mut neighbors: usize = 0;
        for (offset_x, offset_y) in NEIGHBOR_COORDINATES.iter() {
            // Calculate the offest, and if it is invalid (i.e) -1, then skip it
            let neighbor_x = (x as i32) + offset_x;
            let neighbor_y = (y as i32) + offset_y;
            if neighbor_x < 0i32 || neighbor_y < 0i32 {
                continue;
            }

            if let Some(neighbor) = self.get_cell(neighbor_x as usize, neighbor_y as usize) {
                neighbors += match neighbor {
                    CellState::Alive => 1,
                    CellState::Dead => 0,
                }
            }
        }

        neighbors
    }

    fn get_cell(&self, x: usize, y: usize) -> Option<CellState> {
        self.cells.get(x + y * self.width).copied()
    }

    fn set_cell(&mut self, x: usize, y: usize, state: CellState) -> Result<(), String> {
        if x + y * self.width > self.cells.len() {
            return Err(format!(
                "Coordinate pair {x},{y} was out of bounds for board size {}x{}",
                self.width, self.height
            ));
        }
        self.cells[x + y * self.width] = state;

        Ok(())
    }

    /// Ticks the game board, checking if the next set of cells is alive.
    /// This will return ``true`` if the game managed to tick, else it will return ``false``.
    fn tick(&mut self) -> bool {
        let mut changed: Vec<(usize, usize, CellState)> = vec![];
        for y in 0..self.height {
            for x in 0..self.width {
                let neighbors = self.neighbors(x, y);
                let cell = self.get_cell(x, y).expect("Somehow the index for the cells were off.");
                match cell {
                    CellState::Alive => {
                        // if an alive cell has anything but 2 or 3 neighbors, it dies.
                        if neighbors < 2 || neighbors > 3 {
                            changed.push((x, y, CellState::Dead));
                        }
                    }
                    CellState::Dead => {
                        // if a dead cell has 3 neighbors, it becomes alive again.
                        if neighbors == 3 {
                            changed.push((x, y, CellState::Alive))
                        }
                    }
                }
            }
        }

        if changed.is_empty() {
            return false;
        }

        for (x, y, state) in changed {
            if let Err(s) = self.set_cell(x, y, state) {
                eprintln!("{}", s);
                exit(1);
            }
        }

        true
    }
}
