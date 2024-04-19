use std::{iter::Enumerate, usize};

use crate::{valid_iter::ValidIter, ValidErr};

use super::valid_result::VResult;

#[derive(Debug, Clone)]
pub struct AtMost<I, Msg>
where
    I: ValidIter + Iterator<Item = VResult<I::BaseType>>,
    Msg: Fn(&I::BaseType, &usize, &usize) -> String,
{
    iter: Enumerate<I>,
    max_count: usize,
    msg_writer: Msg,
}

impl<I, Msg> AtMost<I, Msg>
where
    I: Sized + ValidIter + Iterator<Item = VResult<I::BaseType>>,
    Msg: Fn(&I::BaseType, &usize, &usize) -> String,
{
    pub(crate) fn new(iter: I, max_count: usize, err_msg: Msg) -> AtMost<I, Msg> {
        AtMost {
            iter: iter.enumerate(),
            max_count,
            msg_writer: err_msg,
        }
    }
}

impl<I, Msg> Iterator for AtMost<I, Msg>
where
    I: ValidIter + Iterator<Item = VResult<I::BaseType>>,
    Msg: Fn(&I::BaseType, &usize, &usize) -> String,
{
    type Item = VResult<I::BaseType>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some((i, Ok(val))) => match i < self.max_count {
                true => Some(Ok(val)),
                false => {
                    let msg = (self.msg_writer)(&val, &i, &self.max_count);
                    Some(Err(ValidErr::TooMany { element: val, msg }))
                }
            },
            Some((_, err)) => Some(err),
            None => None,
        }
    }
}

impl<I, Msg> ValidIter for AtMost<I, Msg>
where
    I: ValidIter + Iterator<Item = VResult<I::BaseType>>,
    Msg: Fn(&I::BaseType, &usize, &usize) -> String,
{
    type BaseType = I::BaseType;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::valid_iter::{Unvalidatable, ValidIter};

    #[test]
    fn test_at_most() {
        (1..10)
            .validate()
            .at_most(5, |elmt, i, max| {
                format!("err: elmt {}, i {}, max {}", elmt, i, max)
            })
            .for_each(|res_i| match res_i {
                Ok(i) => assert!(i < 6),
                Err(err_i) => match err_i {
                    ValidErr::TooMany { element, msg } => {
                        assert!(element >= 6);
                        assert_eq!(
                            msg,
                            format!("err: elmt {}, i {}, max 5", element, element - 1)
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
            .at_most(9, |_, _, _| "".to_string())
            .collect::<Result<Vec<_>, _>>();
        assert!(matches!(failed_collection, Err(ValidErr::TooMany { .. })));

        let collection = (0..10)
            .validate()
            .at_most(10, |_, _, _| "".to_string())
            .collect::<Result<Vec<_>, _>>();
        assert!(matches!(collection, Ok(_)));

        let empty_collection = (0..0)
            .validate()
            .at_most(0, |_, _, _| "".to_string())
            .collect::<Result<Vec<_>, _>>();
        assert!(matches!(empty_collection, Ok(_)));
    }

    #[test]
    fn test_at_most_all_elements_are_present_and_in_order() {
        (-10..0)
            .validate()
            .at_most(5, |elmt, i, max| {
                format!("err: elmt {}, i {}, max {}", elmt, i, max)
            })
            .enumerate()
            .for_each(|(i, res_i)| match i < 5 {
                true => match res_i {
                    Ok(int) if int == (i as i32 - 10) as i32 => {}
                    _ => panic!("bad match for item {}: {:?}", i, res_i),
                },
                false => match res_i {
                    Err(ValidErr::TooMany { element, msg })
                        if element == (i as i32 - 10) as i32 =>
                    {
                        print!("{}", msg);
                        assert_eq!(msg, format!("err: elmt {}, i {}, max 5", element, i))
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
            .at_most(3, |elmt, i, max| {
                format!("err: elmt {}, i {}, max {}", elmt, i, max)
            })
            .enumerate()
            .for_each(|(i, res_i)| match i < 3 {
                true => assert!(matches!(res_i, Ok(_))),
                false => {
                    let expected_msg = "err: elmt 3, i 3, max 3".to_string();
                    // assert!(matches!(res_i, Err(ValidErr::TooMany { element: &3, .. })));

                    match res_i {
                        Err(ValidErr::TooMany { element: &3, msg }) => {
                            assert_eq!(msg, expected_msg)
                        }
                        _ => panic!("bad match for at_most error"),
                    }
                }
            })
    }

    #[test]
    fn test_at_most_messaging() {
        #[derive(Debug, PartialEq)]
        struct Struct;
        let mut iter = [Struct, Struct]
            .iter()
            .validate()
            .at_most(0, |elmt, i, max| format!("{:?}-{}-{}", elmt, i, max));
        assert_eq!(
            iter.next(),
            Some(Err(ValidErr::TooMany {
                element: &Struct,
                msg: { "Struct-0-0".to_string() }
            }))
        );
        assert_eq!(
            iter.next(),
            Some(Err(ValidErr::TooMany {
                element: &Struct,
                msg: { "Struct-1-0".to_string() }
            }))
        );
    }
}
