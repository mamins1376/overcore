use std::sync::Mutex;
use std::ops::{Deref, DerefMut};
use super::Buffer;

pub struct BufferPool<T> {
    length: usize,
    stack: Mutex<Vec<T>>
}

impl<T: Buffer> BufferPool<T> {
    pub fn new(length: usize) -> Self {
        Self { length, stack: Mutex::new(Vec::new()) }
    }

    /// Allocate new buffers.
    pub fn allocate(&self, n: usize) {
        let mut stack = self.stack.lock().unwrap();
        (0..n).map(|_| T::with_length(self.length)).for_each(|b| stack.push(b));
    }

    pub fn len() -> usize {
        let stack = self.stack.lock().unwrap();
        stack.len()
    }

    pub fn acquire(&self) -> BufferGuard<T> {
        let buffer = { let mut stack = self.stack.lock().unwrap(); stack.pop() };
        let buffer = buffer.unwrap_or_else(|| T::with_length(self.length));
        BufferGuard { stack: &self.stack, buffer: Some(buffer) }
    }

    pub fn guard(&self) -> BufferGuard<T> {
        BufferGuard { stack: &self.stack, buffer: None }
    }
}

pub struct BufferGuard<'a, T: Buffer + 'a> {
    stack: &'a Mutex<Vec<T>>,
    buffer: Option<T>
}

impl<'a, T: Buffer> BufferGuard<'a, T> {
    pub fn has_buffer(&self) -> bool {
        self.buffer.is_some()
    }

    pub fn put(&mut self, buffer: T) {
        self.buffer = Some(buffer);
    }

    pub fn take(&mut self) -> Option<T> {
        self.buffer.take()
    }

    pub fn unwrap(mut self) -> T {
        self.take().unwrap()
    }
}

impl<'a, T: Buffer> Deref for BufferGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.buffer.as_ref().unwrap()
    }
}

impl<'a, T: Buffer> DerefMut for BufferGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        self.buffer.as_mut().unwrap()
    }
}

impl<'a, T: Buffer> Drop for BufferGuard<'a, T> {
    fn drop(&mut self) {
        if let Some(mut buffer) = self.buffer.take() {
            buffer.clear();
            let mut stack = self.stack.lock().unwrap();
            stack.push(buffer);
        }
    }
}
