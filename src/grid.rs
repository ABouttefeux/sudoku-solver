//! Contain the main utility for [`Sudoku`]
// TODO

use std::error::Error;
use std::fmt::{Display, Formatter};
use std::iter::FusedIterator;
use std::ops::{BitAnd, BitOr, BitXor, Index, IndexMut};

use array_macro::array;
use console::Style;
use rand::distributions::Uniform;
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::cell::{Cell, CellGuess, CellNumber, CellPossibilities, CellState};

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

#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq, Hash, Serialize, Deserialize)]
/// Represent the grid with the cell and the current game state
pub struct Sudoku<const SQUARE_SIZE: usize>
where
    [[Cell<SQUARE_SIZE>; SQUARE_SIZE * SQUARE_SIZE]; SQUARE_SIZE * SQUARE_SIZE]: Sized,
{
    #[serde(bound(
        serialize = "[[Cell<SQUARE_SIZE>; SQUARE_SIZE * SQUARE_SIZE]; SQUARE_SIZE * SQUARE_SIZE]: Serialize",
        deserialize = "[[Cell<SQUARE_SIZE>; SQUARE_SIZE * SQUARE_SIZE]; SQUARE_SIZE * SQUARE_SIZE]: Deserialize<'de>"
    ))]
    data: [[Cell<SQUARE_SIZE>; SQUARE_SIZE * SQUARE_SIZE]; SQUARE_SIZE * SQUARE_SIZE],
}

// #[derive(
//     Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash, Serialize, Deserialize,
// )]
// pub struct SudokuSolved {
//     data: [[Cell; GAME_SIZE]; GAME_SIZE],
// }

impl<const SQUARE_SIZE: usize> Sudoku<SQUARE_SIZE>
where
    [[Cell<SQUARE_SIZE>; SQUARE_SIZE * SQUARE_SIZE]; SQUARE_SIZE * SQUARE_SIZE]: Sized,
{
    /// Create a configuration with the given array, number 0 ore >= 10 are replaces by empty cells.
    pub fn new(input: [[usize; SQUARE_SIZE * SQUARE_SIZE]; SQUARE_SIZE * SQUARE_SIZE]) -> Self {
        Self {
            data: array![
                x => array![
                    y => Cell::new(CellState::new(CellNumber::new(input[x][y])));
                    SQUARE_SIZE * SQUARE_SIZE
                ];
                SQUARE_SIZE * SQUARE_SIZE
            ],
        }
    }

    /// Create an empty config with `number_of_hints` randoms hints
    // TODO give valide config
    pub fn random(number_of_hints: usize, rng: &mut impl Rng) -> Option<Self> {
        if number_of_hints > SQUARE_SIZE.pow(4) {
            None
        } else {
            let distribution = Uniform::new(0, SQUARE_SIZE.pow(2));
            let data = array![array![Cell::new(CellState::Empty(None)); SQUARE_SIZE * SQUARE_SIZE];
                SQUARE_SIZE * SQUARE_SIZE];
            let mut sudoku = Self { data };
            let mut number_of_hints_placed = 0;
            while number_of_hints_placed < number_of_hints {
                let position = CellPosition::new_from_number(
                    rng.sample(distribution),
                    rng.sample(distribution),
                )
                .unwrap();
                if let CellState::Empty(_) = sudoku[position].state() {
                    // TODO check the unwrap
                    let possibilities = sudoku.possibility_cell(position).unwrap();
                    if possibilities.number_of_possibility() > 0 {
                        let vec = possibilities.into_vec();
                        let d2 = Uniform::new(0, vec.len());
                        *sudoku[position].state_mut() = CellState::Given(vec[rng.sample(d2)]);
                        number_of_hints_placed += 1;
                    }
                }
            }
            Some(sudoku)
        }
    }

    /// Get a reference to the cell at the given position
    pub const fn get_cell(&self, index: CellPosition<SQUARE_SIZE>) -> &Cell<SQUARE_SIZE> {
        &self.data[index.x_usize()][index.y_usize()]
    }

    /// Get a mut reference to the cell at the given position
    pub fn get_cell_mut(&mut self, index: CellPosition<SQUARE_SIZE>) -> &mut Cell<SQUARE_SIZE> {
        &mut self.data[index.x_usize()][index.y_usize()]
    }

    /// Try solve the system by using deducting method
    /// # Errors
    /// return an error if there is an inconsitency in the configuration
    pub fn try_solve(&mut self) -> Result<VerificationResult, VerificationError<SQUARE_SIZE>> {
        // TODO optmize
        loop {
            // println!("{}", self);
            // console::Term::stderr()
            //     .move_cursor_up(GAME_SIZE * 2 + 2)
            //     .unwrap();
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
    /// # Errors
    /// return an error if there is an inconsitency in the configuration
    pub fn solve_back_trace(&mut self) -> Result<(), SolveError<SQUARE_SIZE>> {
        let mut direction = Direction::Forward;
        let mut pos_tracker = BackTracePositionTracker::new();
        loop {
            //TODO better prints

            println!("{}", self);
            console::Term::stdout()
                .move_cursor_up(SQUARE_SIZE.pow(2) * 2 + 2)
                .unwrap();
            let pos = pos_tracker.move_pos(direction);
            match pos {
                Some(pos) => match self[pos].state_mut() {
                    CellState::Given(_number)
                    | CellState::SolvedDeduction(_number)
                    | CellState::SolvedBackTrace(_number) => {}
                    CellState::Empty(_possibilities) => {
                        // let p = match possibilities {
                        //     Some(possibilities) => possibilities,
                        //     None => self.possibility_cell(pos)?,
                        // };
                        let p = self.possibility_cell(pos)?;
                        match CellGuess::new(p) {
                            Some(guess) => {
                                direction = Direction::Forward;
                                self[pos] = Cell::new(CellState::Guess(guess));
                            }
                            None => direction = Direction::Backward,
                        }
                    }
                    CellState::Guess(ref mut guess) => match guess.next_guess() {
                        Some(_) => direction = Direction::Forward,
                        None => {
                            direction = Direction::Backward;
                            self[pos] = Cell::new(CellState::Empty(None));
                        }
                    },
                },
                None => match direction {
                    Direction::Forward => break Ok(()),
                    Direction::Backward => break Err(SolveError::ImpossibleConfiguration),
                },
            }
        }
    }

    /// Returns all rows.
    pub fn rows() -> [Row<SQUARE_SIZE>; SQUARE_SIZE * SQUARE_SIZE] {
        // array![ i => Row::new(CellPosition::new_from_number(0, i).unwrap(), self); 9]
        array![
            x => Row::new(CellPosition::new_from_number(0, x).unwrap());
            SQUARE_SIZE * SQUARE_SIZE
        ]
    }

    /// returns all columns.
    pub fn columns() -> [Column<SQUARE_SIZE>; SQUARE_SIZE * SQUARE_SIZE] {
        array![
            x => Column::new(CellPosition::new_from_number(x, 0).unwrap());
            SQUARE_SIZE * SQUARE_SIZE
        ]
    }

    /// returns all squares
    pub fn squares() -> [Square<SQUARE_SIZE>; SQUARE_SIZE * SQUARE_SIZE] {
        array![
            i => Square::new(
                CellPosition::new_from_number(
                    (i % SQUARE_SIZE)* SQUARE_SIZE,
                    (i / SQUARE_SIZE) * SQUARE_SIZE)
                .unwrap()
            );
            SQUARE_SIZE * SQUARE_SIZE]
    }

    /// Returns the row colum and square at the given position.
    fn row_column_square(
        pos: CellPosition<SQUARE_SIZE>,
    ) -> (Row<SQUARE_SIZE>, Column<SQUARE_SIZE>, Square<SQUARE_SIZE>) {
        (Row::new(pos), Column::new(pos), Square::new(pos))
    }

    /// Take an iteractors and collects it and return an Vec sorted by cell number
    fn sorted_cell_by_number(
        &self,
        it: impl Iterator<Item = CellPosition<SQUARE_SIZE>>,
    ) -> [Vec<(CellPosition<SQUARE_SIZE>, CellState<SQUARE_SIZE>)>; SQUARE_SIZE * SQUARE_SIZE] {
        let mut array = array![Vec::new(); SQUARE_SIZE * SQUARE_SIZE];
        for el in it {
            let state = self[el].state();
            if let Some(number) = state.cell_number() {
                array[number.number() - 1].push((el, state.clone()));
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
        it: &mut impl Iterator<Item = CellPosition<SQUARE_SIZE>>,
    ) -> Result<VerificationResult, VerificationError<SQUARE_SIZE>> {
        let array = self.sorted_cell_by_number(it);

        let mut complete = VerificationResult::Complete;
        for vec in array {
            if vec.len() >= 2 {
                return Err(Self::report_verification_error_conflict(vec));
            } else if vec.is_empty() {
                complete = VerificationResult::Incomplete;
            }
        }
        Ok(complete)
    }

    /// take an vector and report the error
    /// # Panics
    /// panics if the vector does not have at least two elements
    fn report_verification_error_conflict(
        vec: Vec<(CellPosition<SQUARE_SIZE>, CellState<SQUARE_SIZE>)>,
    ) -> VerificationError<SQUARE_SIZE> {
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
    fn verify_list_iter<'a, I>(
        &self,
        list: impl IntoIterator<Item = &'a mut I>,
    ) -> Result<VerificationResult, VerificationError<SQUARE_SIZE>>
    where
        I: Iterator<Item = CellPosition<SQUARE_SIZE>> + 'a,
    {
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
    pub fn verify_configuration(
        &self,
    ) -> Result<VerificationResult, VerificationError<SQUARE_SIZE>> {
        Ok(self.verify_list_iter(&mut Self::rows())?
            & self.verify_list_iter(&mut Self::columns())?
            & self.verify_list_iter(&mut Self::squares())?)
    }

    fn possibility_iter(
        &self,
        it: impl Iterator<Item = CellPosition<SQUARE_SIZE>>,
    ) -> Result<CellPossibilities<SQUARE_SIZE>, VerificationError<SQUARE_SIZE>> {
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

    /// Return the possibility of numbers a cell can have
    /// note : Exclude pos from the numbers so it can be use to recompute the possibility on a guessed cell
    fn possibility_cell(
        &self,
        pos: CellPosition<SQUARE_SIZE>,
    ) -> Result<CellPossibilities<SQUARE_SIZE>, VerificationError<SQUARE_SIZE>> {
        let (row, col, square) = Self::row_column_square(pos);
        let filter_cell = |cell_pos: &CellPosition<SQUARE_SIZE>| *cell_pos != pos;
        Ok(self.possibility_iter(row.filter(filter_cell))?
            & self.possibility_iter(col.filter(filter_cell))?
            & self.possibility_iter(square.filter(filter_cell))?)
    }

    /// Create a iterators on all cells
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = &Cell<SQUARE_SIZE>> + FusedIterator + DoubleEndedIterator {
        self.data.iter().flatten()
    }

    /// Create a iterators on all cells with a mut reference
    pub fn iter_mut(
        &mut self,
    ) -> impl Iterator<Item = &mut Cell<SQUARE_SIZE>> + FusedIterator + DoubleEndedIterator {
        self.data.iter_mut().flatten()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
/// Error of a Sudoku configuration, the first cell position is the place of the stronger cell
pub enum VerificationError<const SQUARE_SIZE: usize> {
    /// Inconsitency in the hints
    HintInconsistency(CellPosition<SQUARE_SIZE>, CellPosition<SQUARE_SIZE>),
    /// A solution is wrong
    WrongSolution(CellPosition<SQUARE_SIZE>, CellPosition<SQUARE_SIZE>),
    /// A Guess is wrong
    WrongGuess(CellPosition<SQUARE_SIZE>, CellPosition<SQUARE_SIZE>),
}

impl<const SQUARE_SIZE: usize> Display for VerificationError<SQUARE_SIZE> {
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

impl<const SQUARE_SIZE: usize> Error for VerificationError<SQUARE_SIZE> {}

/// Error returned by [`Sudoku::solve_back_trace`]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum SolveError<const SQUARE_SIZE: usize> {
    /// Verification error, the grid is inconsistant
    VerificationError(VerificationError<SQUARE_SIZE>),
    /// Ther is no solution for this configuration
    ImpossibleConfiguration,
}

impl<const SQUARE_SIZE: usize> Display for SolveError<SQUARE_SIZE> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::VerificationError(error) => write!(f, "{}", error),
            Self::ImpossibleConfiguration => {
                write!(f, "the given configuration has no solution")
            }
        }
    }
}

impl<const SQUARE_SIZE: usize> Error for SolveError<SQUARE_SIZE> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::VerificationError(error) => Some(error),
            Self::ImpossibleConfiguration => None,
        }
    }
}

impl<const SQUARE_SIZE: usize> From<VerificationError<SQUARE_SIZE>> for SolveError<SQUARE_SIZE> {
    fn from(error: VerificationError<SQUARE_SIZE>) -> Self {
        Self::VerificationError(error)
    }
}

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
impl<const SQUARE_SIZE: usize> Index<CellPosition<SQUARE_SIZE>> for Sudoku<SQUARE_SIZE>
where
    [[Cell<SQUARE_SIZE>; SQUARE_SIZE * SQUARE_SIZE]; SQUARE_SIZE * SQUARE_SIZE]: Sized,
{
    type Output = Cell<SQUARE_SIZE>;

    fn index(&self, index: CellPosition<SQUARE_SIZE>) -> &Self::Output {
        self.get_cell(index)
    }
}

impl<const SQUARE_SIZE: usize> IndexMut<CellPosition<SQUARE_SIZE>> for Sudoku<SQUARE_SIZE>
where
    [[Cell<SQUARE_SIZE>; SQUARE_SIZE * SQUARE_SIZE]; SQUARE_SIZE * SQUARE_SIZE]: Sized,
{
    fn index_mut(&mut self, index: CellPosition<SQUARE_SIZE>) -> &mut Self::Output {
        self.get_cell_mut(index)
    }
}

//====================================================//

const SPACING_CELL: usize = 1;

#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_sign_loss)]
fn size_number(number: usize) -> usize {
    ((number as f64).log10().floor() as usize) + 1
}

fn write_line_separation_basic<const SQUARE_SIZE: usize>(
    f: &mut Formatter<'_>,
) -> std::fmt::Result {
    let line_length = size_number(SQUARE_SIZE.pow(2)) + SPACING_CELL * 2;
    for _ in 0..SQUARE_SIZE.pow(2) {
        write!(f, "+")?;
        write!(f, "{}", "-".repeat(line_length))?;
    }
    writeln!(f, "+")
}

// ╣ ║ ┐ └ ┴ ┬ ├ ─ ┼ ╩ ╦ ╠ ═ ╬ ╟ ╞ ╡ ╢ ╤ ╧ ╫ ╪
#[allow(clippy::non_ascii_literal)]
fn write_line_separation<const SQUARE_SIZE: usize>(
    f: &mut Formatter<'_>,
    number: usize,
) -> std::fmt::Result {
    let (char_left, double_line, char_right, single_cross, double_cross) = if number == 0 {
        ('╔', true, '╗', '╤', '╦')
    } else if number == SQUARE_SIZE * SQUARE_SIZE {
        ('╚', true, '╝', '╧', '╩')
    } else if number % SQUARE_SIZE == 0 {
        ('╠', true, '╣', '╪', '╬')
    } else {
        ('╟', false, '╢', '┼', '╫')
    };
    let line_str = if double_line { "═" } else { "─" };
    let line_length = size_number(SQUARE_SIZE * SQUARE_SIZE) + SPACING_CELL * 2;
    write!(f, "{}", char_left)?;
    for index in 1..(SQUARE_SIZE * SQUARE_SIZE) {
        write!(f, "{}", line_str.repeat(line_length))?;
        let cross_char = if index % SQUARE_SIZE == 0 {
            double_cross
        } else {
            single_cross
        };
        write!(f, "{}", cross_char)?;
    }
    write!(f, "{}", line_str.repeat(line_length))?;
    writeln!(f, "{}", char_right)
}

fn style_cell<const SQUARE_SIZE: usize>(cell: &Cell<SQUARE_SIZE>) -> Style
where
    [bool; SQUARE_SIZE * SQUARE_SIZE]: Sized,
{
    match cell.state() {
        CellState::Given(_) => Style::new().white(),
        CellState::SolvedDeduction(_) => Style::new().green(),
        CellState::SolvedBackTrace(_) => Style::new().green(),
        CellState::Empty(_) => Style::new().white(),
        CellState::Guess(_) => Style::new().red().bright(),
    }
}

#[allow(clippy::repeat_once)] // because it is a constant that could be not 1.
fn display_cell_interior<const SQUARE_SIZE: usize>(
    f: &mut Formatter<'_>,
    cell: &Cell<SQUARE_SIZE>,
) -> std::fmt::Result
where
    [bool; SQUARE_SIZE * SQUARE_SIZE]: Sized,
{
    if let Some(number) = cell.state().cell_number() {
        write!(
            f,
            "{}",
            " ".repeat(
                SPACING_CELL + size_number(SQUARE_SIZE * SQUARE_SIZE)
                    - size_number(number.number())
            )
        )?;
        let style = style_cell(cell);
        write!(f, "{}{}", style.apply_to(number), " ".repeat(SPACING_CELL))
    } else {
        write!(
            f,
            "{}",
            " ".repeat(size_number(SQUARE_SIZE * SQUARE_SIZE) + SPACING_CELL * 2)
        )
    }
}

impl<const SQUARE_SIZE: usize> Display for Sudoku<SQUARE_SIZE>
where
    [[Cell<SQUARE_SIZE>; SQUARE_SIZE * SQUARE_SIZE]; SQUARE_SIZE * SQUARE_SIZE]: Sized,
{
    #[allow(clippy::non_ascii_literal)]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if f.sign_minus() {
            for row in &self.data {
                write_line_separation_basic::<SQUARE_SIZE>(f)?;
                for cell in row {
                    write!(f, "|")?;
                    display_cell_interior::<SQUARE_SIZE>(f, &cell)?;
                }
                writeln!(f, "|")?;
            }
            write_line_separation_basic::<SQUARE_SIZE>(f)
        } else {
            for (l_index, row) in self.data.iter().enumerate() {
                write_line_separation::<SQUARE_SIZE>(f, l_index)?;
                for (index, cell) in row.iter().enumerate() {
                    let v_line = if index % SQUARE_SIZE == 0 {
                        '║'
                    } else {
                        '│'
                    };
                    write!(f, "{}", v_line)?;
                    display_cell_interior::<SQUARE_SIZE>(f, cell)?;
                }
                writeln!(f, "║")?;
            }
            write_line_separation::<SQUARE_SIZE>(f, SQUARE_SIZE * SQUARE_SIZE)
        }
    }
}
