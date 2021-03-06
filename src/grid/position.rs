use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

use crate::error::SetError;

/// Represent a coordinate on a [`crate::grid::Sudoku`] grid. It is a number between 0 and [`GAME_SIZE`] - 1 (which is 8).
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash, Serialize, Deserialize)]
pub struct CellCoordinate<const SQUARE_SIZE: usize> {
    number: usize,
}

impl<const SQUARE_SIZE: usize> CellCoordinate<SQUARE_SIZE> {
    /// Test if the given value is in bounds
    pub const fn is_in_bound(number: usize) -> bool {
        number < SQUARE_SIZE.pow(2)
    }

    /// Create a new cell number. the input should be `< 10` otherwise return [`None`]
    /// # Example
    /// ```
    /// use sudoku::grid::CellCoordinate;
    ///
    /// assert!(CellCoordinate::<3>::new(0).is_some());
    /// assert!(CellCoordinate::<3>::new(1).is_some());
    /// assert!(CellCoordinate::<3>::new(8).is_some());
    /// assert!(CellCoordinate::<3>::new(9).is_none());
    /// assert!(CellCoordinate::<3>::new(10).is_none());
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
    /// use sudoku::error::ExampleError;
    /// use sudoku::grid::CellCoordinate;
    /// # use std::error::Error;
    ///
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// let cell = CellCoordinate::<3>::new(2).ok_or(ExampleError::NoneError)?;
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
    /// use sudoku::grid::CellCoordinate;
    ///
    /// let mut c = CellCoordinate::<3>::new(0).unwrap();
    /// c.set_number(2).unwrap();
    /// assert_eq!(c.number(), 2);
    /// assert!(c.set_number(9).is_err());
    /// assert_eq!(c.number(), 2);
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

impl<const SQUARE_SIZE: usize> Default for CellCoordinate<SQUARE_SIZE> {
    fn default() -> Self {
        Self::new(0).expect("unreachable")
    }
}

impl<const SQUARE_SIZE: usize> Display for CellCoordinate<SQUARE_SIZE> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.number)
    }
}

#[derive(
    Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash, Serialize, Deserialize, Default,
)]
/// Represent a [`crate::cell::Cell`] position on a [`crate::grid::Sudoku`]
pub struct CellPosition<const SQUARE_SIZE: usize> {
    x: CellCoordinate<SQUARE_SIZE>,
    y: CellCoordinate<SQUARE_SIZE>,
}

impl<const SQUARE_SIZE: usize> CellPosition<SQUARE_SIZE> {
    /// Create a new position.
    pub const fn new(x: CellCoordinate<SQUARE_SIZE>, y: CellCoordinate<SQUARE_SIZE>) -> Self {
        Self { x, y }
    }

    /// Get the x coord.
    pub const fn x(&self) -> CellCoordinate<SQUARE_SIZE> {
        self.x
    }

    /// Get the y coord.
    pub const fn y(&self) -> CellCoordinate<SQUARE_SIZE> {
        self.y
    }

    /// Get the x coord as mut.
    pub fn x_mut(&mut self) -> &mut CellCoordinate<SQUARE_SIZE> {
        &mut self.x
    }

    /// Get the y coord as mut.
    pub fn y_mut(&mut self) -> &mut CellCoordinate<SQUARE_SIZE> {
        &mut self.y
    }

    /// Get the x coord as a [`usize`].
    pub const fn x_usize(&self) -> usize {
        self.x().number()
    }

    /// Get the y coord as a [`usize`].
    pub const fn y_usize(&self) -> usize {
        self.y().number()
    }

    /// Try create a new [`CellPosition`] from 2 usize
    pub fn new_from_number(x: usize, y: usize) -> Option<Self> {
        Some(Self::new(CellCoordinate::new(x)?, CellCoordinate::new(y)?))
    }
}

impl<const SQUARE_SIZE: usize> Display for CellPosition<SQUARE_SIZE> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
