#[cfg(not(feature = "std"))]
use core::{fmt::Debug, prelude::rust_2021::derive};
#[cfg(feature = "std")]
use std::fmt::{Display, Formatter, Result};

/// Error returned if the supplied buffer is too small.
#[derive(Clone, Debug)]
pub struct BufferTooSmall {
    pub size: usize,
    pub expected: usize,
}

#[cfg(feature = "std")]
impl Display for BufferTooSmall {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "The supplied buffer is to small. Got {}, expected at least {}",
            self.size, self.expected,
        )
    }
}
