use crate::{valid_iter::ValidIter, valid_result::ValidErr};

use super::valid_result::VResult;

#[macro_export]
macro_rules! too_few {
    () => {
        |count, min_count| format!("Too Few error: {count} elements were found in an iteration with a minimum count of {min_count}")
    };
}

#[derive(Debug, Clone)]
pub struct AtLeast<I, Msg>
where
    I: ValidIter + Iterator<Item = VResult<I::BaseType>>,
    Msg: Fn(&usize, &usize) -> String,
{
    iter: I,
    min_count: usize,
    counter: usize,
    msg_writer: Msg,
}

impl<I, Msg> AtLeast<I, Msg>
where
    I: Sized + ValidIter + Iterator<Item = VResult<I::BaseType>>,
    Msg: Fn(&usize, &usize) -> String,
{
    pub(crate) fn new(iter: I, min_count: usize, err_msg: Msg) -> AtLeast<I, Msg> {
        AtLeast {
            iter,
            min_count,
            counter: 0,
            msg_writer: err_msg,
        }
    }
}

impl<I, Msg> Iterator for AtLeast<I, Msg>
where
    I: ValidIter + Iterator<Item = VResult<I::BaseType>>,
    Msg: Fn(&usize, &usize) -> String,
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
                    let msg = (self.msg_writer)(&self.counter, &self.min_count);
                    self.counter = self.min_count;
                    Some(Err(ValidErr::TooFew(msg)))
                }
            },
            other => other,
        }
    }
}

impl<I, Msg> ValidIter for AtLeast<I, Msg>
where
    I: ValidIter + Iterator<Item = VResult<I::BaseType>>,
    Msg: Fn(&usize, &usize) -> String,
{
    type BaseType = I::BaseType;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::valid_iter::{Unvalidatable, ValidIter};

    #[test]
    fn test_at_least_on_failure() {
        assert_eq!(
            (0..10)
                .validate()
                .at_least(100, |count, min| format!("{count}-{min}"))
                .count(),
            11
        );
        (0..10)
            .validate()
            .at_least(100, |count, min| format!("{count}-{min}"))
            .enumerate()
            .for_each(|(i, res_i)| match res_i {
                Ok(_) if i < 10 => {}
                Err(ValidErr::TooFew(msg)) if i == 10 => assert_eq!(msg, "10-100"),
                _ => panic!("unexpected value in at least adapter"),
            })
    }

    #[test]
    fn test_at_least_on_success() {
        assert_eq!((0..10).validate().at_least(5, |_, _| "".into()).count(), 10);
        (0..10)
            .validate()
            .at_least(5, |_, _| "".into())
            .for_each(|res_i| match res_i {
                Ok(_) => {}
                _ => panic!("unexpected error in at least adapter"),
            })
    }

    #[test]
    fn test_at_least_successful_bounds() {
        let tightly_bound_success = (0..10)
            .validate()
            .at_least(10, |_, _| "".into())
            .collect::<Result<Vec<_>, _>>();
        assert!(matches!(tightly_bound_success, Ok(_)));

        let empty_success = (0..0)
            .validate()
            .at_least(0, |_, _| "".into())
            .collect::<Result<Vec<_>, _>>();
        assert!(matches!(empty_success, Ok(_)));
    }

    #[test]
    fn test_at_least_unsuccessful_bounds() {
        let tightly_bound_failure = (0..10)
            .validate()
            .at_least(11, |_, _| "".into())
            .collect::<Result<Vec<_>, _>>();
        assert!(matches!(tightly_bound_failure, Err(_)));

        let empty_failure = (0..0)
            .validate()
            .at_least(1, |_, _| "".into())
            .collect::<Result<Vec<_>, _>>();
        assert!(matches!(empty_failure, Err(_)));
    }

    #[test]
    fn test_at_least_all_elements_are_present_and_in_order_on_failure() {
        (0..10)
            .validate()
            .at_least(11, |count, min| format!("{count}-{min}"))
            .enumerate()
            .for_each(|(i, res_i)| match res_i {
                Ok(int) if int == i as i32 && i < 10 => {}
                Err(ValidErr::TooFew(msg)) if i == 10 => assert_eq!(msg, "10-11"),
                _ => panic!("bad iteration after at least adapter failure"),
            })
    }

    #[test]
    fn test_at_least_all_elements_are_present_and_in_order_on_success() {
        (0..10)
            .validate()
            .at_least(10, |_, _| "".into())
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
            .at_least(100, |_, _| "".into())
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
            .at_least(100, |count, min| format!("{count}-{min}"))
            .take(11)
            .enumerate()
            .for_each(|(i, res_i)| {
                match res_i {
                    Ok(_) if i < 10 => {},
                    Err(ValidErr::TooFew(msg)) if i == 10 => assert_eq!(msg, "10-100"),
                    _ => panic!("did not fail the iteration in short circuit when last error element was not truncated")
                }
            })
    }

    fn test_at_least_macro() {
        let result = (0..0).validate().at_least(1, too_few!()).next();
        match result {
            Some(Err(ValidErr::TooFew(msg))) => {
                assert_eq!(msg, "Too Few error: 0 elements were found in an iteration with a minimum count of 1")
            },
            _ => {panic!("bad value out of at_least adapter")}
        }
    }
}
