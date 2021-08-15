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

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Ord, Eq, Hash, Serialize, Deserialize)]
enum Direction {
    Forward,
    Backward,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq, Hash, Serialize, Deserialize)]
struct BackTracePositionTracker {
    el: ElementTracker<CellPosition>,
}

impl BackTracePositionTracker {
    pub const fn new() -> Self {
        Self {
            el: ElementTracker::First,
        }
    }

    pub fn next(&mut self) -> Option<CellPosition> {
        self.move_pos(Direction::Forward)
    }

    pub fn previous(&mut self) -> Option<CellPosition> {
        self.move_pos(Direction::Backward)
    }

    fn move_pos(&mut self, d: Direction) -> Option<CellPosition> {
        match self.el {
            ElementTracker::First => match d {
                Direction::Forward => {
                    let pos = CellPosition::new_from_number(0, 0).unwrap();
                    self.el = ElementTracker::Element(pos);
                    Some(pos)
                }
                Direction::Backward => None,
            },
            ElementTracker::Element(pos) => {
                let offset = match d {
                    Direction::Forward => 1_isize,
                    Direction::Backward => -1_isize,
                };
                todo!()
            }
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
        self.next()
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
