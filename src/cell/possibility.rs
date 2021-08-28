//! Contains [`CellPossibilities`] a structure to keep track of which number can be placed inside a cell

use std::iter::FusedIterator;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Index, IndexMut};

use serde::{Deserialize, Serialize};

use crate::cell::CellNumber;

/// Represent the pssibility of number that call can have
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash, Serialize, Deserialize)]
// TODO remove pub crate
pub(crate) struct CellPossibilities<const SQUARE_SIZE: usize>
where
    [bool; SQUARE_SIZE * SQUARE_SIZE]: Sized,
{
    #[serde(bound(
        serialize = "[bool; SQUARE_SIZE * SQUARE_SIZE]: Serialize",
        deserialize = "[bool; SQUARE_SIZE * SQUARE_SIZE]: Deserialize<'de>"
    ))]
    possibility: [bool; SQUARE_SIZE * SQUARE_SIZE],
}

impl<const SQUARE_SIZE: usize> CellPossibilities<SQUARE_SIZE>
where
    [bool; SQUARE_SIZE * SQUARE_SIZE]: Sized,
{
    /// Create a new configuration where all the numbers are possible.
    pub const fn new() -> Self {
        Self {
            possibility: [true; SQUARE_SIZE * SQUARE_SIZE],
        }
    }

    /// Create a new configuration with no possiblity at all.
    pub const fn new_no_possibility() -> Self {
        Self {
            possibility: [false; SQUARE_SIZE * SQUARE_SIZE],
        }
    }

    // TODO

    // pub fn get(&self, pos: CellNumber) -> &bool {
    //     &self[pos]
    // }

    // pub fn get_mut(&mut self, pos: CellNumber) -> &mut bool {
    //     &mut self[pos]
    // }

    // pub fn remove_possibility(&mut self, pos: CellNumber) -> &mut Self {
    //     self[pos] = false;
    //     self
    // }

    pub fn number_of_possibility(&self) -> usize {
        self.possibility.iter().filter(|b| **b).count()
    }

    pub fn cell_number(&self) -> Option<CellNumber<SQUARE_SIZE>> {
        let mut number = None;
        for (index, poss) in self.possibility.iter().enumerate() {
            if *poss {
                if number.is_none() {
                    number = CellNumber::new(index + 1);
                    // the number should always be created with Some(value)
                    debug_assert!(number.is_some());
                } else {
                    return None;
                }
            }
        }
        number
    }

    /// returns the possibility as a (sorted) vector
    pub fn into_vec(self) -> Vec<CellNumber<SQUARE_SIZE>> {
        IntoIterator::into_iter(self.possibility)
            .enumerate()
            .filter_map(|(index, b)| {
                if b {
                    Some(CellNumber::new(index + 1).unwrap())
                    // let nb = CellNumber::new(index + 1);
                    // debug_assert!(nb.is_some());
                    // nb
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn iter(
        &self,
    ) -> impl Iterator<Item = &bool> + FusedIterator + ExactSizeIterator + DoubleEndedIterator {
        self.possibility.iter()
    }

    pub fn iter_mut(
        &mut self,
    ) -> impl Iterator<Item = &mut bool> + FusedIterator + ExactSizeIterator + DoubleEndedIterator
    {
        self.possibility.iter_mut()
    }
}

//TODO range
impl<const SQUARE_SIZE: usize> Index<CellNumber<SQUARE_SIZE>> for CellPossibilities<SQUARE_SIZE>
where
    [bool; SQUARE_SIZE * SQUARE_SIZE]: Sized,
{
    type Output = bool;

    fn index(&self, pos: CellNumber<SQUARE_SIZE>) -> &Self::Output {
        &self.possibility[pos.number() - 1]
    }
}

impl<const SQUARE_SIZE: usize> IndexMut<CellNumber<SQUARE_SIZE>> for CellPossibilities<SQUARE_SIZE>
where
    [bool; SQUARE_SIZE * SQUARE_SIZE]: Sized,
{
    fn index_mut(&mut self, pos: CellNumber<SQUARE_SIZE>) -> &mut Self::Output {
        &mut self.possibility[pos.number() - 1]
    }
}

impl<const SQUARE_SIZE: usize> Default for CellPossibilities<SQUARE_SIZE>
where
    [bool; SQUARE_SIZE * SQUARE_SIZE]: Sized,
{
    /// Create a [`CellPossibilities`] with all number possible
    /// # Example
    /// ```ignore
    /// use sudoku::cell::CellPossibilities;
    ///
    /// let poss = CellPossibilities::default();
    /// assert_eq!(CellPossibilities::new(), poss);
    /// assert!(poss.iter/.all(|el| *el));
    /// ```
    fn default() -> Self {
        Self::new()
    }
}

impl<const SQUARE_SIZE: usize> BitAnd for CellPossibilities<SQUARE_SIZE>
where
    [bool; SQUARE_SIZE * SQUARE_SIZE]: Sized,
{
    type Output = Self;

    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}

impl<const SQUARE_SIZE: usize> BitAndAssign for CellPossibilities<SQUARE_SIZE>
where
    [bool; SQUARE_SIZE * SQUARE_SIZE]: Sized,
{
    fn bitand_assign(&mut self, rhs: Self) {
        for (b1, b2) in self.iter_mut().zip(rhs.iter()) {
            *b1 &= b2;
        }
    }
}

impl<const SQUARE_SIZE: usize> BitOr for CellPossibilities<SQUARE_SIZE>
where
    [bool; SQUARE_SIZE * SQUARE_SIZE]: Sized,
{
    type Output = Self;

    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}

impl<const SQUARE_SIZE: usize> BitOrAssign for CellPossibilities<SQUARE_SIZE>
where
    [bool; SQUARE_SIZE * SQUARE_SIZE]: Sized,
{
    fn bitor_assign(&mut self, rhs: Self) {
        for (b1, b2) in self.iter_mut().zip(rhs.iter()) {
            *b1 |= b2;
        }
    }
}

impl<const SQUARE_SIZE: usize> From<CellPossibilities<SQUARE_SIZE>> for Vec<CellNumber<SQUARE_SIZE>>
where
    [bool; SQUARE_SIZE * SQUARE_SIZE]: Sized,
{
    fn from(poss: CellPossibilities<SQUARE_SIZE>) -> Self {
        poss.into_vec()
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn possibilities() {
        let poss = CellPossibilities::default();
        assert_eq!(CellPossibilities::new(), poss);
        assert!(poss.iter().all(|el| *el));
    }
}
