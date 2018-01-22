use std::{iter, fmt};
use std::ops::{Deref, DerefMut};
use super::Buffer;

pub mod prelude {
    pub use super::{Sample, Frame, AudioBuffer};
}

mod frame;
pub use self::frame::*;

/// A single sample representing signal amplitude.
pub type Sample = f64;

#[derive(Clone)]
/// A buffer holding audio frames.
pub struct AudioBuffer(Box<[Frame]>);

impl Buffer for AudioBuffer {
    fn with_length(length: usize) -> Self {
        let frames = iter::repeat(Default::default()).take(length);
        Self { 0: frames.collect::<Vec<_>>().into_boxed_slice() }
    }

    fn gain<T: Into<f64>>(&mut self, gain: T) {
        let gain = gain.into();
        self.0.iter_mut().for_each(|f| f.iter_mut().for_each(|s| *s *= gain));
    }

    fn clear(&mut self) {
        self.0.iter_mut().for_each(|f| *f = Default::default());
    }
}

impl fmt::Debug for AudioBuffer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let len = self.len();
        write!(f, "AudioBuffer {{ length: {}, [", len)?;
        for (i, frame) in self.iter().enumerate() {
            if i >= 3 { write!(f, "...")?; break; }
            write!(f, "{:?}, ", *frame)?;
        }
        write!(f, "] }}")
    }
}

impl Deref for AudioBuffer {
    type Target = Box<[Frame]>;

    fn deref(&self) -> &Self::Target { &self.0 }
}

impl DerefMut for AudioBuffer {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}
