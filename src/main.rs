use std::sync::{Arc, Mutex};
use std::thread;

const GRID_SIZE: usize = 9;
const SUBGRID_SIZE: usize = 3;

fn main() {
    let grid = [
        [6, 2, 4, 5, 3, 9, 1, 8, 7],
        [5, 1, 9, 7, 2, 8, 6, 3, 4],
        [8, 3, 7, 6, 1, 4, 2, 9, 5],
        [1, 4, 3, 8, 6, 5, 7, 2, 9],
        [9, 5, 8, 2, 4, 7, 3, 6, 1],
        [7, 6, 2, 3, 9, 1, 4, 5, 8],
        [3, 7, 1, 9, 5, 6, 8, 4, 2],
        [4, 9, 6, 1, 8, 2, 5, 7, 3],
        [2, 8, 5, 4, 7, 3, 9, 1, 6],
    ];

    let validator = SudokuValidator::new(grid);
    validator.validate();
}

struct SudokuValidator {
    grid: [[u8; GRID_SIZE]; GRID_SIZE],
    is_valid: Arc<Mutex<bool>>,
}

impl SudokuValidator {
    fn new(grid: [[u8; GRID_SIZE]; GRID_SIZE]) -> Self {
        SudokuValidator {
            grid,
            is_valid: Arc::new(Mutex::new(true)),
        }
    }

    fn validate(&self) {
        let mut threads = vec![];

        // Create thread for validating rows
        let grid = self.grid;
        let is_valid = Arc::clone(&self.is_valid);
        let row_thread = thread::spawn(move || {
            for row in 0..GRID_SIZE {
                if !*is_valid.lock().unwrap() {
                    break;
                }
                let mut digits = [false; GRID_SIZE + 1];
                for col in 0..GRID_SIZE {
                    let digit = grid[row][col];
                    if digit < 1 || digit > 9 || digits[digit as usize] {
                        *is_valid.lock().unwrap() = false;
                        break;
                    }
                    digits[digit as usize] = true;
                }
            }
        });
        threads.push(row_thread);

        // Create thread for validating columns
        let grid = self.grid;
        let is_valid = Arc::clone(&self.is_valid);
        let col_thread = thread::spawn(move || {
            for col in 0..GRID_SIZE {
                if !*is_valid.lock().unwrap() {
                    break;
                }
                let mut digits = [false; GRID_SIZE + 1];
                for row in 0..GRID_SIZE {
                    let digit = grid[row][col];
                    if digit < 1 || digit > 9 || digits[digit as usize] {
                        *is_valid.lock().unwrap() = false;
                        break;
                    }
                    digits[digit as usize] = true;
                }
            }
        });
        threads.push(col_thread);

        // Create threads for validating subgrids
        for i in 0..GRID_SIZE {
            let grid = self.grid;
            let is_valid = Arc::clone(&self.is_valid);
            let subgrid_thread = thread::spawn(move || {
                let row_start = (i / SUBGRID_SIZE) * SUBGRID_SIZE;
                let col_start = (i % SUBGRID_SIZE) * SUBGRID_SIZE;
                let mut digits = [false; GRID_SIZE + 1];

                for row in row_start..row_start + SUBGRID_SIZE {
                    for col in col_start..col_start + SUBGRID_SIZE {
                        let digit = grid[row][col];
                        if digit < 1 || digit > 9 || digits[digit as usize] {
                            *is_valid.lock().unwrap() = false;
                            return;
                        }
                        digits[digit as usize] = true;
                    }
                }
            });
            threads.push(subgrid_thread);
        }

        // Wait for all threads to complete
        for thread in threads {
            thread.join().unwrap();
        }

        // Check the validation result
        if *self.is_valid.lock().unwrap() {
            println!("The Sudoku puzzle is valid.");
        } else {
            println!("The Sudoku puzzle is not valid.");
        }
    }
}