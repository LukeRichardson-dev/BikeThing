use crate::prelude::{Result, BikeError};


pub struct Queue<T, const S: usize>{
    data: [Option<T>; S],
    head: usize,
    length: usize,
}

impl<T: Copy, const S: usize> Default for Queue<T, S> {
    fn default() -> Self {
        Self { 
            data: [None; S], 
            head: 0, 
            length: 0,
        }
    }
}

impl<T: Copy, const S: usize> Queue<T, S> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_array(array: &[T]) -> Result<Self> {
        if array.len() > S {
            return Err(BikeError::InvalidArgument("Array length greater than max size parameter S."))
        }

        let mut n = Self::default();
        for (a, b) in array.iter().zip(n.data.iter_mut()) {
            *b = Some(*a);
        }

        Ok(n)
    }

    pub fn enqueue(&mut self, item: T) -> Option<T> {
        self.head += 1;
        self.head %= S;
        
        let out = if self.length == S {
            self.data[self.head]
        } else {
            self.length += 1;
            None
        };
        
        self.data[self.head] = Some(item);

        out
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if self.length == 0 {
            return None;
        }

        let out = self.data[(self.head + (S - self.length) + 1) % S];
        self.length -= 1;
        out
    }

    pub fn peek_next<'a>(&'a self) -> &'a Option<T> {
        &self.data[self.head]
    }
}