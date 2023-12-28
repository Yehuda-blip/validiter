use super::{valid_iter::{ValidationSpaceAdapter, ValidIter}, valid_result::VResult};

pub struct Validatable<I: Iterator> {
    pub(crate) iter: I,
}

impl<I> ValidationSpaceAdapter for Validatable<I>
where
    I: Iterator,
{
    type BaseType = I::Item;
}

impl<I: Iterator> Iterator for Validatable<I> {
    type Item = VResult<I::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(val) => Some(Ok(val)),
            None => None
        }
    }
}

impl<I: Iterator> ValidIter for Validatable<I> {
    type BaseType = I::Item;

    // type InnerIter;
}
