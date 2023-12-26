use crate::{at_least::AtLeast, at_most::AtMost, ValidatedIterator};

impl<I> ValidatedIterator for I
where
    I: Iterator,
{
    fn at_most(self, max_count: usize) -> AtMost<Self> {
        AtMost::new(self, max_count)
    }

    fn at_least(self, min_count: usize) -> AtLeast<Self> {
        AtLeast::new(self, min_count)
    }
}

#[cfg(test)]
mod tests {
    use crate::valid_err::ValidErr;

    use super::*;

    //// at most tests start ////
    #[test]
    fn test_at_most_is_ok_under_bound() {
        let collection: Result<Vec<i32>, _> = (0..10).at_most(10).collect();
        assert!(matches!(collection, Ok(_)))
    }

    #[test]
    fn test_at_most_errs_on_too_many() {
        let collection: Result<Vec<i32>, _> = (0..10).at_most(9).collect();
        assert!(matches!(collection, Err(ValidErr::TooMany(_))))
    }

    #[test]
    fn test_at_most_all_elements_present_and_in_order() {
        let validated_collection = (0..10)
            .at_most(10)
            .collect::<Result<Vec<i32>, _>>()
            .expect("could not collect the validated vector");
        let unvalidated_collection = (0..10).collect::<Vec<i32>>();
        assert_eq!(validated_collection, unvalidated_collection);
    }

    #[test]
    fn test_at_most_nth_is_err_when_overflowing() {
        let first_overflow = (0..10).at_most(9).nth(9).unwrap();
        assert!(matches!(first_overflow, Err(ValidErr::TooMany(9))))
    }

    #[test]
    fn test_at_most_must_be_empty_on_max_count_is_0() {
        let first_overflow = (0..10).at_most(0).next().unwrap();
        assert!(matches!(first_overflow, Err(ValidErr::TooMany(0))))
    }
    //// at most tests end ////

    //// at least tests start ////
    #[test]
    fn test_at_least_is_ok_over_bound() {
        if (0..10).at_least(10).any(|element| element.is_err()) {
            panic!("got validation err when count is under min bound")
        }
    }

    #[test]
    fn test_at_least_adds_err_when_stopping_before_bound() {
        if !(0..10).at_least(11).any(|element| element.is_err()) {
            panic!("did not get validation err when count is tightly under min bound")
        }

        if !(0..10).at_least(100).any(|element| element.is_err()) {
            panic!("did not get validation err when count is untightly over min bound")
        }
    }

    #[test]
    fn test_at_least_adds_err_on_too_few_elements_and_stops() {
        assert_eq!((0..10).at_least(100).count(), 11)
    }

    #[test]
    fn test_at_least_yields_ok_on_too_few_elements() {
        if (0..10)
            .at_least(100)
            .take(10)
            .any(|element| element.is_err())
        {
            panic!("got err when iterating over existing elements in iterator")
        }
    }
    //// at least tests end ////
}
