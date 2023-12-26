use crate::at_most::AtMost;

mod at_most;
mod valid_err;

pub trait ValidatedIterator: Iterator {
    fn at_most(self, max_count: usize) -> AtMost<Self> where Self: Sized;
}
