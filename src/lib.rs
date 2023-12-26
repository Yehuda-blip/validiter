use at_least::AtLeast;
use between::Between;
use validate::Validate;

use crate::at_most::AtMost;

mod at_least;
mod at_most;
mod between;
mod valid_err;
mod validate;
mod validated_iterator;

pub trait ValidatedIterator: Iterator {
    fn at_most(self, max_count: usize) -> AtMost<Self>
    where
        Self: Sized;
    fn at_least(self, min_count: usize) -> AtLeast<Self>
    where
        Self: Sized;
    fn validate<F: FnMut(&Self::Item) -> bool>(self, validation: F) -> Validate<Self, F>
    where
        Self: Sized;
}

pub trait ValidatedOrderedIterator: ValidatedIterator
where
    Self: Sized,
    Self::Item: PartialOrd,
{
    fn between(self, lower_bound: Self::Item, upper_bound: Self::Item) -> Between<Self>;
}
