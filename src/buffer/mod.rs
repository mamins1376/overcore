pub mod prelude {
    pub use super::audio::prelude::*;
    pub use super::control::prelude::*;
}

pub mod audio;
pub mod control;

mod pool;
pub use self::pool::*;

/// A trait to abstract buffers methods.
pub trait Buffer: Clone {
    /// Allocate a buffer with length `length`.
    fn with_length(length: usize) -> Self;

    /// Multiply inner items by given `value`.
    fn gain<T: Into<f64>>(&mut self, gain: T);

    /// Clear the buffer for reuse.
    fn clear(&mut self);
}
