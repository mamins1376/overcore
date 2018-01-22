use std::{iter, fmt};
use std::ops::{Deref, DerefMut};
use super::Buffer;

pub mod prelude {
    pub use super::{NoteAlphabet, NoteName, NoteRef};
    pub use super::{NoteParam, NoteParams, Note};
    pub use super::{Event, Moment, ControlBuffer};
}

mod note;
pub use self::note::*;

mod param;
pub use self::param::*;

#[derive(Debug, PartialEq, Clone)]
/// A message containing control state changes.
pub enum Event {
    NoteOn(Note, Option<usize>),
    NoteSet(NoteRef, NoteParam),
    NoteOff(NoteRef),
    ParamSet(usize, ParamValue),
    ParamReset(usize),
    Panic
}

/// A moment that contains some events.
pub type Moment = Option<Box<[Event]>>;

#[derive(Clone)]
pub struct ControlBuffer(Box<[Moment]>);

impl Buffer for ControlBuffer {
    fn with_length(length: usize) -> Self {
        let moments = iter::repeat(None).take(length).collect::<Vec<_>>();
        Self { 0: moments.into_boxed_slice() }
    }

    /// Gain velocity of all notes in the buffer.
    fn gain<T: Into<f64>>(&mut self, gain: T) {
        let gain = gain.into();
        for moment in self.iter_mut() {
            if let &mut Some(ref mut events) = moment {
                for event in events.iter_mut() {
                    if let &mut Event::NoteOn(ref mut note, _) = event {
                        note.gain(gain);
                    }
                }
            }
        }
    }

    fn clear(&mut self) {
        self.iter_mut().for_each(|moment| *moment = None)
    }
}

impl fmt::Debug for ControlBuffer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let len = self.len();
        write!(f, "ControlBuffer {{ len: {}, [", len)?;
        for (i, frame) in self.iter().enumerate() {
            if i >= 3 { write!(f, "...")?; break; }
            write!(f, "{:?}, ", *frame)?;
        }
        write!(f, "] }}")
    }
}

impl Deref for ControlBuffer {
    type Target = Box<[Moment]>;

    fn deref(&self) -> &Self::Target { &self.0 }
}

impl DerefMut for ControlBuffer {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}
