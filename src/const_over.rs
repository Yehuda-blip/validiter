use crate::{
    valid_iter::ValidIter,
    valid_result::{VResult, ValidErr},
};

#[macro_export]
macro_rules! broken_const {
    ($description:literal plus_auto) => {
        |elmt, ext, cst| $description.to_string() + &format!("found '{elmt}' which evaluates to {ext} in an iteration where elements must evaluate to {cst}")
    };
    ($description:literal plus_auto_debug) => {
        |elmt, ext, cst| $description.to_string() + &format!("found '{elmt:?}' which evaluates to {ext:?} in an iteration where elements must evaluate to {cst:?}")
    };
    ($description:literal) => {
        |_, _, _| $description.to_string()
    };
}

#[derive(Debug, Clone)]
pub struct ConstOver<I, A, M, Msg>
where
    I: ValidIter + Iterator<Item = VResult<I::BaseType>>,
    A: PartialEq,
    M: FnMut(&I::BaseType) -> A,
    Msg: Fn(&I::BaseType, &A, &A) -> String,
{
    iter: I,
    stored_value: Option<A>,
    extractor: M,
    msg_writer: Msg,
}

impl<I, A, M, Msg> ConstOver<I, A, M, Msg>
where
    I: ValidIter + Iterator<Item = VResult<I::BaseType>>,
    A: PartialEq,
    M: FnMut(&I::BaseType) -> A,
    Msg: Fn(&I::BaseType, &A, &A) -> String,
{
    pub(crate) fn new(iter: I, extractor: M, err_msg: Msg) -> ConstOver<I, A, M, Msg> {
        Self {
            iter,
            stored_value: None,
            extractor,
            msg_writer: err_msg,
        }
    }
}

impl<I, A, M, Msg> Iterator for ConstOver<I, A, M, Msg>
where
    I: ValidIter + Iterator<Item = VResult<I::BaseType>>,
    A: PartialEq,
    M: FnMut(&I::BaseType) -> A,
    Msg: Fn(&I::BaseType, &A, &A) -> String,
{
    type Item = VResult<I::BaseType>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(Ok(val)) => match &self.stored_value {
                Some(expected_const) => {
                    let extraction = (self.extractor)(&val);
                    match extraction == *expected_const {
                        true => Some(Ok(val)),
                        false => {
                            let msg = (self.msg_writer)(&val, &extraction, expected_const);
                            Some(Err(ValidErr::BrokenConstant(val, msg)))
                        }
                    }
                }
                None => {
                    self.stored_value = Some((self.extractor)(&val));
                    Some(Ok(val))
                }
            },
            other => other,
        }
    }
}

impl<I, A, M, Msg> ValidIter for ConstOver<I, A, M, Msg>
where
    I: ValidIter + Iterator<Item = VResult<I::BaseType>>,
    A: PartialEq,
    M: FnMut(&I::BaseType) -> A,
    Msg: Fn(&I::BaseType, &A, &A) -> String,
{
    type BaseType = I::BaseType;
}

#[cfg(test)]
mod tests {
    use std::iter::repeat;

    use crate::{
        valid_iter::{Unvalidatable, ValidIter},
        valid_result::ValidErr,
    };

    #[test]
    fn test_const_over_ok() {
        if repeat(1)
            .take(5)
            .validate()
            .const_over(|i| *i, |_, _, _| "".to_string())
            .any(|res| res.is_err())
        {
            panic!("const over failed on constant iteration")
        }
    }

    #[test]
    fn test_const_over_err() {
        let results: Vec<_> = [0, 0, 0, 1]
            .into_iter()
            .validate()
            .const_over(
                |i| *i + 2,
                |elmt, extract, constant| format!("{elmt}-{extract}-{constant}"),
            )
            .collect();
        assert_eq!(
            results,
            [
                Ok(0),
                Ok(0),
                Ok(0),
                Err(ValidErr::BrokenConstant(1, "1-3-2".to_string()))
            ]
        )
    }

    #[test]
    fn test_const_over_bounds() {
        if (0..0)
            .validate()
            .const_over(|i| *i, |_, _, _| "".to_string())
            .any(|res| res.is_err())
        {
            panic!("const over failed on empty iter")
        }

        if (0..1)
            .validate()
            .const_over(|i| *i, |_, _, _| "".to_string())
            .any(|res| res.is_err())
        {
            panic!("const over failed on count == 1 iter")
        }
    }

    #[test]
    fn test_const_over_all_elements_are_present_and_in_order() {
        let results: Vec<_> = [[0], [0], [0], [1], [0], [2]]
            .into_iter()
            .validate()
            .const_over(
                |slice| slice[0],
                |elmt, extract, constant| format!("{elmt:?}-{extract}-{constant}"),
            )
            .collect();
        assert_eq!(
            results,
            [
                Ok([0]),
                Ok([0]),
                Ok([0]),
                Err(ValidErr::BrokenConstant([1], "[1]-1-0".to_string())),
                Ok([0]),
                Err(ValidErr::BrokenConstant([2], "[2]-2-0".to_string()))
            ]
        )
    }

    #[test]
    fn test_broken_const_macro_user_input_only() {
        match (0..=1)
            .validate()
            .const_over(|e| *e + 2, broken_const!("test"))
            .nth(1)
        {
            Some(Err(ValidErr::BrokenConstant(1, msg))) => {
                assert_eq!(msg, "test")
            }
            _ => panic!("bad value in const_over"),
        }
    }

    #[test]
    fn test_broken_const_macro_auto() {
        match (0..=1)
            .validate()
            .const_over(|e| *e + 2, broken_const!("test" plus_auto))
            .nth(1)
        {
            Some(Err(ValidErr::BrokenConstant(1, msg))) => {
                assert_eq!(msg, "testfound '1' which evaluates to 3 in an iteration where elements must evaluate to 2")
            }
            _ => panic!("bad value in const_over"),
        }
    }

    #[test]
    fn test_broken_const_macro_auto_debug() {
        match [[[0]], [[1]]]
            .into_iter()
            .validate()
            .const_over(|e| e[0], broken_const!("test" plus_auto_debug))
            .nth(1)
        {
            Some(Err(ValidErr::BrokenConstant([[1]], msg))) => {
                assert_eq!(msg, "testfound '[[1]]' which evaluates to [1] in an iteration where elements must evaluate to [0]")
            }
            _ => panic!("bad value in const_over"),
        }
    }
}
