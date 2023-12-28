use crate::{valid_iter::ValidIter, valid_result::ValidErr};

use super::{valid_iter::ValidationSpaceAdapter, valid_result::VResult};

pub struct Ensure<I: ValidationSpaceAdapter, F: FnMut(&I::BaseType) -> bool> {
    iter: I,
    validation: F,
}

impl<I, F> Ensure<I, F>
where
    I: ValidationSpaceAdapter,
    F: FnMut(&I::BaseType) -> bool,
{
    pub fn new(iter: I, validation: F) -> Ensure<I, F>
    where
        I: Sized,
    {
        Ensure { iter, validation }
    }
}

impl<I: ValidationSpaceAdapter, F> Iterator for Ensure<I, F>
where
    I: Iterator<Item = VResult<I::BaseType>>,
    F: FnMut(&I::BaseType) -> bool,
{
    type Item = VResult<I::BaseType>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(Ok(val)) => match (self.validation)(&val) {
                true => Some(Ok(val)),
                false => Some(Err(ValidErr::Invalid(val))),
            },
            other => other,
        }
    }
}

impl<I: ValidationSpaceAdapter, F> ValidationSpaceAdapter for Ensure<I, F>
where
    F: FnMut(&I::BaseType) -> bool,
{
    type BaseType = I::BaseType;
}

impl<I: ValidationSpaceAdapter, F> ValidIter for Ensure<I, F>
where
    F: FnMut(&I::BaseType) -> bool,
{
    type BaseType = I::BaseType;
}
