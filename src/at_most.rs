use crate::{valid_result::ValidErr, valid_iter::ValidIter};

use super::{
    valid_iter::ValidationSpaceAdapter,
    valid_result::VResult,
};

pub struct AtMost<I: ValidationSpaceAdapter> {
    iter: I,
    max_count: usize,
    counter: usize,
}

impl<I> AtMost<I>
where
    I: ValidationSpaceAdapter,
{
    pub fn new(iter: I, max_count: usize) -> AtMost<I>
    where
        I: Sized,
    {
        AtMost {
            iter,
            max_count,
            counter: 0,
        }
    }
}

impl<I: ValidationSpaceAdapter> Iterator for AtMost<I>
where
    I: Iterator<Item = VResult<I::BaseType>>,
{
    type Item = VResult<I::BaseType>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(Ok(val)) => {
                match self.counter >= self.max_count {
                    true => Some(Err(ValidErr::TooMany(val))),
                    false => Some(Ok(val))
                }
            },
            other => other
        }
    }
}

impl<I: ValidationSpaceAdapter> ValidationSpaceAdapter for AtMost<I> {
    type BaseType = I::BaseType;
}

impl<I: ValidationSpaceAdapter> ValidIter for AtMost<I> {
    type BaseType = I::BaseType;
}
