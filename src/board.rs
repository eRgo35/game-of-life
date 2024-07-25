use std::{io::{Read, Write}, path::PathBuf};

use rand::{Rng, SeedableRng};

#[derive(Clone, PartialEq)]
enum State {
    Alive,
    Dead,
}

#[derive(Clone)]
pub struct Cell {
    state: State,
}

pub struct Board {
    cells: Vec<Vec<Cell>>,
    width: usize,
    height: usize,
}

impl Board {
    fn random_state(probability: f64, seed: u64, row: usize, col: usize) -> State {
        let mut rng: rand::rngs::StdRng;

        if seed == 0 {
            rng = rand::rngs::StdRng::from_entropy();
        } else {
            rng = rand::rngs::StdRng::seed_from_u64(seed * row as u64 + col as u64);
        }

        match rng.gen_bool(probability) {
            true => State::Alive,
            false => State::Dead,
        }
    }

    fn count_alive_neightbors(cells: &[Vec<Cell>], i: usize, j: usize) -> usize {
        let mut alive_neightbors = 0;

        let a_lim = cells.len() as i32;
        let b_lim = cells[i].len() as i32;

        for a in 0..3 {
            for b in 0..3 {
                let row: i32 = i as i32 + 1 - a;
                let col: i32 = j as i32 + 1 - b;

                if a == 1 && b == 1 {
                    continue;
                }

                if row < 0 || col < 0 || row >= a_lim || col >= b_lim {
                    continue;
                }

                if cells[row as usize][col as usize].state == State::Alive {
                    alive_neightbors += 1;
                }
            }
        }

        alive_neightbors
    }

    fn generate_cells(width: usize, height: usize, probability: f64, seed: u64) -> Vec<Vec<Cell>> {
        let mut cells = Vec::new();

        for i in 0..height {
            let mut row = Vec::new();
            for j in 0..width {
                row.push(Cell { state: Self::random_state(probability, seed, i, j) });
            }
            cells.push(row);
        }

        cells
    }

    pub fn init(width: usize, height: usize, probability: f64, seed: u64) -> Self {
        let cells = Self::generate_cells(width, height, probability, seed);

        Self {
            cells,
            width,
            height,
        }
    }

    pub fn load(path: String) -> Self {
        let path = PathBuf::from(path);

        let mut file = match std::fs::File::open(path) {
            Ok(file) => file,
            Err(err) => panic!("{}", err),
        };

        let mut contents = String::new();
        if let Err(err) = file.read_to_string(&mut contents) {
            panic!("{}", err);
        }

        let width = contents.lines().next().unwrap().len();
        let height = contents.lines().count();
        let mut cells = vec![vec![Cell { state: State::Dead }; width]; height];

        println!("{}x{}", width, height);

        for (i, line) in contents.lines().enumerate() {
            for (j, c) in line.chars().enumerate() {
                match c {
                    'X' => cells[i][j].state = State::Alive,
                    ' ' => cells[i][j].state = State::Dead,
                    _ => panic!("Invalid character in file: {}. Use X to mark alive cells and space to mark dead cells.", c),
                }
            }
        }

        Self {
            cells,
            width,
            height,
        }
    }

    pub fn save(&self, path: String) {
        let path = PathBuf::from(path);
        let mut file = match std::fs::File::create(path) {
            Ok(file) => file,
            Err(err) => panic!("{}", err),
        };

        let mut contents = String::new();
        for row in &self.cells {
            for cell in row {
                match cell.state {
                    State::Alive => contents.push('X'),
                    State::Dead => contents.push(' '),
                }
            }
            contents.push('\n');
        }

        if let Err(err) = file.write(contents.as_bytes()) {
            panic!("{}", err);
        }
    }

    pub fn render(&self) {
        for row in &self.cells {
            for cell in row {
                match cell.state {
                    State::Alive => print!("██"),
                    State::Dead => print!("  "),
                }
            }
            println!();
        }
    }

    pub fn next_generation(&mut self, overpopulation: usize, underpopulation: usize, repopulation: usize) {
        let width = self.width;
        let height = self.height;
        let cells = self.cells.clone();
        
        let mut new_cells: Vec<Vec<Cell>> = cells.clone();

        for i in 0..width {
            for j in 0..height {
                let current_cell = &cells[i][j];
                let alive_neightbors = Self::count_alive_neightbors(&cells, i, j);

                // Main Game of Life Logic
                match current_cell.state {
                    State::Alive => {
                        // Any live cell with less than two or more than three live neighbours survives.
                        if !(underpopulation..=overpopulation).contains(&alive_neightbors) {
                            new_cells[i][j].state = State::Dead;
                        }
                    },
                    State::Dead => {
                        // Any dead cell with three live neighbours becomes a live cell.
                        if alive_neightbors == repopulation {
                            new_cells[i][j].state = State::Alive;
                        }
                    },
                }
            }
        }

        self.cells = new_cells;

    }
}

