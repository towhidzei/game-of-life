use std::{
    io::Write,
    time::{SystemTime, UNIX_EPOCH},
};

/// Represents the state of a cell in the Game of Life.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum CellState {
    Dead,
    Alive,
}

/// Represents the Game of Life grid.
pub struct GameOfLife {
    grid: Vec<Vec<CellState>>,      // Current state
    prev_grid: Vec<Vec<CellState>>, // Previous state
}

impl GameOfLife {
    /// Creates a new Game of Life instance with a given width and height.
    pub fn new(width: usize, height: usize) -> Self {
        let empty_grid = vec![vec![CellState::Dead; width]; height];
        Self {
            grid: empty_grid.clone(),
            prev_grid: empty_grid,
        }
    }

    /// Randomly initializes the grid with alive cells (around 3% chance per cell).
    pub fn randomize(&mut self) -> &mut Self {
        let mut seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos(); // Use system time as a pseudo-random seed

        for row in &mut self.grid {
            for cell in row {
                // Simple pseudo-random generation using bitwise operations
                seed ^= seed << 13;
                seed ^= seed >> 7;
                seed ^= seed << 17;

                let chance = 15; // percentage of chance to being alive
                *cell = if seed % 100 < chance {
                    CellState::Alive
                } else {
                    CellState::Dead
                };
            }
        }
        self
    }

    /// Counts the number of alive neighbors for a given cell.
    fn count_alive_neighbors(&self, row: usize, col: usize) -> u8 {
        let directions = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        let mut count = 0;

        for (dx, dy) in directions.iter() {
            let new_row = row as isize + dx;
            let new_col = col as isize + dy;

            if new_row >= 0
                && new_row < self.grid.len() as isize
                && new_col >= 0
                && new_col < self.grid[0].len() as isize
            {
                if let CellState::Alive = self.grid[new_row as usize][new_col as usize] {
                    count += 1;
                }
            }
        }

        count
    }

    /// Advances the game to the next state following Conway's Game of Life rules.
    pub fn next_generation(&mut self) -> &mut Self {
        let mut new_state = self.grid.clone(); // Create a new grid for the next state

        for (i, row) in self.grid.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                let neighbors = self.count_alive_neighbors(i, j);
                new_state[i][j] = match (cell, neighbors) {
                    (CellState::Alive, 2) | (CellState::Alive, 3) => CellState::Alive,
                    (CellState::Dead, 3) => CellState::Alive,
                    _ => CellState::Dead,
                };
            }
        }

        self.prev_grid = self.grid.clone(); // Store the previous state
        self.grid = new_state; // Update the current state

        self
    }

    /// Displays the current state of the grid in the terminal.
    pub fn display(&self) {
        // Clear screen using ANSI escape codes
        print!("\x1B[2J\x1B[H");

        for row in &self.grid {
            for cell in row.iter() {
                match cell {
                    CellState::Alive => print!("██"), // Alive cell
                    CellState::Dead => print!("  "),  // Dead cell
                }
            }
            println!();
        }

        // Flush output for immediate update
        std::io::stdout().flush().unwrap();
    }

    /// Initializes the **Gosper Glider Gun** at a predefined position.
    pub fn add_gosper_glider_gun(&mut self) {
        let x = 5;
        let y = 1;

        let gun_pattern = [
            (x, y + 24),
            (x + 1, y + 22),
            (x + 1, y + 24),
            (x + 2, y + 12),
            (x + 2, y + 13),
            (x + 2, y + 20),
            (x + 2, y + 21),
            (x + 2, y + 34),
            (x + 2, y + 35),
            (x + 3, y + 11),
            (x + 3, y + 15),
            (x + 3, y + 20),
            (x + 3, y + 21),
            (x + 3, y + 34),
            (x + 3, y + 35),
            (x + 4, y),
            (x + 4, y + 1),
            (x + 4, y + 10),
            (x + 4, y + 16),
            (x + 4, y + 20),
            (x + 4, y + 21),
            (x + 5, y),
            (x + 5, y + 1),
            (x + 5, y + 10),
            (x + 5, y + 14),
            (x + 5, y + 16),
            (x + 5, y + 17),
            (x + 5, y + 22),
            (x + 5, y + 24),
            (x + 6, y + 10),
            (x + 6, y + 16),
            (x + 6, y + 24),
            (x + 7, y + 11),
            (x + 7, y + 15),
            (x + 8, y + 12),
            (x + 8, y + 13),
        ];

        for &(row, col) in &gun_pattern {
            self.grid[row][col] = CellState::Alive;
        }
    }
}
