use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use ::buffer::prelude::*;

#[derive(Default, Clone)]
/// A place holding active notes and custom data of type `T` per one.
pub struct ActiveNotes<T>(HashMap<NoteRef, (Note, T)>);

impl<T> ActiveNotes<T> {
    #[inline]
    /// Append `note` and `value` to the store.
    pub fn store(&mut self, note: Note, value: T) {
        self.insert(note.name.clone(), (note, value));
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item=(&Note, &T)> {
        self.0.values().map(|p| { let &(ref n, ref t) = p; (n, t) })
    }

    #[inline]
    pub fn iter_mut(&mut self) -> impl Iterator<Item=(&mut Note, &mut T)> {
        self.0.values_mut()
            .map(|p| { let &mut (ref mut n, ref mut t) = p; (n, t) })
    }
}

impl<T: Clone> ActiveNotes<T> {
    /// Apply event `event`, use `value` as value if `event` is a
    /// [`Event::NoteOn`](../buffer/control/enum.Event.html#variant.NoteOn).
    pub fn apply_event(&mut self, event: &Event, value: &T) {
        match event {
            &Event::NoteOn(ref note, _) => {
                self.store(note.clone(), value.clone())
            },
            &Event::NoteSet(ref name, ref param) => {
                self.0.get_mut(name).map(|pack| {
                    let &mut (ref mut note, _) = pack;
                    note.params.apply(param);
                });
            },
            &Event::NoteOff(ref name) => {
                self.0.remove(name);
            },
            &Event::Panic => {
                self.0.clear()
            },
            _ => {}
        }
    }

    #[inline]
    /// Update internal params according to `moment`.
    pub fn apply_moment(&mut self, moment: &Moment, value: &T) {
        let apply = |event| self.apply_event(event, value);
        moment.as_ref().map(|events| events.iter().for_each(apply));
    }
}

impl<T> Deref for ActiveNotes<T> {
    type Target = HashMap<NoteRef, (Note, T)>;

    fn deref(&self) -> &Self::Target { &self.0 }
}

impl<T> DerefMut for ActiveNotes<T> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}
