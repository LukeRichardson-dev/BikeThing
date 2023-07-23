use crate::prelude::*;

pub struct Vec<T, const S: usize> {
    space: [Option<T>; S],
    length: usize,
}

impl<T: Copy, const S: usize> Vec<T, S> {
    pub fn new<const A: usize>(data: [T; A]) -> Self {
        let mut space = [None; S];

        for (idx, item) in data.into_iter().enumerate() {
            space[idx] = Some(item);
        }

        Self { space, length: A }
    }

    pub fn append(&mut self, data: &[T]) -> Result<()> {
        if data.len() + self.length == 0 {
            return Err(BikeError::VecFull);
        }
        
        for i in 0..data.len() {
            self.space[i + self.length] = Some(data[i]);
        }
        self.length += data.len();

        Ok(())
    }
}