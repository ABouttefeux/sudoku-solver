use serde::{Deserialize, Serialize};

use super::{CellNumber, CellPossibilities};

/// manage the guess for a cell using the backtracing algorithme
#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq, Hash, Default)]
//TODO Serialize, Deserialize,
pub(crate) struct CellGuess<const SQUARE_SIZE: usize>
where
    [bool; SQUARE_SIZE * SQUARE_SIZE]: Sized,
{
    number: Option<CellNumber<SQUARE_SIZE>>,
    possibility: CellPossibilities<SQUARE_SIZE>,
}

impl<const SQUARE_SIZE: usize> CellGuess<SQUARE_SIZE>
where
    [bool; SQUARE_SIZE * SQUARE_SIZE]: Sized,
{
    /// Create a new guess from a list of possibility.
    /// Use the first possibility as guess
    /// TODO # Example
    pub fn new(possibility: CellPossibilities<SQUARE_SIZE>) -> Option<Self> {
        for (index, b) in possibility.iter().enumerate() {
            if *b {
                let number = CellNumber::new(index + 1);
                // we do not weant to send a None and I find it
                // cleaner to do that than ton unwrapp and rewrap
                debug_assert!(number.is_some());
                return Some(Self {
                    number,
                    possibility,
                });
            }
        }
        None
    }

    /// mut self to give the next guess
    pub fn next_guess(&mut self) -> Option<CellNumber<SQUARE_SIZE>> {
        if let Some(number) = self.number {
            for (index, b) in self.possibility.iter().enumerate().skip(number.number()) {
                if *b {
                    let number = CellNumber::new(index + 1);
                    // we do not weant to send a None and I find it
                    // cleaner to do that than ton unwrapp and rewrap
                    debug_assert!(number.is_some());
                    self.number = number;
                    return number;
                }
            }
            // if we hit here non next guess are found
            self.number = None;
            None
        } else {
            None
        }
    }

    /// Get the number guessed or None is there is no more possible guess
    pub const fn cell_number(&self) -> Option<CellNumber<SQUARE_SIZE>> {
        self.number
    }
}

#[cfg(test)]
mod test {

    use crate::cell::{CellGuess, CellNumber, CellPossibilities};
    #[test]
    fn guess() {
        let mut guess = CellGuess::<3>::new(CellPossibilities::new()).unwrap();
        assert_eq!(guess.cell_number().unwrap(), CellNumber::new(1).unwrap());
        assert_eq!(guess.next_guess().unwrap(), CellNumber::new(2).unwrap());
        assert_eq!(guess.cell_number().unwrap(), CellNumber::new(2).unwrap());
        for i in 3..10 {
            assert_eq!(guess.next_guess().unwrap().number(), i);
        }
        assert_eq!(guess.next_guess(), None);

        let mut poss = CellPossibilities::<3>::new_no_possibility();
        assert!(CellGuess::new(poss).is_none());
        poss[CellNumber::new(4).unwrap()] = true;
        let mut guess = CellGuess::new(poss).unwrap();
        assert_eq!(guess.cell_number().unwrap().number(), 4);
        assert_eq!(guess.next_guess(), None);
        assert_eq!(guess.cell_number(), None);
    }
}
