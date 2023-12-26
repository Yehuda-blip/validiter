use std::iter::Enumerate;

use crate::valid_err::ValidErr;

pub struct AtMost<I: Iterator> {
    iter: Enumerate<I>,
    max_count: usize,
}

impl<I: Iterator> AtMost<I> {
    pub(crate) fn new(iter: I, max_count: usize) -> AtMost<I> {
        AtMost {
            iter: iter.enumerate(),
            max_count,
        }
    }
}

impl<I: Iterator> Iterator for AtMost<I> {
    type Item = Result<I::Item, ValidErr<I>>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some((i, val)) => match i < self.max_count {
                true => Some(Ok(val)),
                false => Some(Err(ValidErr::TooMany(val))),
            },
            None => None,
        }
    }
}
