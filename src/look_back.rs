use crate::{valid_iter::ValidIter, valid_result::ValidErr};

use super::valid_result::VResult;

#[macro_export]
macro_rules! failed_look_back {
    ($description:literal plus_auto) => {
        |elmt, against| $description.to_string() + &format!("validation failed when testing '{elmt}' against the extracted value '{against}'")
    };
    ($description:literal plus_auto_debug) => {
        |elmt, against| $description.to_string() + &format!("validation failed when testing '{elmt:?}' against the extracted value '{against:?}'")
    };
    ($description:literal) => {
        |_, _| $description.to_string()
    };
}

#[derive(Debug, Clone)]
pub struct LookBack<I, A, M, F, Msg, const N: usize>
where
    I: ValidIter + Iterator<Item = VResult<I::BaseType>>,
    A: Default,
    M: FnMut(&I::BaseType) -> A,
    F: FnMut(&A, &I::BaseType) -> bool,
    Msg: Fn(&I::BaseType, &A) -> String,
{
    iter: I,
    pos: usize,
    value_store: [A; N],
    extractor: M,
    validation: F,
    msg_writer: Msg,
}

impl<I, A, M, F, Msg, const N: usize> LookBack<I, A, M, F, Msg, N>
where
    I: Sized + ValidIter + Iterator<Item = VResult<I::BaseType>>,
    A: Default,
    M: FnMut(&I::BaseType) -> A,
    F: FnMut(&A, &I::BaseType) -> bool,
    Msg: Fn(&I::BaseType, &A) -> String,
{
    pub fn new(iter: I, extractor: M, validation: F, err_msg: Msg) -> LookBack<I, A, M, F, Msg, N> {
        Self {
            iter,
            pos: 0,
            //https://stackoverflow.com/a/67180898/16887886
            value_store: [(); N].map(|_| A::default()),
            extractor,
            validation,
            msg_writer: err_msg,
        }
    }
}

impl<I, A, M, F, Msg, const N: usize> Iterator for LookBack<I, A, M, F, Msg, N>
where
    I: ValidIter + Iterator<Item = VResult<I::BaseType>>,
    A: Default,
    M: FnMut(&I::BaseType) -> A,
    F: FnMut(&A, &I::BaseType) -> bool,
    Msg: Fn(&I::BaseType, &A) -> String,
{
    type Item = VResult<I::BaseType>;

    fn next(&mut self) -> Option<Self::Item> {
        // there isn't a way currently to evaluate
        // constant generics at compile time.
        // for more info: "error[E0401]: can't
        // use generic parameters from outer item"
        // in order to make sure that the program
        // does not crash when 'self.value_store'
        // has size 0, we have this edge case check
        if self.value_store.len() == 0 {
            return self.iter.next();
        }

        match self.iter.next() {
            Some(Ok(val)) => {
                if self.pos >= N {
                    let cycle_index = self.pos % N;
                    let former = &self.value_store[cycle_index];
                    let vresult = (self.validation)(former, &val);
                    match vresult {
                        true => {
                            self.value_store[cycle_index] = (self.extractor)(&val);
                            self.pos += 1;
                            Some(Ok(val))
                        }
                        false => {
                            let msg = (self.msg_writer)(&val, former);
                            Some(Err(ValidErr::LookBackFailed(val, msg)))
                        }
                    }
                } else {
                    self.value_store[self.pos] = (self.extractor)(&val);
                    self.pos += 1;
                    Some(Ok(val))
                }
            }
            other => other,
        }
    }
}

impl<I, A, M, F, Msg, const N: usize> ValidIter for LookBack<I, A, M, F, Msg, N>
where
    I: ValidIter + Iterator<Item = VResult<I::BaseType>>,
    A: Default,
    M: FnMut(&I::BaseType) -> A,
    F: FnMut(&A, &I::BaseType) -> bool,
    Msg: Fn(&I::BaseType, &A) -> String,
{
    type BaseType = I::BaseType;
}

#[cfg(test)]
mod tests {
    use crate::{
        valid_iter::{Unvalidatable, ValidIter},
        valid_result::{VResult, ValidErr},
    };

    #[test]
    fn test_lookback_ok() {
        if (0..10)
            .validate()
            .look_back_n::<3, _, _, _, _>(|i| *i, |prev, i| prev < i, |_, _| "".to_string())
            .any(|res| res.is_err())
        {
            panic!("look back failed on ok iteration")
        }
    }

    #[test]
    fn test_lookback_err() {
        let lookback_err: Vec<VResult<_>> = (2..=4)
            .chain(2..=2)
            .chain(0..6)
            .validate()
            .look_back_n::<3, _, _, _, _>(
                |i| *i,
                |prev, i| prev < i,
                |elmt, against| format!("{elmt}-{against}"),
            )
            .collect();

        assert_eq!(
            lookback_err,
            [
                Ok(2),
                Ok(3),
                Ok(4),
                Err(ValidErr::LookBackFailed(2, "2-2".to_string())),
                Err(ValidErr::LookBackFailed(0, "0-2".to_string())),
                Err(ValidErr::LookBackFailed(1, "1-2".to_string())),
                Err(ValidErr::LookBackFailed(2, "2-2".to_string())),
                Ok(3),
                Ok(4),
                Ok(5),
            ]
        )
    }

    #[test]
    fn test_lookback_does_nothing_on_0() {
        if (0..5)
            .chain(0..5)
            .validate()
            .look_back_n::<0, _, _, _, _>(|i| *i, |prev, i| prev < i, |_, _| "".to_string())
            .any(|res| res.is_err())
        {
            panic!("look back failed when it should not be validating anything")
        }
    }

    #[test]
    fn test_lookback_does_nothing_when_lookback_is_larger_than_iter() {
        if (0..5)
            .chain(0..=0)
            .validate()
            .look_back_n::<7, _, _, _, _>(|i| *i, |prev, i| prev < i, |_, _| "".to_string())
            .any(|res| res.is_err())
        {
            panic!("look back failed when lookback is out of bounds")
        }
    }

    #[test]
    fn test_lookback_bounds() {
        if (0..5)
            .validate()
            .look_back_n::<5, _, _, _, _>(|i| *i, |prev, i| prev == i, |_, _| "".to_string())
            .any(|res| res.is_err())
        {
            panic!("failed on too early look back")
        }

        if !(0..5)
            .validate()
            .look_back_n::<4, _, _, _, _>(|i| *i, |prev, i| prev == i, |_, _| "".to_string())
            .any(|res| res.is_err())
        {
            panic!("did not fail on count-1 look back")
        }

        if (0..=0)
            .validate()
            .look_back_n::<1, _, _, _, _>(|i| *i, |prev, i| prev == i, |_, _| "".to_string())
            .any(|res| res.is_err())
        {
            panic!("failed on look back when count is 1")
        }

        if (0..0)
            .validate()
            .look_back_n::<0, _, _, _, _>(|i| *i, |prev, i| prev == i, |_, _| "".to_string())
            .any(|res| res.is_err())
        {
            panic!("failed on look back when count is 0")
        }
    }

    #[test]
    fn test_default_lookback_is_1() {
        if (0..4)
            .validate()
            .look_back(|i| *i, |prev, i| i - 1 == *prev, |_, _| "".to_string())
            .any(|res| res.is_err())
        {
            panic!("should be incrementing iteration, approved by look back")
        }
    }

    #[test]
    fn test_lookback_ignores_its_errors() {
        let results: Vec<VResult<_>> = [0, 0, 1, 2, 0]
            .iter()
            .validate()
            .look_back_n::<2, _, _, _, _>(
                |i| **i,
                |prev, i| *i == prev,
                |elmt, against| format!("{elmt}-{against}"),
            )
            .collect();
        assert_eq!(
            results,
            [
                Ok(&0),
                Ok(&0),
                Err(ValidErr::LookBackFailed(&1, "1-0".to_string())),
                Err(ValidErr::LookBackFailed(&2, "2-0".to_string())),
                Ok(&0)
            ]
        )
    }

    #[test]
    fn test_lookback_ok_then_err_then_ok_then_err_then_ok() {
        let results: Vec<VResult<_>> = [0, 1, 0, 1, 1, 0, 1, 1, 0, 1]
            .iter()
            .validate()
            .look_back_n::<2, _, _, _, _>(
                |i| **i,
                |prev, i| *i % 2 == prev % 2,
                |elmt, against| format!("{elmt}-{against}"),
            )
            .collect();
        assert_eq!(
            results,
            [
                Ok(&0),
                Ok(&1),
                Ok(&0),
                Ok(&1),
                Err(ValidErr::LookBackFailed(&1, "1-0".to_string())),
                Ok(&0),
                Ok(&1),
                Err(ValidErr::LookBackFailed(&1, "1-0".to_string())),
                Ok(&0),
                Ok(&1),
            ]
        )
    }

    #[test]
    fn test_failed_look_back_macro_user_input_only() {
        let res = (0..=1)
            .validate()
            .look_back(|_| 0, |val, prev| val == prev, failed_look_back!("oops"))
            .nth(1);
        assert_eq!(
            res,
            Some(Err(ValidErr::LookBackFailed(1, "oops".to_string())))
        )
    }

    #[test]
    fn test_failed_look_back_macro_plus_auto() {
        let res = (0..=1)
            .validate()
            .look_back(
                |_| 0,
                |val, prev| val == prev,
                failed_look_back!("oops" plus_auto),
            )
            .nth(1);
        assert_eq!(
            res,
            Some(Err(ValidErr::LookBackFailed(
                1,
                "oopsvalidation failed when testing '1' against the extracted value '0'"
                    .to_string()
            )))
        )
    }

    #[derive(Debug, PartialEq)]
    struct Struct1;

    #[derive(Debug, Default)]
    struct Struct2;

    #[test]
    fn test_failed_look_back_macro_plus_auto_debug() {
        let res = [Struct1, Struct1]
            .into_iter()
            .validate()
            .look_back(
                |_| Struct2,
                |_, _| false,
                failed_look_back!("oops" plus_auto_debug),
            )
            .nth(1);
        assert_eq!(res, Some(Err(ValidErr::LookBackFailed(Struct1, "oopsvalidation failed when testing 'Struct1' against the extracted value 'Struct2'".to_string()))))
    }
}
