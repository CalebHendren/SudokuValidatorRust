# Sudoku Validator

The Sudoku Validator is a Rust program that validates a Sudoku puzzle using multithreading. It checks if the given Sudoku puzzle is valid by ensuring that each row, column, and 3x3 subgrid contains the digits 1 through 9 without repetition.

## Prerequisites

To compile and run the Sudoku Validator program, you need to have the following installed on your system:

- Rust (version 1.x.x)

## Getting Started

1. Clone the repository or download the source code files.

2. Open a terminal or command prompt and navigate to the directory where the source code files are located.

## Compiling the Program

To compile the Sudoku Validator program, use the following command:

`rustc sudoku_validator.rs`

## Running the Program

To run the Sudoku Validator program, use the following command:

`./sudoku_validator`

This command executes the compiled Sudoku Validator program, which validates the predefined Sudoku puzzle.

## Modifying the Puzzle

By default, the program uses a predefined Sudoku puzzle `grid` for validation. If you want to validate a different Sudoku puzzle, you can modify the grid array in the `main()` function of the `sudoku_validator.rs` file.
Update the values in the `grid` array to represent your Sudoku puzzle. Each row of the puzzle corresponds to a subarray in the grid array, and each digit in the puzzle is represented by an integer value.
For example, to validate a different Sudoku puzzle, you can modify the `grid` array as follows:

`let grid = [`

    [6, 3, 4, 6, 7, 8, 9, 1, 2],
    [6, 7, 2, 1, 9, 5, 3, 4, 8],
    [8, 9, 8, 3, 4, 2, 5, 6, 7],
    [8, 5, 9, 7, 6, 1, 4, 2, 3],
    [4, 2, 6, 8, 5, 3, 7, 9, 1],
    [7, 1, 3, 9, 2, 4, 8, 5, 6],
    [9, 6, 1, 5, 3, 7, 2, 8, 4],
    [2, 8, 7, 4, 1, 9, 6, 3, 5],
    [3, 4, 5, 2, 8, 6, 1, 7, 9],
`];`

After modifying the `grid` array, save the changes and recompile the program using the `rustc` command mentioned earlier.

## Validation Result

After running the program, it will display the validation result in the console. If the Sudoku puzzle is valid, it will print `"The Sudoku puzzle is valid."` Otherwise, it will print `"The Sudoku puzzle is not valid."`

## Troubleshooting

If you encounter any issues while compiling or running the program, ensure that you have Rust installed correctly on your system.
