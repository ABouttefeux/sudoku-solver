use std::iter::FusedIterator;

use serde::{Deserialize, Serialize};

use crate::grid::{CellCoordinate, CellPosition};
use crate::{Sealed, GAME_SIZE, SQUARE_SIZE};

#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq, Hash, Serialize, Deserialize, Default)]
/// Row iterator of a [`Sudoku`]. It iterate over the first coordinate.
pub struct Row {
    position: Option<CellPosition>,
}

impl Row {
    /// Create a iterator of the row at the given position.
    pub fn new(position: CellPosition) -> Self {
        let top_pos = CellPosition::new(CellCoordinate::new(0).unwrap(), position.y());
        Self {
            position: Some(top_pos),
        }
    }
}

impl Iterator for Row {
    type Item = CellPosition;

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
        let number = self.position.map_or(0, |pos| GAME_SIZE - pos.x_usize());
        (number, Some(number))
    }
}

impl ExactSizeIterator for Row {}

impl FusedIterator for Row {}

#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq, Hash, Serialize, Deserialize, Default)]
/// Column iterator of a [`Sudoku`]. It iterate over the second coordinate.
pub struct Column {
    position: Option<CellPosition>,
}

impl Column {
    /// Create a iterator of the column at the given position.
    pub fn new(position: CellPosition) -> Self {
        let left_pos = CellPosition::new(position.x(), CellCoordinate::new(0).unwrap());
        Self {
            position: Some(left_pos),
        }
    }
}

impl Iterator for Column {
    type Item = CellPosition;

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
        let number = self.position.map_or(0, |pos| GAME_SIZE - pos.y_usize());
        (number, Some(number))
    }
}

impl ExactSizeIterator for Column {}

impl FusedIterator for Column {}

#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq, Hash, Serialize, Deserialize, Default)]
/// Square iterator of a [`Sudoku`].
pub struct Square {
    position: Option<CellPosition>,
}

impl Square {
    /// Create a iterator of the square at the given position.
    pub fn new(position: CellPosition) -> Self {
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

impl Iterator for Square {
    type Item = CellPosition;

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
            GAME_SIZE - (pos.x_usize() % SQUARE_SIZE) + SQUARE_SIZE * (pos.y_usize() % SQUARE_SIZE)
        });
        (number, Some(number))
    }
}

impl ExactSizeIterator for Square {}

impl FusedIterator for Square {}

trait SudokuIter: Sealed + Iterator<Item = CellPosition> {}

impl Sealed for Row {}
impl SudokuIter for Row {}
impl Sealed for Column {}
impl SudokuIter for Column {}
impl Sealed for Square {}
impl SudokuIter for Square {}

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
pub struct BackTracePositionTracker {
    el: ElementTracker<CellPosition>,
}

impl BackTracePositionTracker {
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
    pub fn next_element(&mut self) -> Option<CellPosition> {
        self.move_pos(Direction::Forward)
    }

    /// Get the previous position, see [`BackTracePositionTracker::move_pos`]
    pub fn previous(&mut self) -> Option<CellPosition> {
        self.move_pos(Direction::Backward)
    }

    /// Move the position in a certain direction.
    /// Returns [`None`] if at the end of the configuration
    pub fn move_pos(&mut self, d: Direction) -> Option<CellPosition> {
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
                            let x = GAME_SIZE - 1;
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
                    let pos = CellPosition::new_from_number(GAME_SIZE - 1, GAME_SIZE - 1).unwrap();
                    self.el = ElementTracker::Element(pos);
                    Some(pos)
                }
            },
        }
    }
}

impl Iterator for BackTracePositionTracker {
    type Item = CellPosition;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_element()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        todo!()
    }
}

impl Default for BackTracePositionTracker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn back_trace_iter_basic() {
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
