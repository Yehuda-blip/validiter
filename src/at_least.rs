use std::rc::Rc;

use crate::{valid_iter::ValidIter, valid_result::ValidErr};

use super::valid_result::VResult;


/// The [`Atleast`] ValidIter adapter, for more info see [`at_least`](crate::ValidIter::at_least).
#[derive(Debug, Clone)]
pub struct AtLeast<I>
where
    I: ValidIter + Iterator<Item = VResult<I::BaseType>>,
{
    iter: I,
    min_count: usize,
    counter: usize,
    desc: Rc<str>,
}

impl<I> AtLeast<I>
where
    I: Sized + ValidIter + Iterator<Item = VResult<I::BaseType>>,
{
    pub(crate) fn new(iter: I, min_count: usize, desc: &str) -> AtLeast<I> {
        AtLeast {
            iter,
            min_count,
            counter: 0,
            desc: Rc::from(desc),
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
                    Some(Err(ValidErr::Description(Rc::clone(&self.desc))))
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
        assert_eq!((0..10).validate().at_least(100, "test").count(), 11);
        (0..10)
            .validate()
            .at_least(100, "test")
            .enumerate()
            .for_each(|(i, res_i)| match res_i {
                Ok(_) if i < 10 => {}
                Err(ValidErr::Description(msg)) if i == 10 => {
                    assert_eq!(msg, Rc::from("test"))
                }
                _ => panic!("unexpected value in at least adapter"),
            })
    }

    #[test]
    fn test_at_least_on_success() {
        assert_eq!((0..10).validate().at_least(5, "test").count(), 10);
        (0..10)
            .validate()
            .at_least(5, "test")
            .for_each(|res_i| match res_i {
                Ok(_) => {}
                _ => panic!("unexpected error in at least adapter"),
            })
    }

    #[test]
    fn test_at_least_successful_bounds() {
        let tightly_bound_success = (0..10)
            .validate()
            .at_least(10, "test")
            .collect::<Result<Vec<_>, _>>();
        assert!(matches!(tightly_bound_success, Ok(_)));

        let empty_success = (0..0)
            .validate()
            .at_least(0, "test")
            .collect::<Result<Vec<_>, _>>();
        assert!(matches!(empty_success, Ok(_)));
    }

    #[test]
    fn test_at_least_unsuccessful_bounds() {
        let tightly_bound_failure = (0..10)
            .validate()
            .at_least(11, "test")
            .collect::<Result<Vec<_>, _>>();
        match tightly_bound_failure {
            Ok(_) => panic!("collection should fail"),
            Err(ValidErr::Description(msg)) => assert_eq!(msg, Rc::from("test")),
            _ => panic!("bad variant"),
        }

        let empty_failure = (0..0)
            .validate()
            .at_least(1, "test")
            .collect::<Result<Vec<_>, _>>();
        match empty_failure {
            Ok(_) => panic!("collection should fail"),
            Err(ValidErr::Description(msg)) => assert_eq!(msg, Rc::from("test")),
            _ => panic!("bad variant"),
        }
    }

    #[test]
    fn test_at_least_all_elements_are_present_and_in_order_on_failure() {
        (0..10)
            .validate()
            .at_least(11, "test")
            .enumerate()
            .for_each(|(i, res_i)| match res_i {
                Ok(int) if int == i as i32 && i < 10 => {}
                Err(ValidErr::Description(msg)) if i == 10 => {
                    assert_eq!(msg, Rc::from("test"))
                }
                _ => panic!("bad iteration after at least adapter failure"),
            })
    }

    #[test]
    fn test_at_least_all_elements_are_present_and_in_order_on_success() {
        (0..10)
            .validate()
            .at_least(10, "test")
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
            .at_least(100, "test")
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
            .at_least(100, "test")
            .take(11)
            .enumerate()
            .for_each(|(i, res_i)| {
                match res_i {
                    Ok(_) if i < 10 => {},
                    Err(ValidErr::Description(msg)) if i == 10 => {assert_eq!(msg, Rc::from("test"))}
                    _ => panic!("did not fail the iteration in short circuit when last error element was not truncated")
                }
            })
    }

    #[test]
    fn test_at_least_counting_iterator_correctly_skips_errors() {
        let results = (0..1)
            .validate()
            .ensure(|i| i % 2 == 1, "ensure")
            .at_least(1, "at-least")
            .collect::<Vec<_>>();
        assert_eq!(
            results,
            vec![
                Err(ValidErr::WithElement(0, Rc::from("ensure"))),
                Err(ValidErr::Description(Rc::from("at-least")))
            ]
        )
    }
}
