//! Module for the [`GameSize`] which define the size of the game

use approx::relative_eq;
use serde::{Deserialize, Serialize};

/// Represent a game size
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash, Serialize, Deserialize)]
pub struct GameSize {
    square_size: usize,
}

impl GameSize {
    /// Create a gme size form a square size, the total number of cells are square_size^2
    pub const fn new_square_size(square_size: usize) -> Self {
        Self { square_size }
    }

    // both OK as the inital number is a usize

    /// Try to create a game size from the input number, only accept square number
    /// # Example
    /// ```
    /// use sudoku::size::GameSize;
    ///
    /// assert_eq!(GameSize::new(16).unwrap(), GameSize::new_square_size(4));
    /// assert_eq!(GameSize::new(25).unwrap(), GameSize::new_square_size(5));
    /// assert_eq!(GameSize::new(36).unwrap(), GameSize::new_square_size(6));
    /// assert!(GameSize::new(15).is_none());
    /// assert!(GameSize::new(35).is_none());
    /// assert!(GameSize::new(50).is_none());
    /// ```
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    pub fn new(size: usize) -> Option<Self> {
        let sqrt = (size as f64).sqrt();
        if relative_eq!(sqrt, sqrt.round()) {
            Some(Self {
                square_size: sqrt.round() as usize,
            })
        } else {
            None
        }
    }

    /// Get the square size
    /// # Example
    /// ```
    /// use sudoku::size::GameSize;
    ///
    /// let size = GameSize::new(16).unwrap();
    /// assert_eq!(size.square_size(), 4);
    /// let size = GameSize::new_square_size(3);
    /// assert_eq!(size.square_size(), 3);
    /// ```
    pub const fn square_size(&self) -> usize {
        self.square_size
    }

    /// Get the game size
    /// # Example
    /// ```
    /// use sudoku::size::GameSize;
    ///
    /// let size = GameSize::new(16).unwrap();
    /// assert_eq!(size.game_size(), 16);
    /// let size = GameSize::new_square_size(3);
    /// assert_eq!(size.game_size(), 9);
    /// ```
    pub const fn game_size(&self) -> usize {
        self.square_size() * self.square_size()
    }

    /// Get the number of cell in total
    /// # Example
    /// ```
    /// use sudoku::size::GameSize;
    ///
    /// let size = GameSize::new(16).unwrap();
    /// assert_eq!(size.cell_number(), 16 * 16);
    /// let size = GameSize::new_square_size(3);
    /// assert_eq!(size.cell_number(), 81);
    /// ```
    pub const fn cell_number(&self) -> usize {
        self.game_size() * self.game_size()
    }
}

impl Default for GameSize {
    /// Returns the classic 9X9 grid
    /// # Example
    /// ```
    /// use sudoku::size::GameSize;
    ///
    /// assert_eq!(GameSize::default(), GameSize::new_square_size(3));
    /// ```
    fn default() -> Self {
        Self::new_square_size(3)
    }
}
