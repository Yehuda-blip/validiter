use super::{valid_iter::ValidIter, valid_result::VResult};

/// The trait defining a validatable [`Iterator`]. For more information, see [`validate`](crate::Unvalidatable::validate)
#[derive(Debug, Clone)]
pub struct Validatable<I: Iterator> {
    iter: I,
}

impl<I: Iterator> Validatable<I> {
    pub(crate) fn new(iter: I) -> Validatable<I> {
        Self { iter }
    }
}

impl<I: Iterator> Iterator for Validatable<I> {
    type Item = VResult<I::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(val) => Some(Ok(val)),
            None => None,
        }
    }
}

impl<I: Iterator> ValidIter for Validatable<I> {
    type BaseType = I::Item;
}
