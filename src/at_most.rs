use crate::{valid_iter::ValidIter, ValidErr};

use super::valid_result::VResult;

#[macro_export]
macro_rules! too_many {
    ($description:literal plus_auto) => {
        |elmt, max_count| $description.to_string() + &format!("iteration got '{elmt}' after exceeding the {max_count} elements cap")
    };
    ($description:literal plus_auto_debug) => {
        |elmt, max_count| $description.to_string() + &format!("iteration got '{elmt:?}' after exceeding the {max_count:?} elements cap")
    };
    ($description:literal) => {
        |_, _| $description.to_string()
    };
}

#[derive(Debug, Clone)]
/// This struct is created by the [`at_most`](crate::ValidIter::at_most) method on [`ValidIter`]. See its documetation for more.
pub struct AtMost<I, Msg>
where
    I: ValidIter + Iterator<Item = VResult<I::BaseType>>,
    Msg: Fn(&I::BaseType, &usize) -> String,
{
    iter: I,
    position: usize,
    max_count: usize,
    msg_writer: Msg,
}

impl<I, Msg> AtMost<I, Msg>
where
    I: Sized + ValidIter + Iterator<Item = VResult<I::BaseType>>,
    Msg: Fn(&I::BaseType, &usize) -> String,
{
    pub(crate) fn new(iter: I, max_count: usize, err_msg: Msg) -> AtMost<I, Msg> {
        AtMost {
            iter,
            position: 0,
            max_count,
            msg_writer: err_msg,
        }
    }
}

impl<I, Msg> Iterator for AtMost<I, Msg>
where
    I: ValidIter + Iterator<Item = VResult<I::BaseType>>,
    Msg: Fn(&I::BaseType, &usize) -> String,
{
    type Item = VResult<I::BaseType>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(Ok(val)) => match self.position < self.max_count {
                true => {
                    self.position += 1;
                    Some(Ok(val))
                }
                false => {
                    let msg = (self.msg_writer)(&val, &self.max_count);
                    Some(Err(ValidErr::TooMany(val, msg)))
                }
            },
            Some(err) => Some(err),
            None => None,
        }
    }
}

impl<I, Msg> ValidIter for AtMost<I, Msg>
where
    I: ValidIter + Iterator<Item = VResult<I::BaseType>>,
    Msg: Fn(&I::BaseType, &usize) -> String,
{
    type BaseType = I::BaseType;
}

#[cfg(test)]
mod tests {
    use std::fmt::Display;

    use super::*;
    use crate::{out_of_bounds, valid_iter::{Unvalidatable, ValidIter}};

    #[test]
    fn test_at_most() {
        (1..10)
            .validate()
            .at_most(5, |elmt, max| {
                format!("err: elmt {}, max {}", elmt, max)
            })
            .for_each(|res_i| match res_i {
                Ok(i) => assert!(i < 6),
                Err(err_i) => match err_i {
                    ValidErr::TooMany(element, msg) => {
                        assert!(element >= 6);
                        assert_eq!(
                            msg,
                            format!("err: elmt {}, max 5", element)
                        )
                    }
                    _ => panic!("incorrect err for at most validator"),
                },
            })
    }

    #[test]
    fn test_at_most_has_correct_bounds() {
        let failed_collection = (0..10)
            .validate()
            .at_most(9, |_, _| "".to_string())
            .collect::<Result<Vec<_>, _>>();
        assert!(matches!(failed_collection, Err(ValidErr::TooMany { .. })));

        let collection = (0..10)
            .validate()
            .at_most(10, |_, _| "".to_string())
            .collect::<Result<Vec<_>, _>>();
        assert!(matches!(collection, Ok(_)));

        let empty_collection = (0..0)
            .validate()
            .at_most(0, |_, _| "".to_string())
            .collect::<Result<Vec<_>, _>>();
        assert!(matches!(empty_collection, Ok(_)));
    }

    #[test]
    fn test_at_most_all_elements_are_present_and_in_order() {
        (-10..0)
            .validate()
            .at_most(5, |elmt, max| {
                format!("err: elmt {}, max {}", elmt, max)
            })
            .enumerate()
            .for_each(|(i, res_i)| match i < 5 {
                true => match res_i {
                    Ok(int) if int == (i as i32 - 10) as i32 => {}
                    _ => panic!("bad match for item {}: {:?}", i, res_i),
                },
                false => match res_i {
                    Err(ValidErr::TooMany(element, msg)) if element == (i as i32 - 10) as i32 => {
                        print!("{}", msg);
                        assert_eq!(msg, format!("err: elmt {}, max 5", element))
                    }
                    _ => panic!("bad match for item {}: {:?}", i, res_i),
                },
            })
    }

    #[test]
    fn test_at_most_by_ref() {
        [0, 1, 2, 3]
            .iter()
            .validate()
            .at_most(3, |elmt, max| {
                format!("err: elmt {}, max {}", elmt, max)
            })
            .enumerate()
            .for_each(|(i, res_i)| match i < 3 {
                true => assert!(matches!(res_i, Ok(_))),
                false => {
                    let expected_msg = "err: elmt 3, max 3".to_string();
                    // assert!(matches!(res_i, Err(ValidErr::TooMany { element: &3, .. })));

                    match res_i {
                        Err(ValidErr::TooMany(&3, msg)) => {
                            assert_eq!(msg, expected_msg)
                        }
                        _ => panic!("bad match for at_most error"),
                    }
                }
            })
    }

    #[derive(Debug, PartialEq)]
    struct Struct;

    impl Display for Struct {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}-display", self)
        }
    }

    #[test]
    fn test_at_most_messaging() {
        let mut iter = [Struct, Struct]
            .iter()
            .validate()
            .at_most(0, |elmt, max| format!("{:?}-{}", elmt, max));
        assert_eq!(
            iter.next(),
            Some(Err(ValidErr::TooMany(&Struct, "Struct-0".to_string())))
        );
        assert_eq!(
            iter.next(),
            Some(Err(ValidErr::TooMany(&Struct, "Struct-0".to_string())))
        );
        assert_eq!(iter.next(), None)
    }

    #[test]
    fn test_too_many_macro_just_user_input() {
        let mut iter = [Struct].iter().validate().at_most(0, too_many!("test"));
        match iter.next() {
            Some(Err(ValidErr::TooMany(_, msg))) => {
                assert_eq!(msg, "test")
            }
            _ => panic!("too many error not detected"),
        }
    }

    #[test]
    fn test_too_many_macro_auto() {
        let mut iter = [Struct]
            .iter()
            .validate()
            .at_most(0, too_many!("test" plus_auto));
        match iter.next() {
            Some(Err(ValidErr::TooMany(_, msg))) => {
                assert_eq!(msg, "testiteration got 'Struct-display' after exceeding the 0 elements cap")
            }
            _ => panic!("too many error not detected"),
        }
    }

    #[test]
    fn test_too_many_macro_auto_debug() {
        let mut iter = [Struct]
            .iter()
            .validate()
            .at_most(0, too_many!("test" plus_auto_debug));
        match iter.next() {
            Some(Err(ValidErr::TooMany(_, msg))) => {
                assert_eq!(msg, "testiteration got 'Struct' after exceeding the 0 elements cap")
            }
            _ => panic!("too many error not detected"),
        }
    }

    #[test]
    fn test_too_many_macro_debug_display_equivalent() {
        let disp_iter = [Struct]
            .iter()
            .validate()
            .at_most(0, too_many!("" plus_auto));
        let debug_iter = [Struct]
            .iter()
            .validate()
            .at_most(0, too_many!("" plus_auto_debug));
        match disp_iter.zip(debug_iter).next() {
            Some((Err(ValidErr::TooMany(_, disp_msg)), Err(ValidErr::TooMany(_, debug_msg)))) => {
                assert_eq!(disp_msg.replace("-display", ""), debug_msg)
            }
            _ => panic!("too many error not detected"),
        }
    }

    #[test]
    fn test_failure_in_doctests_out_of_bounds_not_ignored() {
        let mut iter = (-1..=3)
            .validate()
            .between(0, 10, out_of_bounds!("element is out of bounds"))
            .at_most(4, too_many!("limit exceeded"));
        
        assert_eq!(iter.next(), Some(Err(ValidErr::OutOfBounds(-1, "element is out of bounds".to_string()))));
        assert_eq!(iter.next(), Some(Ok(0)));
        assert_eq!(iter.next(), Some(Ok(1)));
        assert_eq!(iter.next(), Some(Ok(2)));
        assert_eq!(iter.next(), Some(Ok(3)));
    }
}
