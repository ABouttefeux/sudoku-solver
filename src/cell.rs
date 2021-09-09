//! contain cell and cell states
// TODO doc

use std::fmt::{Binary, Display, Formatter, LowerHex, Octal, UpperHex};

use serde::{Deserialize, Serialize};

use crate::error::SetError;

mod possibility;
pub(crate) use possibility::*;
mod guess;
pub(crate) use guess::*;

/// Represent cell sate, including the storage of data while solving the configuration
#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq, Hash, Serialize, Deserialize)]
#[allow(clippy::exhaustive_enums)]
// TODO remove pub crate
pub(crate) enum CellState<const SQUARE_SIZE: usize>
where
    [bool; SQUARE_SIZE * SQUARE_SIZE]: Sized,
{
    /// Cell number that is given
    Given(CellNumber<SQUARE_SIZE>),
    /// Cell number that has been solved, kowning exactly the value
    SolvedDeduction(CellNumber<SQUARE_SIZE>),
    /// Cell number that have been solve with backtrace
    SolvedBackTrace(CellNumber<SQUARE_SIZE>),
    /// Unsolved cell containing possibilities
    #[serde(bound(
        serialize = "CellPossibilities<SQUARE_SIZE>: Serialize",
        deserialize = "CellPossibilities<SQUARE_SIZE>: Deserialize<'de>"
    ))]
    Empty(Option<CellPossibilities<SQUARE_SIZE>>),
    /// Value tried
    #[serde(bound(
        serialize = "CellGuess<SQUARE_SIZE>: Serialize",
        deserialize = "CellGuess<SQUARE_SIZE>: Deserialize<'de>"
    ))]
    Guess(CellGuess<SQUARE_SIZE>),
}

impl<const SQUARE_SIZE: usize> CellState<SQUARE_SIZE>
where
    [bool; SQUARE_SIZE * SQUARE_SIZE]: Sized,
{
    /// Get the cell number of this cell
    pub fn cell_number(&self) -> Option<CellNumber<SQUARE_SIZE>> {
        match self {
            Self::Given(number) | Self::SolvedDeduction(number) | Self::SolvedBackTrace(number) => {
                Some(*number)
            }
            Self::Guess(guess) => guess.cell_number(),
            Self::Empty(possibility) => possibility.as_ref()?.cell_number(),
        }
    }

    /// Create eithen a given or an empty configurazion
    pub const fn new(nb: Option<CellNumber<SQUARE_SIZE>>) -> Self {
        match nb {
            Some(nb) => Self::Given(nb),
            None => Self::Empty(None),
        }
    }
}

impl<const SQUARE_SIZE: usize> Default for CellState<SQUARE_SIZE>
where
    [bool; SQUARE_SIZE * SQUARE_SIZE]: Sized,
{
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
#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq, Hash, Default, Serialize, Deserialize)]

pub struct Cell<const SQUARE_SIZE: usize>
where
    [bool; SQUARE_SIZE * SQUARE_SIZE]: Sized,
{
    #[serde(bound(
        serialize = "[bool; SQUARE_SIZE * SQUARE_SIZE]: Serialize",
        deserialize = "[bool; SQUARE_SIZE * SQUARE_SIZE]: Deserialize<'de>"
    ))]
    state: CellState<SQUARE_SIZE>,
}

impl<const SQUARE_SIZE: usize> Cell<SQUARE_SIZE>
where
    [bool; SQUARE_SIZE * SQUARE_SIZE]: Sized,
{
    /// Create a new cell with a given [`CellState`]
    pub(crate) const fn new(state: CellState<SQUARE_SIZE>) -> Self {
        Self { state }
    }

    //TODO remove pub
    /// Getter on the state
    pub(crate) const fn state(&self) -> &CellState<SQUARE_SIZE> {
        &self.state
    }

    //TODO remove pub
    /// mut ref to the sate
    pub(crate) fn state_mut(&mut self) -> &mut CellState<SQUARE_SIZE> {
        &mut self.state
    }

    /// Create eithen a given or an empty configurazion
    pub const fn new_opt(nb: Option<CellNumber<SQUARE_SIZE>>) -> Self {
        Self {
            state: CellState::new(nb),
        }
    }

    /// Create a new empty cell
    pub const fn new_empty() -> Self {
        Self {
            state: CellState::Empty(None),
        }
    }

    /// Create a cell with a given hint
    pub const fn new_hint(hint: CellNumber<SQUARE_SIZE>) -> Self {
        Self {
            state: CellState::Given(hint),
        }
    }

    /// Return the cell number or None if it is empty
    pub fn cell_number(&self) -> Option<CellNumber<SQUARE_SIZE>> {
        self.state().cell_number()
    }
}

/// Represent a number that a cell can hold. Can only hold 1 thought 9
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash, Serialize, Deserialize)]
pub struct CellNumber<const SQUARE_SIZE: usize> {
    number: usize,
}
// TODO u8 ?

impl<const SQUARE_SIZE: usize> CellNumber<SQUARE_SIZE> {
    /// Test if the given value is in bounds
    /// # Example
    /// ```
    /// use sudoku::cell::CellNumber;
    ///
    /// const GAME_SIZE: usize = 9;
    ///
    /// // in bounds
    /// assert!(CellNumber::<3>::is_in_bound(1));
    /// assert!(CellNumber::<3>::is_in_bound(GAME_SIZE - 1));
    /// assert!(CellNumber::<3>::is_in_bound(GAME_SIZE));
    ///
    /// // not in bounds
    /// assert!(!CellNumber::<3>::is_in_bound(0));
    /// assert!(!CellNumber::<3>::is_in_bound(GAME_SIZE + 100));
    /// ```
    pub const fn is_in_bound(number: usize) -> bool {
        number <= SQUARE_SIZE.pow(2) && number > 0
    }

    /// Create a new cell number. the input should be <= [`GAME_SIZE`] otherwise return [`None`]
    /// # Example
    /// ```
    /// use sudoku::cell::CellNumber;
    ///
    /// assert!(CellNumber::<3>::new(0).is_none());
    /// assert!(CellNumber::<3>::new(1).is_some());
    /// assert!(CellNumber::<3>::new(8).is_some());
    /// assert!(CellNumber::<3>::new(9).is_some());
    /// assert!(CellNumber::<3>::new(10).is_none());
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
    /// let cell = CellNumber::<3>::new(2).ok_or(ExampleError::NoneError)?;
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
    /// let mut c = CellNumber::<3>::new(1).ok_or(ExampleError::NoneError)?;
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

impl<const SQUARE_SIZE: usize> Default for CellNumber<SQUARE_SIZE> {
    /// Create a [`CellNumber`] with value 1
    /// # Example
    /// ```
    /// use sudoku::cell::CellNumber;
    ///
    /// assert_eq!(CellNumber::<3>::default().number(), 1);
    /// ```
    fn default() -> Self {
        Self::new(1).expect("unreacharble")
    }
}

impl<const SQUARE_SIZE: usize> Display for CellNumber<SQUARE_SIZE> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.number)
    }
}

impl<const SQUARE_SIZE: usize> Binary for CellNumber<SQUARE_SIZE> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:b}", self.number)
    }
}

impl<const SQUARE_SIZE: usize> UpperHex for CellNumber<SQUARE_SIZE> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:X}", self.number)
    }
}

impl<const SQUARE_SIZE: usize> LowerHex for CellNumber<SQUARE_SIZE> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:x}", self.number)
    }
}

impl<const SQUARE_SIZE: usize> Octal for CellNumber<SQUARE_SIZE> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:o}", self.number)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn t_display() {
        for n in 1..=9 {
            let cell = CellNumber::<3>::new(n).unwrap();
            assert_eq!(format!("{}", cell), format!("{}", n));
            assert_eq!(format!("{:b}", cell), format!("{:b}", n));
            assert_eq!(format!("{:X}", cell), format!("{:X}", n));
            assert_eq!(format!("{:x}", cell), format!("{:x}", n));
            assert_eq!(format!("{:o}", cell), format!("{:o}", n));
        }
    }
}
