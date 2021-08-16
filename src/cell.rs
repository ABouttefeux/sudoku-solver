//! contain cell and cell states
// TODO doc

use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

use crate::error::SetError;
use crate::GAME_SIZE;

mod possibility;
pub(crate) use possibility::*;
mod guess;
pub(crate) use guess::*;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash, Serialize, Deserialize)]
#[allow(clippy::exhaustive_enums)]
// TODO remove pub crate
pub(crate) enum CellState {
    /// Cell number that is given
    Given(CellNumber),
    /// Cell number that has been solved, kowning exactly the value
    SolvedDeduction(CellNumber),
    /// Cell number that have been solve with backtrace
    SolvedBackTrace(CellNumber),
    /// Unsolved cell containing possibilities
    Empty(Option<CellPossibilities>),
    /// Value tried
    Guess(CellGuess),
}

impl CellState {
    pub fn cell_number(&self) -> Option<CellNumber> {
        match self {
            Self::Given(number) | Self::SolvedDeduction(number) | Self::SolvedBackTrace(number) => {
                Some(*number)
            }
            Self::Guess(guess) => guess.cell_number(),
            Self::Empty(possibility) => possibility.as_ref()?.cell_number(),
        }
    }

    pub const fn new(nb: Option<CellNumber>) -> Self {
        match nb {
            Some(nb) => Self::Given(nb),
            None => Self::Empty(None),
        }
    }
}

impl Default for CellState {
    fn default() -> Self {
        Self::Empty(None)
    }
}

// #[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash, Serialize, Deserialize)]
// #[allow(clippy::exhaustive_enums)]
// // TODO
// pub enum CellStateInitial {
//     Number(CellNumber),
//     Empty,
// }

// impl Default for CellStateInitial {
//     fn default() -> Self {
//         Self::Empty
//     }
// }

// #[derive(
//     Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash, Serialize, Deserialize, Default,
// )]
// pub struct CellSolved {
//     number: CellNumber,
// }

// impl CellSolved {
//     pub const fn new(cell: Cell) -> Option<Self> {
//         match cell.state {
//             CellState::Empty(_) => None,
//             CellState::Number(number) => Some(Self { number }),
//         }
//     }

//     pub const fn new_from_number(number: CellNumber) -> Self {
//         Self{number}
//     }
// }

#[derive(
    Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash, Serialize, Deserialize, Default,
)]
/// Reprensent a cell in a [`Sudoku`]
pub struct Cell {
    state: CellState,
}

impl Cell {
    pub(crate) const fn new(state: CellState) -> Self {
        Self { state }
    }
    //TODO remove pub
    /// Getter on the state
    pub(crate) const fn state(&self) -> CellState {
        self.state
    }

    //TODO remove pub
    /// mut ref to the sate
    pub(crate) fn state_mut(&mut self) -> &mut CellState {
        &mut self.state
    }
}

/// Represent a number that a cell can hold. Can only hold 1 thought 9
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash, Serialize, Deserialize)]
pub struct CellNumber {
    number: usize,
}
// TODO u8 ?

impl CellNumber {
    /// Test if the given value is in bounds
    pub const fn is_in_bound(number: usize) -> bool {
        number <= GAME_SIZE && number > 0
    }

    /// Create a new cell number. the input should be < 10 otherwise return [`None`]
    /// # Example
    /// ```
    /// use sudoku::cell::CellNumber;
    ///
    /// assert!(CellNumber::new(0).is_none());
    /// assert!(CellNumber::new(1).is_some());
    /// assert!(CellNumber::new(8).is_some());
    /// assert!(CellNumber::new(9).is_some());
    /// assert!(CellNumber::new(10).is_none());
    /// ```
    pub const fn new(number: usize) -> Option<Self> {
        if Self::is_in_bound(number) {
            Some(Self { number })
        } else {
            None
        }
    }

    /// Get the nnumber contained.
    /// # Example
    /// ```
    /// use sudoku::cell::CellNumber;
    /// use sudoku::error::ExampleError;
    /// # use std::error::Error;
    ///
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// let cell = CellNumber::new(2).ok_or(ExampleError::NoneError)?;
    /// assert_eq!(cell.number(), 2);
    /// # Ok(())
    /// # }
    /// ```
    pub const fn number(&self) -> usize {
        self.number
    }

    /// Try set the number inside this, returns an error is the value is out of bounds
    /// # Errors
    /// returns [`SetError::ValueOutOfBounds`]if the value is bigger or equal than nine.
    /// # Example
    /// ```
    /// use sudoku::cell::CellNumber;
    ///
    /// let mut c = CellNumber::new(1).unwrap();
    /// c.set_number(2).unwrap();
    /// assert_eq!(c.number(), 2);
    /// assert!(c.set_number(10).is_err());
    /// assert_eq!(c.number(), 2);
    /// c.set_number(4).unwrap();
    /// assert_eq!(c.number(), 4);
    /// assert!(c.set_number(0).is_err());
    /// assert_eq!(c.number(), 4);
    /// ```
    pub fn set_number(&mut self, number: usize) -> Result<(), SetError> {
        if Self::is_in_bound(number) {
            self.number = number;
            Ok(())
        } else {
            Err(SetError::ValueOutOfBounds)
        }
    }
}

impl Default for CellNumber {
    /// Create a [`CellNumber`] with value 1
    /// # Example
    /// ```
    /// use sudoku::cell::CellNumber;
    ///
    /// assert_eq!(CellNumber::default().number(), 1);
    /// ```
    fn default() -> Self {
        Self::new(1).expect("unreacharble")
    }
}

impl Display for CellNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.number)
    }
}
