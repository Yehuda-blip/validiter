use crate::valid_result::ValidErr;

use super::{
    valid_iter::ValidationSpaceAdapter,
    valid_result::VResult,
};

pub struct AtLeast<I: ValidationSpaceAdapter> {
    iter: I,
    min_count: usize,
    counter: usize,
}

impl<I> AtLeast<I>
where
    I: ValidationSpaceAdapter,
{
    pub fn new(iter: I, min_count: usize) -> AtLeast<I>
    where
        I: Sized,
    {
        AtLeast {
            iter,
            min_count,
            counter: 0,
        }
    }
}

impl<I: ValidationSpaceAdapter> Iterator for AtLeast<I>
where
    I: Iterator<Item = VResult<I::BaseType>>,
{
    type Item = VResult<I::BaseType>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(Ok(val)) => {
                self.counter += 1;
                Some(Ok(val))
            },
            None => {
                match self.counter >= self.min_count {
                    true => None,
                    false => {
                        self.counter = self.min_count;
                        Some(Err(ValidErr::TooFew))
                    }
                }
            }
            other => other
        }
    }
}

impl<I: ValidationSpaceAdapter> ValidationSpaceAdapter for AtLeast<I> {
    type BaseType = I::BaseType;
}
