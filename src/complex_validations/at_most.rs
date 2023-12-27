use super::{
    valid_iter::{ValidIter, ValidationSpaceAdapter},
    valid_result::ValidResult,
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

impl<I: ValidationSpaceAdapter> ValidationSpaceAdapter for AtMost<I> {
    type BaseType = I::BaseType;
    // type Transformed = I;
}

impl<I: ValidationSpaceAdapter> Iterator for AtMost<I>
where
    I: Iterator<Item = ValidResult<I::BaseType>>,
{
    type Item = ValidResult<I::BaseType>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(ValidResult::Ok(val)) => {
                Some(ValidResult::Ok(val))
            },
            other => other
        }
    }
}

impl<I: ValidationSpaceAdapter> ValidIter for AtMost<I> {
    type BaseType = I::BaseType;
}
