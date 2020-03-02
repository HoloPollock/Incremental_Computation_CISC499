
use std::fmt;

#[derive(Clone, PartialEq, Eq)]
pub struct ParseOperationError {}

impl fmt::Display for ParseOperationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        "provided string was not a math operation".fmt(f)
    }
}

impl fmt::Debug for ParseOperationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        "provided string was not a math operation".fmt(f)
    }
}

impl std::error::Error for ParseOperationError {}

