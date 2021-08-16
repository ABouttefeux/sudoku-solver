use serde::{Deserialize, Serialize};

use super::{CellNumber, CellPossibilities};

#[derive(
    Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash, Serialize, Deserialize, Default,
)]
pub(crate) struct CellGuess {
    number: Option<CellNumber>,
    possibility: CellPossibilities,
}

impl CellGuess {
    /// Create a new guess from a list of possibility
    pub fn new(possibility: CellPossibilities) -> Option<Self> {
        for (index, b) in possibility.iter().enumerate() {
            if *b {
                let number = CellNumber::new(index + 1);
                debug_assert!(number.is_some());
                return Some(Self {
                    number,
                    possibility,
                });
            }
        }
        None
    }

    /// mut the contente to refelct the next guess
    pub fn next_guess(&mut self) -> Option<CellNumber> {
        if let Some(number) = self.number {
            for (index, b) in self.possibility.iter().enumerate().skip(number.number()) {
                if *b {
                    let number = CellNumber::new(index + 1);
                    debug_assert!(number.is_some());
                    self.number = number;
                    return number;
                }
            }
        }
        None
    }

    pub const fn cell_number(&self) -> Option<CellNumber> {
        self.number
    }
}
