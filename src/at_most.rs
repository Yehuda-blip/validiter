use std::iter::Enumerate;

use crate::valid_err::ValidErr;
use crate::ValidatedIterator;

pub struct AtMost<I: Iterator> {
    iter: Enumerate<I>,
    max_count: usize,
}

impl<I: Iterator> AtMost<I> {
    fn new(iter: I, max_count: usize) -> AtMost<I> {
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

impl<I> ValidatedIterator for I
where
    I: Iterator,
{
    fn at_most(self, max_count: usize) -> AtMost<Self> {
        AtMost::new(self, max_count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ok_on_under_bound() {
        let collection: Result<Vec<i32>, _> = (0..10).at_most(10).collect();
        assert!(matches!(collection, Ok(_)))
    }

    #[test]
    fn test_err_on_too_many() {
        let collection: Result<Vec<i32>, _> = (0..10).at_most(9).collect();
        assert!(matches!(collection, Err(ValidErr::TooMany(_))))
    }

    #[test]
    fn test_all_elements_present_and_in_order() {
        let validated_collection = (0..10)
            .at_most(10)
            .collect::<Result<Vec<i32>, _>>()
            .expect("could not collect the validated vector");
        let unvalidated_collection = (0..10).collect::<Vec<i32>>();
        assert_eq!(validated_collection, unvalidated_collection);
    }

    #[test]
    fn test_nth_is_err_when_overflowing() {
        let first_overflow = (0..10).at_most(9).nth(9).unwrap();
        assert!(matches!(first_overflow, Err(ValidErr::TooMany(9))))
    }

    #[test]
    fn test_must_be_empty_on_max_count_is_0() {
        let first_overflow = (0..10).at_most(0).next().unwrap();
        assert!(matches!(first_overflow, Err(ValidErr::TooMany(0))))
    }
}
