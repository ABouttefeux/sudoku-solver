//! Contains [`CellPossibilities`] a structure to keep track of which number can be placed inside a cell

use std::iter::FusedIterator;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Index, IndexMut};

use serde::{Deserialize, Serialize};

use crate::cell::CellNumber;
use crate::GAME_SIZE;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash, Serialize, Deserialize)]
/// TODO remove pub crate
pub(crate) struct CellPossibilities {
    possibility: [bool; GAME_SIZE],
}

impl CellPossibilities {
    /// Create a new configuration where all the numbers are possible.
    pub const fn new() -> Self {
        Self {
            possibility: [true; GAME_SIZE],
        }
    }

    /// Create a new configuration with no possiblity at all.
    pub const fn new_no_possibility() -> Self {
        Self {
            possibility: [false; GAME_SIZE],
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

    pub fn cell_number(&self) -> Option<CellNumber> {
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

    pub fn into_vec(self) -> Vec<CellNumber> {
        IntoIterator::into_iter(self.possibility)
            .enumerate()
            .filter_map(|(index, b)| {
                if b {
                    Some(CellNumber::new(index + 1).unwrap())
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
        Self::new()
    }
}

impl BitAnd for CellPossibilities {
    type Output = Self;

    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}

impl BitAndAssign for CellPossibilities {
    fn bitand_assign(&mut self, rhs: Self) {
        for (b1, b2) in self.iter_mut().zip(rhs.iter()) {
            *b1 &= b2;
        }
    }
}

impl BitOr for CellPossibilities {
    type Output = Self;

    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}

impl BitOrAssign for CellPossibilities {
    fn bitor_assign(&mut self, rhs: Self) {
        for (b1, b2) in self.iter_mut().zip(rhs.iter()) {
            *b1 |= b2;
        }
    }
}

impl From<CellPossibilities> for Vec<CellNumber> {
    fn from(poss: CellPossibilities) -> Self {
        poss.into_vec()
    }
}
