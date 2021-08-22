//! contain cell and cell states
// TODO doc

use std::fmt::{Binary, Display, Formatter, LowerHex, Octal, UpperHex};

use serde::{Deserialize, Serialize};

use crate::error::SetError;
use crate::GAME_SIZE;

mod possibility;
pub(crate) use possibility::*;
mod guess;
pub(crate) use guess::*;

/// Represent cell sate, including the storage of data while solving the configuration
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
    /// Get the cell number of this cell
    pub fn cell_number(&self) -> Option<CellNumber> {
        match self {
            Self::Given(number) | Self::SolvedDeduction(number) | Self::SolvedBackTrace(number) => {
                Some(*number)
            }
            Self::Guess(guess) => guess.cell_number(),
            Self::Empty(possibility) => possibility.as_ref()?.cell_number(),
        }
    }

    /// Create eithen a given or an empty configurazion
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

/// Reprensent a cell in a [`crate::grid::Sudoku`]
#[derive(
    Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash, Serialize, Deserialize, Default,
)]
pub struct Cell {
    state: CellState,
}

impl Cell {
    /// Create a new cell with a given [`CellState`]
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
    /// # Example
    /// ```
    /// use sudoku::cell::CellNumber;
    /// use sudoku::GAME_SIZE;
    ///
    /// // in bounds
    /// assert!(CellNumber::is_in_bound(1));
    /// assert!(CellNumber::is_in_bound(GAME_SIZE - 1));
    /// assert!(CellNumber::is_in_bound(GAME_SIZE));
    ///
    /// // not in bounds
    /// assert!(!CellNumber::is_in_bound(0));
    /// assert!(!CellNumber::is_in_bound(GAME_SIZE + 100));
    /// ```
    pub const fn is_in_bound(number: usize) -> bool {
        number <= GAME_SIZE && number > 0
    }

    /// Create a new cell number. the input should be <= [`GAME_SIZE`] otherwise return [`None`]
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
    /// use sudoku::error::ExampleError;
    /// # use std::error::Error;
    ///
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// let mut c = CellNumber::new(1).ok_or(ExampleError::NoneError)?;
    /// c.set_number(2)?; // set is OK()
    /// assert_eq!(c.number(), 2);
    ///
    /// assert!(c.set_number(10).is_err()); // set did not work
    /// assert_eq!(c.number(), 2); // the old valure remains
    ///
    /// c.set_number(4)?; // the set is OK
    /// assert_eq!(c.number(), 4);
    ///
    /// assert!(c.set_number(0).is_err()); // 0 is not a valide numer
    /// assert_eq!(c.number(), 4); // the old valure remains
    /// # Ok(())
    /// # }
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

impl Binary for CellNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:b}", self.number)
    }
}

impl UpperHex for CellNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:X}", self.number)
    }
}

impl LowerHex for CellNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:x}", self.number)
    }
}

impl Octal for CellNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:o}", self.number)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn t_display() {
        let n = 1;
        let cell = CellNumber::new(n).unwrap();
        assert_eq!(format!("{}", cell), format!("{}", n));
        assert_eq!(format!("{:b}", cell), format!("{:b}", n));
        assert_eq!(format!("{:X}", cell), format!("{:X}", n));
        assert_eq!(format!("{:x}", cell), format!("{:x}", n));
        assert_eq!(format!("{:o}", cell), format!("{:o}", n));
    }
}
