use crate::{valid_iter::ValidIter, valid_result::ValidErr};

use super::{valid_iter::ValidationSpaceAdapter, valid_result::VResult};

pub struct Between<I: ValidationSpaceAdapter>
where
    I::BaseType: PartialOrd,
{
    iter: I,
    lower_bound: I::BaseType,
    upper_bound: I::BaseType,
}

impl<I> Between<I>
where
    I: ValidationSpaceAdapter,
    I::BaseType: PartialOrd,
{
    pub fn new(iter: I, lower_bound: I::BaseType, upper_bound: I::BaseType) -> Between<I>
    where
        I: Sized,
    {
        Between {
            iter,
            lower_bound,
            upper_bound,
        }
    }
}

impl<I: ValidationSpaceAdapter> Iterator for Between<I>
where
    I: Iterator<Item = VResult<I::BaseType>>,
    I::BaseType: PartialOrd,
{
    type Item = VResult<I::BaseType>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(Ok(val)) => match self.lower_bound <= val && val < self.upper_bound {
                true => Some(Ok(val)),
                false => Some(Err(ValidErr::OutOfBounds(val))),
            },
            other => other,
        }
    }
}

impl<I: ValidationSpaceAdapter> ValidationSpaceAdapter for Between<I>
where
    I::BaseType: PartialOrd,
{
    type BaseType = I::BaseType;
}

impl<I: ValidationSpaceAdapter> ValidIter for Between<I>
where
    I::BaseType: PartialOrd,
{
    type BaseType = I::BaseType;
}
