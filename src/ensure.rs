use crate::{valid_iter::ValidIter, valid_result::ValidErr};

use super::valid_result::VResult;

#[macro_export]
macro_rules! invalid {
    ($description:literal plus_auto) => {
        |elmt| $description.to_string() + &format!("element '{elmt}' fails the validation")
    };
    ($description:literal plus_auto_debug) => {
        |elmt| $description.to_string() + &format!("element '{elmt:?}' fails the validation")
    };
    ($description:literal) => {
        |_| $description.to_string()
    };
}

#[derive(Debug, Clone)]
pub struct Ensure<I, F, Msg>
where
    I: ValidIter + Iterator<Item = VResult<I::BaseType>>,
    F: FnMut(&I::BaseType) -> bool,
    Msg: Fn(&I::BaseType) -> String,
{
    iter: I,
    validation: F,
    msg_writer: Msg,
}

impl<I, F, Msg> Ensure<I, F, Msg>
where
    I: Sized + ValidIter + Iterator<Item = VResult<I::BaseType>>,
    F: FnMut(&I::BaseType) -> bool,
    Msg: Fn(&I::BaseType) -> String,
{
    pub(crate) fn new(iter: I, validation: F, err_msg: Msg) -> Ensure<I, F, Msg> {
        Ensure {
            iter,
            validation,
            msg_writer: err_msg,
        }
    }
}

impl<I, F, Msg> Iterator for Ensure<I, F, Msg>
where
    I: ValidIter + Iterator<Item = VResult<I::BaseType>>,
    F: FnMut(&I::BaseType) -> bool,
    Msg: Fn(&I::BaseType) -> String,
{
    type Item = VResult<I::BaseType>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(Ok(val)) => match (self.validation)(&val) {
                true => Some(Ok(val)),
                false => {
                    let msg = (self.msg_writer)(&val);
                    Some(Err(ValidErr::Invalid(val, msg)))
                }
            },
            other => other,
        }
    }
}

impl<I, F, Msg> ValidIter for Ensure<I, F, Msg>
where
    I: ValidIter + Iterator<Item = VResult<I::BaseType>>,
    F: FnMut(&I::BaseType) -> bool,
    Msg: Fn(&I::BaseType) -> String,
{
    type BaseType = I::BaseType;
}

#[cfg(test)]
mod tests {
    use crate::{
        valid_iter::{Unvalidatable, ValidIter},
        valid_result::ValidErr,
    };

    #[test]
    fn test_ensure() {
        (0..10)
            .validate()
            .ensure(|i| i % 2 == 0, |elmt| format!("{elmt}"))
            .enumerate()
            .for_each(|(i, res_i)| match res_i {
                Ok(int) if i % 2 == 0 && i as i32 == int => {}
                Err(ValidErr::Invalid(int, msg)) if i % 2 == 1 && i as i32 == int => {
                    assert_eq!(msg, int.to_string())
                }
                _ => panic!("unexpected value in ensure adapter"),
            })
    }

    #[test]
    fn test_invalid_macro_user_input_only() {
        match (0..=0)
            .validate()
            .ensure(|_| false, invalid!("test"))
            .next()
        {
            Some(Err(ValidErr::Invalid(0, msg))) => assert_eq!(msg, "test"),
            _ => panic!("should error"),
        }
    }

    #[test]
    fn test_invalid_macro_auto() {
        match (0..=0)
            .validate()
            .ensure(|_| false, invalid!("test" plus_auto))
            .next()
        {
            Some(Err(ValidErr::Invalid(0, msg))) => {
                assert_eq!(msg, "testelement '0' fails the validation")
            }
            _ => panic!("should error"),
        }
    }

    #[derive(Debug)]
    struct Struct;

    #[test]
    fn test_invalid_macro_auto_debug() {
        match [Struct]
            .into_iter()
            .validate()
            .ensure(|_| false, invalid!("test" plus_auto_debug))
            .next()
        {
            Some(Err(ValidErr::Invalid(Struct, msg))) => {
                assert_eq!(msg, "testelement 'Struct' fails the validation")
            }
            _ => panic!("should error"),
        }
    }
}
