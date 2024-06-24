use rand::Rng;
use std::{thread, time};

struct Board {
    cells: Vec<Vec<Cell>>,
}

#[derive(Clone)]
struct Cell {
    state: State,
}

#[derive(Clone)]
enum State {
    Alive,
    Dead,
}

impl Board {
    fn random_state() -> State {
        let mut rng = rand::thread_rng();

        match rng.gen_bool(0.5) {
            true => State::Alive,
            false => State::Dead
        }
    }

    fn generate_cells(width: usize, height: usize) -> Vec<Vec<Cell>> {
        let mut cells = Vec::new();
        
        for _ in 0..height {
            let mut row = Vec::new();
            for _ in 0..width {
                row.push(Cell { state: Self::random_state() });
            }
            cells.push(row);
        }
        
        cells
    }

    pub fn init(width: usize, height: usize) -> Self {
        let cells = Self::generate_cells(width, height);

        Self { cells }
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

    pub fn next_generation(&mut self) {
        let cells = Self::generate_cells(self.cells.len(), self.cells[0].len());
        self.cells = cells;
    }
}

fn main() {
    let one_second = time::Duration::from_millis(1000);
    let mut board = Board::init(32, 32);
    loop {
        board.render();
        thread::sleep(one_second);
        board.next_generation();
        clearscreen::clear().expect("Failed to clear screen");
    }
}