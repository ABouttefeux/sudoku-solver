use rand::thread_rng;
use sudoku::grid::Sudoku;

fn main() {
    main_predef();
    //main_random();
}

fn main_random() {
    let mut rng = thread_rng();
    let mut sudoku = Sudoku::random(25, &mut rng).unwrap();
    let mut sudoku_2 = sudoku.clone();

    println!("{}", sudoku);
    sudoku_2.try_solve().unwrap();
    println!("{}", sudoku_2);
    println!("-------");
    sudoku.solve_back_trace().unwrap();
    println!("{}", sudoku);
}

fn main_predef() {
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
    sudoku.solve_back_trace().unwrap();
}
