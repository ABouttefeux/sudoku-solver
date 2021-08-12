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
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
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
    pub const fn square_size(&self) -> usize {
        self.square_size
    }

    /// Get the game size
    pub const fn game_size(&self) -> usize {
        self.square_size() * self.square_size()
    }

    /// Get the number of cell in total
    pub const fn cell_number(&self) -> usize {
        self.game_size() * self.game_size()
    }
}

impl Default for GameSize {
    /// Returns the classic 9X9 grid
    fn default() -> Self {
        Self::new_square_size(3)
    }
}
