use rand::thread_rng;
use sudoku::grid::Sudoku;

fn main() {
    main_predef();
    main_random();
}

fn main_random() {
    let mut rng = thread_rng();
    let mut sudoku = Sudoku::<3>::random(45, &mut rng).unwrap();

    println!("{}", sudoku);
    sudoku.try_solve().unwrap();
    println!("{}", sudoku);
    println!("-------");
    match sudoku.solve_back_trace() {
        Ok(()) => println!("{}", sudoku),
        Err(error) => println!("error: {}", error),
    }
}

fn main_predef() {
    let mut sudoku = Sudoku::<3>::new([
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

    let mut sudoku = Sudoku::<3>::new([
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
