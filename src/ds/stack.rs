use crate::prelude::*;

pub struct Stack<T, const S: usize> {
    data: [Option<T>; S],
    top: usize,
}

impl<T: Copy, const S: usize> Default for Stack<T, S> {
    fn default() -> Self {
        Self { 
            data: [None; S], 
            top: Default::default() ,
        }
    }
}


impl<T: Copy, const S: usize> Stack<T, S> {
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

    
    pub fn pop(&mut self) -> Option<T> {
        if self.top == 0 {
            None
        } else {
            self.top -= 1;
            let item = self.data[self.top];
            item
        }
    }

    
}

impl<T, const S: usize> Stack<T, S> {
    pub fn push(&mut self, item: T) -> Result<()> {
        if self.top == S {
            return Err(BikeError::StackFull);
        }

        self.data[self.top] = Some(item);
        self.top += 1;

        Ok(())
    }

    
    pub fn peek<'a>(&'a mut self) -> Option<&'a T> {
        if self.top == 0 {
            return None;
        }
        
        self.data[self.top - 1].as_ref()
    }

}
