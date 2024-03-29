use crate::{valid_iter::ValidIter, valid_result::ValidErr};

use super::valid_result::VResult;

#[derive(Debug, Clone)]
pub struct AtLeast<I>
where
    I: ValidIter + Iterator<Item = VResult<I::BaseType>>,
{
    iter: I,
    min_count: usize,
    counter: usize,
}

impl<I> AtLeast<I>
where
    I: Sized + ValidIter + Iterator<Item = VResult<I::BaseType>>,
{
    pub(crate) fn new(iter: I, min_count: usize) -> AtLeast<I> {
        AtLeast {
            iter,
            min_count,
            counter: 0,
        }
    }
}

impl<I> Iterator for AtLeast<I>
where
    I: ValidIter + Iterator<Item = VResult<I::BaseType>>,
{
    type Item = VResult<I::BaseType>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(Ok(val)) => {
                self.counter += 1;
                Some(Ok(val))
            }
            None => match self.counter >= self.min_count {
                true => None,
                false => {
                    self.counter = self.min_count;
                    Some(Err(ValidErr::TooFew { msg: None }))
                }
            },
            other => other,
        }
    }
}

impl<I> ValidIter for AtLeast<I>
where
    I: ValidIter + Iterator<Item = VResult<I::BaseType>>,
{
    type BaseType = I::BaseType;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::valid_iter::{Unvalidatable, ValidIter};

    #[test]
    fn test_at_least_on_failure() {
        assert_eq!((0..10).validate().at_least(100).count(), 11);
        (0..10)
            .validate()
            .at_least(100)
            .enumerate()
            .for_each(|(i, res_i)| match res_i {
                Ok(_) if i < 10 => {}
                Err(ValidErr::TooFew { .. }) if i == 10 => {}
                _ => panic!("unexpected value in at least adapter"),
            })
    }

    #[test]
    fn test_at_least_on_success() {
        assert_eq!((0..10).validate().at_least(5).count(), 10);
        (0..10)
            .validate()
            .at_least(5)
            .for_each(|res_i| match res_i {
                Ok(_) => {}
                _ => panic!("unexpected error in at least adapter"),
            })
    }

    #[test]
    fn test_at_least_successful_bounds() {
        let tightly_bound_success = (0..10)
            .validate()
            .at_least(10)
            .collect::<Result<Vec<_>, _>>();
        assert!(matches!(tightly_bound_success, Ok(_)));

        let empty_success = (0..0).validate().at_least(0).collect::<Result<Vec<_>, _>>();
        assert!(matches!(empty_success, Ok(_)));
    }

    #[test]
    fn test_at_least_unsuccessful_bounds() {
        let tightly_bound_failure = (0..10)
            .validate()
            .at_least(11)
            .collect::<Result<Vec<_>, _>>();
        assert!(matches!(tightly_bound_failure, Err(_)));

        let empty_failure = (0..0).validate().at_least(1).collect::<Result<Vec<_>, _>>();
        assert!(matches!(empty_failure, Err(_)));
    }

    #[test]
    fn test_at_least_all_elements_are_present_and_in_order_on_failure() {
        (0..10)
            .validate()
            .at_least(11)
            .enumerate()
            .for_each(|(i, res_i)| match res_i {
                Ok(int) if int == i as i32 && i < 10 => {}
                Err(ValidErr::TooFew { .. }) if i == 10 => {}
                _ => panic!("bad iteration after at least adapter failure"),
            })
    }

    #[test]
    fn test_at_least_all_elements_are_present_and_in_order_on_success() {
        (0..10)
            .validate()
            .at_least(10)
            .enumerate()
            .for_each(|(i, res_i)| match res_i {
                Ok(int) if int == i as i32 && i < 10 => {}
                _ => panic!("bad iteration after at least adapter success"),
            })
    }

    #[test]
    fn test_at_least_does_not_validate_on_short_circuiting_before_last_element() {
        (0..10)
            .validate()
            .at_least(100)
            .take(10)
            .for_each(|res_i| match res_i {
                Ok(_) => {}
                _ => panic!("failed the iteration when last error element was truncated"),
            })
    }

    #[test]
    fn test_at_least_validates_on_short_circuiting_after_last_element() {
        (0..10)
            .validate()
            .at_least(100)
            .take(11)
            .enumerate()
            .for_each(|(i, res_i)| {
                match res_i {
                    Ok(_) if i < 10 => {},
                    Err(ValidErr::TooFew{ .. }) if i == 10 => {}
                    _ => panic!("did not fail the iteration in short circuit when last error element was not truncated")
                }
            })
    }
}
