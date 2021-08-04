use std::fmt::{Display, Formatter};
use std::ops::{Index, IndexMut};

use serde::{Deserialize, Serialize};

use crate::error::SetError;
use crate::GAME_SIZE;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash, Serialize, Deserialize)]
#[allow(clippy::exhaustive_enums)]
// TODO
enum CellState {
    Number(CellNumber),
    Empty(CellPossibilities),
}

impl Default for CellState {
    fn default() -> Self {
        Self::Empty(CellPossibilities::default())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash, Serialize, Deserialize)]
struct CellPossibilities {
    possibility: [bool; GAME_SIZE],
}

//TODO range
impl Index<CellNumber> for CellPossibilities {
    type Output = bool;

    fn index(&self, pos: CellNumber) -> &Self::Output {
        &self.possibility[pos.number() - 1]
    }
}

impl IndexMut<CellNumber> for CellPossibilities {
    fn index_mut(&mut self, pos: CellNumber) -> &mut Self::Output {
        &mut self.possibility[pos.number() - 1]
    }
}

impl Default for CellPossibilities {
    fn default() -> Self {
        Self {
            possibility: [false; GAME_SIZE],
        }
    }
}

#[derive(
    Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash, Serialize, Deserialize, Default,
)]
pub struct Cell {
    state: CellState,
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
