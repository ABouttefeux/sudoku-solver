use std::error::Error;
use std::fmt::{Display, Formatter};
use std::iter::FusedIterator;
use std::ops::{BitAnd, BitOr, BitXor, Index, IndexMut};

use array_macro::array;
use serde::{Deserialize, Serialize};

use crate::cell::{Cell, CellNumber, CellPossibilities, CellState};
use crate::GAME_SIZE;

mod iterator;
pub use iterator::*;
mod position;
pub use position::*;

// #[derive(
//     Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash, Serialize, Deserialize, Default,
// )]
// pub struct SudokuInitial {
//     data: [[CellStateInitial; GAME_SIZE]; GAME_SIZE],
// }

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash, Serialize, Deserialize)]
/// Represent the grid with the cell and the current game state
pub struct Sudoku {
    data: [[Cell; GAME_SIZE]; GAME_SIZE],
}

// #[derive(
//     Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash, Serialize, Deserialize,
// )]
// pub struct SudokuSolved {
//     data: [[Cell; GAME_SIZE]; GAME_SIZE],
// }

impl Sudoku {
    pub fn new(input: [[usize; GAME_SIZE]; GAME_SIZE]) -> Self {
        Self {
            data: array![x => array![y => Cell::new(CellState::new(CellNumber::new(input[x][y]))); GAME_SIZE];GAME_SIZE ],
        }
    }

    /// Get a reference to the cell at the given position
    pub const fn get_cell(&self, index: CellPosition) -> &Cell {
        &self.data[index.x_usize()][index.y_usize()]
    }

    /// Get a mut reference to the cell at the given position
    pub fn get_cell_mut(&mut self, index: CellPosition) -> &mut Cell {
        &mut self.data[index.x_usize()][index.y_usize()]
    }

    /// Try solve the system by using deducting method
    /// # Errors
    /// return an error if there is an inconsitency in the configuration
    pub fn try_solve(&mut self) -> Result<VerificationResult, VerificationError> {
        // TODO optmize
        loop {
            let mut modification = false;

            for iterators in Self::rows() {
                for pos in iterators {
                    if let CellState::Empty(_) = self[pos].state() {
                        let possibility = self.possibility_cell(pos)?;
                        if let Some(number) = possibility.cell_number() {
                            modification = true;
                            self[pos] = Cell::new(CellState::SolvedDeduction(number));
                        } else {
                            // add empty
                        }
                    }
                }
            }
            if !modification {
                break;
            }
        }
        self.verify_configuration()
    }

    /// Solve using the backtrace methode
    pub fn solve_back_trace(&mut self) -> &mut Self {
        for iterators in Self::rows() {
            for pos in iterators {
                let cell = self[pos];
            }
        }
        todo!()
    }

    /// Returns all rows.
    pub fn rows() -> [Row; GAME_SIZE] {
        // array![ i => Row::new(CellPosition::new_from_number(0, i).unwrap(), self); 9]
        [
            Row::new(CellPosition::new_from_number(0, 0).unwrap()),
            Row::new(CellPosition::new_from_number(0, 1).unwrap()),
            Row::new(CellPosition::new_from_number(0, 2).unwrap()),
            Row::new(CellPosition::new_from_number(0, 3).unwrap()),
            Row::new(CellPosition::new_from_number(0, 4).unwrap()),
            Row::new(CellPosition::new_from_number(0, 5).unwrap()),
            Row::new(CellPosition::new_from_number(0, 6).unwrap()),
            Row::new(CellPosition::new_from_number(0, 7).unwrap()),
            Row::new(CellPosition::new_from_number(0, 8).unwrap()),
        ]
    }

    /// returns all columns.
    pub fn columns() -> [Column; GAME_SIZE] {
        [
            Column::new(CellPosition::new_from_number(0, 0).unwrap()),
            Column::new(CellPosition::new_from_number(1, 0).unwrap()),
            Column::new(CellPosition::new_from_number(2, 0).unwrap()),
            Column::new(CellPosition::new_from_number(3, 0).unwrap()),
            Column::new(CellPosition::new_from_number(4, 0).unwrap()),
            Column::new(CellPosition::new_from_number(5, 0).unwrap()),
            Column::new(CellPosition::new_from_number(6, 0).unwrap()),
            Column::new(CellPosition::new_from_number(7, 0).unwrap()),
            Column::new(CellPosition::new_from_number(8, 0).unwrap()),
        ]
    }

    /// returns all squares
    pub fn squares() -> [Square; GAME_SIZE] {
        [
            Square::new(CellPosition::new_from_number(0, 0).unwrap()),
            Square::new(CellPosition::new_from_number(3, 0).unwrap()),
            Square::new(CellPosition::new_from_number(6, 0).unwrap()),
            Square::new(CellPosition::new_from_number(0, 3).unwrap()),
            Square::new(CellPosition::new_from_number(3, 3).unwrap()),
            Square::new(CellPosition::new_from_number(6, 3).unwrap()),
            Square::new(CellPosition::new_from_number(0, 6).unwrap()),
            Square::new(CellPosition::new_from_number(3, 6).unwrap()),
            Square::new(CellPosition::new_from_number(6, 6).unwrap()),
        ]
    }

    /// Returns the row colum and square at the given position.
    fn row_column_square(pos: CellPosition) -> (Row, Column, Square) {
        (Row::new(pos), Column::new(pos), Square::new(pos))
    }

    fn sorted_cell_by_number(
        &self,
        it: &mut impl Iterator<Item = CellPosition>,
    ) -> [Vec<(CellPosition, CellState)>; GAME_SIZE] {
        let mut array = array![Vec::new(); GAME_SIZE];
        for el in it {
            let state = self[el].state();
            if let Some(number) = state.cell_number() {
                array[number.number() - 1].push((el, state));
                // - 1 because the number is between 1 and 9
            }
        }
        array
    }

    /// Consume the iterator and give a result corrsponding the completeness or error
    /// of the sudoku game
    ///
    /// # Errors
    /// - [`VerificationError::HintInconsistency`] if two (or more) hints are in conflict
    /// - [`VerificationError::WrongSolution`] if a hint and a solution or two solutions are conflicting
    /// - [`VerificationError::WrongGuess`] if a guess is conflicting with a solution or an given number
    fn verify_iterator(
        &self,
        it: &mut impl Iterator<Item = CellPosition>,
    ) -> Result<VerificationResult, VerificationError> {
        let array = self.sorted_cell_by_number(it);

        let mut complete = true;
        for vec in array {
            if vec.len() >= 2 {
                return Err(Self::report_verification_error_conflict(vec));
            } else if vec.is_empty() {
                complete = false;
            }
        }
        if complete {
            Ok(VerificationResult::Complete)
        } else {
            Ok(VerificationResult::Incomplete)
        }
    }

    /// take an vector and report the error
    fn report_verification_error_conflict(
        vec: Vec<(CellPosition, CellState)>,
    ) -> VerificationError {
        let mut count_given = 0_usize;
        let mut count_guess = 0_usize;
        let mut count_solution = 0_usize;
        let mut vec_pos = Vec::with_capacity(vec.len());
        for (pos, cell) in vec {
            match cell {
                CellState::Given(_) => {
                    vec_pos.insert(count_given, pos);
                    count_given += 1;
                }
                CellState::Guess(_) => {
                    vec_pos.insert(count_given + count_guess, pos);
                    count_guess += 1;
                }
                CellState::SolvedBackTrace(_)
                | CellState::SolvedDeduction(_)
                | CellState::Empty(_) => {
                    vec_pos.push(pos);
                    count_solution += 1;
                }
            }
        }
        // returning block
        if count_given >= 2 {
            VerificationError::HintInconsistency(vec_pos[0], vec_pos[1])
        } else if count_solution + count_given >= 2 {
            VerificationError::WrongSolution(vec_pos[0], vec_pos[1])
        } else if count_solution + count_given + count_guess >= 2 {
            VerificationError::WrongGuess(vec_pos[0], vec_pos[1])
        } else {
            panic!("`Sudoku::report_verification_error_conflict` is used incorectly, the given vectore should be of length 2 ore more")
        }
    }

    /// Verify the validity of iterator in a list
    ///
    /// # Errors
    /// - [`VerificationError::HintInconsistency`] if two (or more) hints are in conflict
    /// - [`VerificationError::WrongSolution`] if a hint and a solution or two solutions are conflicting
    /// - [`VerificationError::WrongGuess`] if a guess is conflicting with a solution or an given number
    fn verify_list_iter(
        self,
        list: &mut [impl Iterator<Item = CellPosition>],
    ) -> Result<VerificationResult, VerificationError> {
        let mut verification_result = VerificationResult::Complete;
        for iter in list {
            let res = self.verify_iterator(iter)?;
            if let VerificationResult::Incomplete = res {
                verification_result = VerificationResult::Incomplete;
            }
        }
        Ok(verification_result)
    }

    /// Verifiy if the configuration is correcte.
    ///
    /// # Errors
    /// - [`VerificationError::HintInconsistency`] if two (or more) hints are in conflict
    /// - [`VerificationError::WrongSolution`] if a hint and a solution or two solutions are conflicting
    /// - [`VerificationError::WrongGuess`] if a guess is conflicting with a solution or an given number
    pub fn verify_configuration(&self) -> Result<VerificationResult, VerificationError> {
        Ok(self.verify_list_iter(&mut Self::rows())?
            & self.verify_list_iter(&mut Self::columns())?
            & self.verify_list_iter(&mut Self::squares())?)
    }

    fn possibility_iter(
        &self,
        it: &mut impl Iterator<Item = CellPosition>,
    ) -> Result<CellPossibilities, VerificationError> {
        let array = self.sorted_cell_by_number(it);

        let mut possibilities = CellPossibilities::new_no_possibility();
        for (index, vec) in IntoIterator::into_iter(array).enumerate() {
            if vec.len() >= 2 {
                return Err(Self::report_verification_error_conflict(vec));
            } else if vec.is_empty() {
                possibilities[CellNumber::new(index + 1).unwrap()] = true;
            }
        }
        Ok(possibilities)
    }

    ///
    /// note : Exclude pos from the numbers so it can be use to recompute the possibility on a guessed cell
    fn possibility_cell(&self, pos: CellPosition) -> Result<CellPossibilities, VerificationError> {
        let (row, col, square) = Self::row_column_square(pos);
        let filter_cell = |cell_pos: &CellPosition| *cell_pos != pos;
        Ok(self.possibility_iter(&mut row.filter(filter_cell))?
            & self.possibility_iter(&mut col.filter(filter_cell))?
            & self.possibility_iter(&mut square.filter(filter_cell))?)
    }

    /// Create a iterators on all cells
    pub fn iter(&self) -> impl Iterator<Item = &Cell> + FusedIterator + DoubleEndedIterator {
        self.data.iter().flatten()
    }

    /// Create a iterators on all cells with a mut reference
    pub fn iter_mut(
        &mut self,
    ) -> impl Iterator<Item = &mut Cell> + FusedIterator + DoubleEndedIterator {
        self.data.iter_mut().flatten()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
/// Error of a Sudoku configuration, the first cell position is the place of the stronger cell
pub enum VerificationError {
    /// Inconsitency in the hints
    HintInconsistency(CellPosition, CellPosition),
    /// A solution is wrong
    WrongSolution(CellPosition, CellPosition),
    /// A Guess is wrong
    WrongGuess(CellPosition, CellPosition),
}

impl Display for VerificationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WrongGuess(pos1, pos2) => write!(f, "wrong guess between {} and {}", pos2, pos1,),
            Self::WrongSolution(pos1, pos2) => {
                write!(f, "wrong solution between {} and {}", pos2, pos1)
            }
            Self::HintInconsistency(pos1, pos2) => {
                write!(f, "hints are inconsistante between {} and {}", pos1, pos2)
            }
        }
    }
}

impl Error for VerificationError {}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash, Serialize, Deserialize)]
#[allow(clippy::exhaustive_enums)]
/// Contain a verification sucess but either for a complete configuration (or sub configuration)
/// or incomplete.
pub enum VerificationResult {
    /// Incomplete configuration
    Incomplete,
    /// Complete configuration
    Complete,
}

impl BitAnd for VerificationResult {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Complete, Self::Complete) => Self::Complete,
            _ => Self::Incomplete,
        }
    }
}

impl BitOr for VerificationResult {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Incomplete, Self::Incomplete) => Self::Incomplete,
            _ => Self::Complete,
        }
    }
}

impl BitXor for VerificationResult {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Incomplete, Self::Complete) | (Self::Complete, Self::Incomplete) => {
                Self::Complete
            }
            _ => Self::Incomplete,
        }
    }
}

//TODO range
impl Index<CellPosition> for Sudoku {
    type Output = Cell;

    fn index(&self, index: CellPosition) -> &Self::Output {
        self.get_cell(index)
    }
}

impl IndexMut<CellPosition> for Sudoku {
    fn index_mut(&mut self, index: CellPosition) -> &mut Self::Output {
        self.get_cell_mut(index)
    }
}

const SPACING_CELL: usize = 1;

#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_sign_loss)]
fn size_number(number: usize) -> usize {
    ((number as f64).log10().floor() as usize) + 1
}

fn write_line_separation(f: &mut Formatter<'_>) -> std::fmt::Result {
    for _ in 0..GAME_SIZE {
        write!(f, "+")?;
        write!(
            f,
            "{}",
            "-".repeat(size_number(GAME_SIZE) + SPACING_CELL * 2)
        )?;
    }
    writeln!(f, "+")
}

impl Display for Sudoku {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in self.data {
            write_line_separation(f)?;
            for cell in row {
                write!(f, "|")?;
                if let Some(number) = cell.state().cell_number() {
                    write!(
                        f,
                        "{}",
                        " ".repeat(
                            SPACING_CELL + size_number(GAME_SIZE) - size_number(number.number())
                        )
                    )?;
                    #[allow(clippy::repeat_once)] // because it is a constant that could be not 1.
                    write!(f, "{}{}", number, " ".repeat(SPACING_CELL))?;
                } else {
                    write!(
                        f,
                        "{}",
                        " ".repeat(size_number(GAME_SIZE) + SPACING_CELL * 2)
                    )?;
                }
            }
            writeln!(f, "|")?;
        }
        write_line_separation(f)
    }
}
