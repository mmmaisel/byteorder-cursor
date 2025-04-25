use core::fmt::{Display, Formatter, Result};
#[cfg(not(feature = "std"))]
use core::{fmt::Debug, prelude::rust_2021::derive};

/// Error returned if the supplied buffer is too small.
#[derive(Clone, Debug)]
pub struct BufferTooSmall {
    pub size: usize,
    pub expected: usize,
}

impl Display for BufferTooSmall {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "The supplied buffer is to small. Got {}, expected at least {}",
            self.size, self.expected,
        )
    }
}
