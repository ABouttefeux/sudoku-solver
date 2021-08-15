use sudoku::grid::Sudoku;

fn main() {
    let mut sudoku = Sudoku::new([
        [3, 0, 0, 6, 0, 0, 0, 9, 0],
        [0, 4, 0, 0, 2, 0, 0, 5, 0],
        [0, 8, 0, 0, 7, 0, 1, 6, 0],
        [9, 0, 0, 3, 0, 4, 7, 0, 0],
        [0, 5, 0, 0, 8, 0, 0, 2, 0],
        [0, 0, 1, 9, 0, 0, 0, 0, 6],
        [0, 2, 7, 0, 3, 0, 0, 4, 0],
        [0, 9, 0, 0, 6, 0, 0, 1, 0],
        [0, 3, 0, 0, 0, 5, 0, 0, 8],
    ]);
    println!("{}", sudoku);
    let _ = sudoku.try_solve().unwrap();
    println!();
    println!("{}", sudoku);
}
