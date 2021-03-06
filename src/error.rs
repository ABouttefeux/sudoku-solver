//! Module containing the differents error

use std::error::Error;
use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

/// Error used in examples
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum ExampleError {
    /// None on option
    NoneError,
}

impl Display for ExampleError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoneError => write!(f, "none on option"),
        }
    }
}

impl Error for ExampleError {}

/// Error used in examples
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum SetError {
    /// the value is out of bounds.
    ValueOutOfBounds,
}

impl Display for SetError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ValueOutOfBounds => write!(f, "the value is out of bounds"),
        }
    }
}

impl Error for SetError {}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn display() {
        let string = ExampleError::NoneError.to_string();
        assert!(string.contains("none"));
        assert!(string.contains("option"));

        let string = SetError::ValueOutOfBounds.to_string();
        assert!(string.contains("the value"));
        assert!(string.contains("out of bounds"));
    }
}
