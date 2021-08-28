use std::iter::FusedIterator;

use serde::{Deserialize, Serialize};

use crate::grid::{CellCoordinate, CellPosition};
use crate::private::Sealed;

/// Row iterator of a [`crate::grid::Sudoku`]. It iterate over the first coordinate.
#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq, Hash, Serialize, Deserialize, Default)]
pub struct Row<const SQUARE_SIZE: usize> {
    position: Option<CellPosition<SQUARE_SIZE>>,
}

impl<const SQUARE_SIZE: usize> Row<SQUARE_SIZE> {
    /// Create a iterator of the row at the given position.
    pub fn new(position: CellPosition<SQUARE_SIZE>) -> Self {
        let top_pos = CellPosition::new(CellCoordinate::new(0).unwrap(), position.y());
        Self {
            position: Some(top_pos),
        }
    }
}

impl<const SQUARE_SIZE: usize> Iterator for Row<SQUARE_SIZE> {
    type Item = CellPosition<SQUARE_SIZE>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.position {
            Some(ref mut position) => {
                let return_val = *position;
                let x = position.x_usize() + 1;
                match position.x_mut().set_number(x) {
                    Ok(_) => {}
                    Err(_) => self.position = None,
                }
                Some(return_val)
            }
            None => None,
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let number = self
            .position
            .map_or(0, |pos| SQUARE_SIZE.pow(2) - pos.x_usize());
        (number, Some(number))
    }
}

impl<const SQUARE_SIZE: usize> ExactSizeIterator for Row<SQUARE_SIZE> {}

impl<const SQUARE_SIZE: usize> FusedIterator for Row<SQUARE_SIZE> {}

#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq, Hash, Serialize, Deserialize, Default)]
/// Column iterator of a [`crate::grid::Sudoku`]. It iterate over the second coordinate.
pub struct Column<const SQUARE_SIZE: usize> {
    position: Option<CellPosition<SQUARE_SIZE>>,
}

impl<const SQUARE_SIZE: usize> Column<SQUARE_SIZE> {
    /// Create a iterator of the column at the given position.
    pub fn new(position: CellPosition<SQUARE_SIZE>) -> Self {
        let left_pos = CellPosition::new(position.x(), CellCoordinate::new(0).unwrap());
        Self {
            position: Some(left_pos),
        }
    }
}

impl<const SQUARE_SIZE: usize> Iterator for Column<SQUARE_SIZE> {
    type Item = CellPosition<SQUARE_SIZE>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.position {
            Some(ref mut position) => {
                let return_val = *position;
                let y = position.y_usize() + 1;
                match position.y_mut().set_number(y) {
                    Ok(_) => {}
                    Err(_) => self.position = None,
                }
                Some(return_val)
            }
            None => None,
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let number = self
            .position
            .map_or(0, |pos| SQUARE_SIZE.pow(2) - pos.y_usize());
        (number, Some(number))
    }
}

impl<const SQUARE_SIZE: usize> ExactSizeIterator for Column<SQUARE_SIZE> {}

impl<const SQUARE_SIZE: usize> FusedIterator for Column<SQUARE_SIZE> {}

#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq, Hash, Serialize, Deserialize, Default)]
/// Square iterator of a [`crate::grid::Sudoku`].
pub struct Square<const SQUARE_SIZE: usize> {
    position: Option<CellPosition<SQUARE_SIZE>>,
}

impl<const SQUARE_SIZE: usize> Square<SQUARE_SIZE> {
    /// Create a iterator of the square at the given position.
    pub fn new(position: CellPosition<SQUARE_SIZE>) -> Self {
        let x_pos = (position.x_usize() / SQUARE_SIZE) * SQUARE_SIZE;
        let y_pos = (position.y_usize() / SQUARE_SIZE) * SQUARE_SIZE;
        // x and y are always valide pos
        let pos_square = CellPosition::new(
            CellCoordinate::new(x_pos).unwrap(),
            CellCoordinate::new(y_pos).unwrap(),
        );
        Self {
            position: Some(pos_square),
        }
    }
}

impl<const SQUARE_SIZE: usize> Iterator for Square<SQUARE_SIZE> {
    type Item = CellPosition<SQUARE_SIZE>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.position {
            Some(position) => {
                let return_val = position;
                let mut x_pos = position.x_usize() + 1;
                let mut y_pos = position.y_usize();
                if x_pos % SQUARE_SIZE == 0 {
                    x_pos -= SQUARE_SIZE;
                    y_pos += 1;
                }
                let position_new = if y_pos % SQUARE_SIZE == 0 && x_pos % SQUARE_SIZE == 0 {
                    None
                } else {
                    Some(CellPosition::new(
                        CellCoordinate::new(x_pos).unwrap(),
                        CellCoordinate::new(y_pos).unwrap(),
                    ))
                };

                self.position = position_new;
                Some(return_val)
            }
            None => None,
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let number = self.position.map_or(0, |pos| {
            SQUARE_SIZE.pow(2) - (pos.x_usize() % SQUARE_SIZE)
                + SQUARE_SIZE * (pos.y_usize() % SQUARE_SIZE)
        });
        (number, Some(number))
    }
}

impl<const SQUARE_SIZE: usize> ExactSizeIterator for Square<SQUARE_SIZE> {}

impl<const SQUARE_SIZE: usize> FusedIterator for Square<SQUARE_SIZE> {}

/// This trait cannot be implemented outside the crate as
/// it depend on a the Sealed which is private
pub trait SudokuIter<const SQUARE_SIZE: usize>:
    Sealed + Iterator<Item = CellPosition<SQUARE_SIZE>>
{
}

impl<const SQUARE_SIZE: usize> Sealed for Row<SQUARE_SIZE> {}
impl<const SQUARE_SIZE: usize> SudokuIter<SQUARE_SIZE> for Row<SQUARE_SIZE> {}
impl<const SQUARE_SIZE: usize> Sealed for Column<SQUARE_SIZE> {}
impl<const SQUARE_SIZE: usize> SudokuIter<SQUARE_SIZE> for Column<SQUARE_SIZE> {}
impl<const SQUARE_SIZE: usize> Sealed for Square<SQUARE_SIZE> {}
impl<const SQUARE_SIZE: usize> SudokuIter<SQUARE_SIZE> for Square<SQUARE_SIZE> {}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Ord, Eq, Hash, Serialize, Deserialize)]
enum ElementTracker<T> {
    First,
    Element(T),
    Last,
}

/// Represent either a forward or backward direction
#[allow(clippy::exhaustive_enums)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Ord, Eq, Hash, Serialize, Deserialize)]
pub enum Direction {
    /// Forward direction i.e. `+1`
    Forward,
    /// Backward direction i.e. `-1`
    Backward,
}

/// An iterator that can move forward or backwward.
/// It is used to track the position in the back trace method
#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq, Hash, Serialize, Deserialize)]
pub struct BackTracePositionTracker<const SQUARE_SIZE: usize> {
    el: ElementTracker<CellPosition<SQUARE_SIZE>>,
}

impl<const SQUARE_SIZE: usize> BackTracePositionTracker<SQUARE_SIZE> {
    /// Create a new tracker stating on the fist ellement
    /// # Example
    /// ```
    /// use sudoku::grid::{BackTracePositionTracker, CellPosition};
    ///
    /// let mut iter = BackTracePositionTracker::new();
    /// assert_eq!(iter.previous(), None);
    /// let mut iter = BackTracePositionTracker::new();
    /// assert_eq!(iter.next(), CellPosition::new_from_number(0, 0));
    /// ```
    pub const fn new() -> Self {
        Self {
            el: ElementTracker::First,
        }
    }

    /// Get the next position see [`BackTracePositionTracker::move_pos`]
    pub fn next_element(&mut self) -> Option<CellPosition<SQUARE_SIZE>> {
        self.move_pos(Direction::Forward)
    }

    /// Get the previous position, see [`BackTracePositionTracker::move_pos`]
    pub fn previous(&mut self) -> Option<CellPosition<SQUARE_SIZE>> {
        self.move_pos(Direction::Backward)
    }

    /// Move the position in a certain direction.
    /// Returns [`None`] if at the end of the configuration
    pub fn move_pos(&mut self, d: Direction) -> Option<CellPosition<SQUARE_SIZE>> {
        match self.el {
            ElementTracker::First => match d {
                Direction::Forward => {
                    let pos = CellPosition::new_from_number(0, 0).unwrap();
                    self.el = ElementTracker::Element(pos);
                    Some(pos)
                }
                Direction::Backward => None,
            },
            ElementTracker::Element(ref mut pos) => match d {
                Direction::Forward => {
                    let x = pos.x_usize() + 1;
                    match pos.x_mut().set_number(x) {
                        Ok(()) => Some(*pos),
                        Err(_) => {
                            pos.x_mut().set_number(0).unwrap();
                            let y = pos.y_usize() + 1;
                            match pos.y_mut().set_number(y) {
                                Ok(()) => Some(*pos),
                                Err(_) => {
                                    self.el = ElementTracker::Last;
                                    None
                                }
                            }
                        }
                    }
                }
                Direction::Backward => {
                    let x = pos.x_usize().checked_sub(1);
                    match x {
                        Some(x) => {
                            pos.x_mut().set_number(x).unwrap();
                            Some(*pos)
                        }
                        None => {
                            let x = SQUARE_SIZE.pow(2) - 1;
                            let y = pos.y_usize().checked_sub(1);
                            match y {
                                Some(y) => {
                                    pos.x_mut().set_number(x).unwrap();
                                    pos.y_mut().set_number(y).unwrap();
                                    Some(*pos)
                                }
                                None => {
                                    self.el = ElementTracker::First;
                                    None
                                }
                            }
                        }
                    }
                }
            },
            ElementTracker::Last => match d {
                Direction::Forward => None,
                Direction::Backward => {
                    let pos = CellPosition::new_from_number(
                        SQUARE_SIZE.pow(2) - 1,
                        SQUARE_SIZE.pow(2) - 1,
                    )
                    .unwrap();
                    self.el = ElementTracker::Element(pos);
                    Some(pos)
                }
            },
        }
    }
}

impl<const SQUARE_SIZE: usize> Iterator for BackTracePositionTracker<SQUARE_SIZE> {
    type Item = CellPosition<SQUARE_SIZE>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_element()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        todo!()
    }
}

impl<const SQUARE_SIZE: usize> Default for BackTracePositionTracker<SQUARE_SIZE> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn back_trace_iter_basic() {
        const GAME_SIZE: usize = 9;
        let mut iter = BackTracePositionTracker::new();
        assert_eq!(iter.next(), CellPosition::new_from_number(0, 0));
        assert_eq!(iter.next(), CellPosition::new_from_number(1, 0));
        for i in 2..GAME_SIZE {
            let pos = CellPosition::new_from_number(i, 0);
            assert!(pos.is_some());
            assert_eq!(iter.next(), pos);
        }
        assert_eq!(iter.next(), CellPosition::new_from_number(0, 1));
    }

    #[test]
    fn back_trace_iter_extensive() {
        const GAME_SIZE: usize = 9;
        let mut iter = BackTracePositionTracker::new();
        for y in 0..GAME_SIZE {
            for x in 0..GAME_SIZE {
                let pos = CellPosition::new_from_number(x, y);
                assert!(pos.is_some());
                assert_eq!(iter.next(), pos);
            }
        }
        for _ in 0_i32..10_i32 {
            assert_eq!(iter.next(), None);
        }

        for y in (0..GAME_SIZE).rev() {
            for x in (0..GAME_SIZE).rev() {
                let pos = CellPosition::new_from_number(x, y);
                assert!(pos.is_some());
                assert_eq!(iter.previous(), pos);
            }
        }

        for _ in 0_i32..10_i32 {
            assert_eq!(iter.previous(), None);
        }
        assert_eq!(iter.next(), CellPosition::new_from_number(0, 0));
        assert_eq!(iter.next(), CellPosition::new_from_number(1, 0));
        for i in 2..GAME_SIZE {
            let pos = CellPosition::new_from_number(i, 0);
            assert!(pos.is_some());
            assert_eq!(iter.next(), pos);
        }
        assert_eq!(iter.previous(), CellPosition::new_from_number(7, 0));
        assert_eq!(iter.next(), CellPosition::new_from_number(8, 0));
        assert_eq!(iter.next(), CellPosition::new_from_number(0, 1));
        assert_eq!(iter.previous(), CellPosition::new_from_number(8, 0));
        assert_eq!(iter.previous(), CellPosition::new_from_number(7, 0));
    }
}
