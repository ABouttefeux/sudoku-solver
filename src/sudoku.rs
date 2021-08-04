use std::fmt::{Display, Formatter};
use std::ops::{Index, IndexMut};

use serde::{Deserialize, Serialize};

use crate::cell::Cell;
use crate::error::SetError;
use crate::GAME_SIZE;

/// Position in a square
// TODO keep ?
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash, Serialize, Deserialize)]
#[allow(clippy::exhaustive_enums)]
pub enum SquarePositoin {
    /// Top left
    TopLeft,
    /// Top
    Top,
    /// Top right
    TopRight,
    /// Left
    Left,
    /// Middle
    Middle,
    /// Right
    Right,
    /// BottomLeft
    BottomLeft,
    /// Bottom
    Bottom,
    /// BottomRight
    BottomRight,
}

/// Represent a coordinate on a [`Sudoku`] grid. It is a number between 0 and [`GAME_SIZE`] - 1 (which is 8).
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash, Serialize, Deserialize)]
pub struct CellCoordinate {
    number: usize,
}

impl CellCoordinate {
    /// Test if the given value is in bounds
    pub const fn is_in_bound(number: usize) -> bool {
        number < GAME_SIZE
    }

    /// Create a new cell number. the input should be < 10 otherwise return [`None`]
    /// # Example
    /// ```
    /// use sudoku::sudoku::CellCoordinate;
    ///
    /// assert!(CellCoordinate::new(0).is_some());
    /// assert!(CellCoordinate::new(1).is_some());
    /// assert!(CellCoordinate::new(8).is_some());
    /// assert!(CellCoordinate::new(9).is_none());
    /// assert!(CellCoordinate::new(10).is_none());
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
    /// use sudoku::sudoku::CellCoordinate;
    /// # use std::error::Error;
    ///
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// let cell = CellCoordinate::new(2).ok_or(ExampleError::NoneError)?;
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
    /// use sudoku::sudoku::CellCoordinate;
    ///
    /// let mut c = CellCoordinate::new(0).unwrap();
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

impl Default for CellCoordinate {
    fn default() -> Self {
        Self::new(0).expect("unreachable")
    }
}

impl Display for CellCoordinate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.number)
    }
}

#[derive(
    Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash, Serialize, Deserialize, Default,
)]
/// Represnet a [`Cell`] position on a [`Sudoku`]
pub struct CellPosition {
    x: CellCoordinate,
    y: CellCoordinate,
}

impl CellPosition {
    /// Create a new position.
    pub const fn new(x: CellCoordinate, y: CellCoordinate) -> Self {
        Self { x, y }
    }

    /// Get the x coord.
    pub const fn x(&self) -> CellCoordinate {
        self.x
    }

    /// Get the y coord.
    pub const fn y(&self) -> CellCoordinate {
        self.y
    }

    /// Get the x coord as a [`usize`].
    pub const fn x_usize(&self) -> usize {
        self.x().number()
    }

    /// Get the y coord as a [`usize`].
    pub const fn y_usize(&self) -> usize {
        self.y().number()
    }
}

#[derive(
    Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash, Serialize, Deserialize, Default,
)]
pub struct Sudoku {
    data: [[Cell; GAME_SIZE]; GAME_SIZE],
}

impl Sudoku {
    /// Get a reference to the cell at the given position
    pub const fn get_cell(&self, index: CellPosition) -> &Cell {
        &self.data[index.x_usize()][index.y_usize()]
    }

    /// Get a mut reference to the cell at the given position
    pub fn get_cell_mut(&mut self, index: CellPosition) -> &mut Cell {
        &mut self.data[index.x_usize()][index.y_usize()]
    }

    pub fn solve(&mut self) -> &mut Self {
        //TODO
        todo!()
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
