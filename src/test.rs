mod version_number;

use crate::grid::{CellPosition, Sudoku, VerificationError, VerificationResult};

#[test]
fn sudoku_solving_deducation() -> Result<(), VerificationError> {
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

    let sudoku_solution = Sudoku::new([
        [3, 1, 5, 6, 4, 8, 2, 9, 7],
        [7, 4, 6, 1, 2, 9, 8, 5, 3],
        [2, 8, 9, 5, 7, 3, 1, 6, 4],
        [9, 6, 2, 3, 1, 4, 7, 8, 5],
        [4, 5, 3, 7, 8, 6, 9, 2, 1],
        [8, 7, 1, 9, 5, 2, 4, 3, 6],
        [6, 2, 7, 8, 3, 1, 5, 4, 9],
        [5, 9, 8, 4, 6, 7, 3, 1, 2],
        [1, 3, 4, 2, 9, 5, 6, 7, 8],
    ]);
    match sudoku_solution.verify_configuration()? {
        VerificationResult::Incomplete => panic!("solution config incomplete"),
        VerificationResult::Complete => {}
    }

    match sudoku.try_solve()? {
        VerificationResult::Incomplete => panic!("config incomplete"),
        VerificationResult::Complete => {}
    }

    for (cell_left, cell_right) in sudoku.iter().zip(sudoku_solution.iter()) {
        if cell_left.state().cell_number() != cell_right.state().cell_number() {
            panic!("error cell not equal");
        }
    }

    Ok(())
}

#[test]
fn sudoku_solving_back_trace() -> Result<(), VerificationError> {
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

    let sudoku_solution = Sudoku::new([
        [3, 1, 5, 6, 4, 8, 2, 9, 7],
        [7, 4, 6, 1, 2, 9, 8, 5, 3],
        [2, 8, 9, 5, 7, 3, 1, 6, 4],
        [9, 6, 2, 3, 1, 4, 7, 8, 5],
        [4, 5, 3, 7, 8, 6, 9, 2, 1],
        [8, 7, 1, 9, 5, 2, 4, 3, 6],
        [6, 2, 7, 8, 3, 1, 5, 4, 9],
        [5, 9, 8, 4, 6, 7, 3, 1, 2],
        [1, 3, 4, 2, 9, 5, 6, 7, 8],
    ]);
    match sudoku_solution.verify_configuration()? {
        VerificationResult::Incomplete => panic!("solution config incomplete"),
        VerificationResult::Complete => {}
    }

    sudoku.solve_back_trace()?;

    for (cell_left, cell_right) in sudoku.iter().zip(sudoku_solution.iter()) {
        if cell_left.state().cell_number() != cell_right.state().cell_number() {
            panic!("error cell not equal");
        }
    }

    Ok(())
}

#[test]
fn sudoku_solving_back_trace_error() {
    let mut sudoku = Sudoku::new([
        [3, 0, 0, 6, 0, 0, 0, 9, 0],
        [0, 4, 0, 0, 2, 0, 0, 5, 0],
        [0, 8, 0, 0, 7, 0, 1, 6, 0],
        [9, 0, 0, 3, 0, 4, 7, 0, 0],
        [3, 5, 0, 0, 8, 0, 0, 2, 0],
        [0, 0, 1, 9, 0, 0, 0, 0, 6],
        [0, 2, 7, 0, 3, 0, 0, 4, 0],
        [0, 9, 0, 0, 6, 0, 0, 1, 0],
        [0, 3, 0, 0, 0, 5, 0, 0, 8],
    ]);

    let verify_error = |result| match result {
        Err(VerificationError::HintInconsistency(c1, c2)) => {
            let p1 = CellPosition::new_from_number(0, 0).unwrap();
            let p2 = CellPosition::new_from_number(4, 0).unwrap();
            assert!(
                (c1 == p1 && c2 == p2) || (c2 == p1 && c1 == p2),
                "wrong cell position"
            );
        }
        _ => panic!("expected `VerificationError::HintInconsistency`"),
    };

    verify_error(sudoku.verify_configuration().map(|_| ()));
    verify_error(sudoku.solve_back_trace());
}
