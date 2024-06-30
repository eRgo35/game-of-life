use rand::Rng;

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
    // fn dead_state() -> State {
    //     State::Dead
    // }

    // fn alive_state() -> State {
    //     State::Alive
    // }

    fn random_state() -> State {
        let mut rng = rand::thread_rng();

        match rng.gen_bool(0.5) {
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

    fn generate_cells(width: usize, height: usize, state: &dyn Fn() -> State) -> Vec<Vec<Cell>> {
        let mut cells = Vec::new();

        for _ in 0..height {
            let mut row = Vec::new();
            for _ in 0..width {
                row.push(Cell { state: state() });
            }
            cells.push(row);
        }

        cells
    }

    pub fn init(width: usize, height: usize) -> Self {
        let cells = Self::generate_cells(width, height, &Self::random_state);

        Self {
            cells,
            width,
            height,
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

    pub fn next_generation(&mut self) {
        let width = self.width;
        let height = self.height;
        let cells = self.cells.clone();
        
        let mut new_cells: Vec<Vec<Cell>> = cells.clone();

        for i in 0..width {
            for j in 0..height {
                let current_cell = &cells[i][j];
                let alive_neightbors = Self::count_alive_neightbors(&cells, i, j);

                // Main Gmae Logic
                // TODO: Optimize and remove magic numbers
                match current_cell.state {
                    State::Alive => {
                        // Any live cell with less than two or more than three live neighbours survives.
                        if !(2..=3).contains(&alive_neightbors) {
                            new_cells[i][j].state = State::Dead;
                        }
                    },
                    State::Dead => {
                        // Any dead cell with three live neighbours becomes a live cell.
                        if alive_neightbors == 3 {
                            new_cells[i][j].state = State::Alive;
                        }
                    },
                }
            }
        }

        self.cells = new_cells;

    }
}
