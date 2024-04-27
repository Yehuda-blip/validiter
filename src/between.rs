use crate::{valid_iter::ValidIter, valid_result::ValidErr};

use super::valid_result::VResult;

#[macro_export]
macro_rules! out_of_bounds {
    ($description:literal plus_auto) => {
        |elmt, lower, upper| {
            $description.to_string()
                + &format!("found '{elmt}' which is out of the iteration bounds [{lower}, {upper}]")
        }
    };
    ($description:literal plus_auto_debug) => {
        |elmt, lower, upper| {
            $description.to_string()
                + &format!(
                    "found '{elmt:?}' which is out of the iteration bounds [{lower:?}, {upper:?}]"
                )
        }
    };
    ($description:literal) => {
        |_, _, _| $description.to_string()
    };
}

#[derive(Debug, Clone)]
pub struct Between<I, Msg>
where
    I: ValidIter + Iterator<Item = VResult<I::BaseType>>,
    I::BaseType: PartialOrd,
    Msg: Fn(&I::BaseType, &I::BaseType, &I::BaseType) -> String,
{
    iter: I,
    lower_bound: I::BaseType,
    upper_bound: I::BaseType,
    msg_writer: Msg,
}

impl<I, Msg> Between<I, Msg>
where
    I: Sized + ValidIter + Iterator<Item = VResult<I::BaseType>>,
    I::BaseType: PartialOrd,
    Msg: Fn(&I::BaseType, &I::BaseType, &I::BaseType) -> String,
{
    pub(crate) fn new(
        iter: I,
        lower_bound: I::BaseType,
        upper_bound: I::BaseType,
        err_msg: Msg,
    ) -> Between<I, Msg> {
        Between {
            iter,
            lower_bound,
            upper_bound,
            msg_writer: err_msg,
        }
    }
}

impl<I, Msg> Iterator for Between<I, Msg>
where
    I: ValidIter + Iterator<Item = VResult<I::BaseType>>,
    I::BaseType: PartialOrd,
    Msg: Fn(&I::BaseType, &I::BaseType, &I::BaseType) -> String,
{
    type Item = VResult<I::BaseType>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(Ok(val)) => match self.lower_bound <= val && val <= self.upper_bound {
                true => Some(Ok(val)),
                false => {
                    let msg = (self.msg_writer)(&val, &self.lower_bound, &self.upper_bound);
                    Some(Err(ValidErr::OutOfBounds(val, msg)))
                }
            },
            other => other,
        }
    }
}

impl<I, Msg> ValidIter for Between<I, Msg>
where
    I: ValidIter + Iterator<Item = VResult<I::BaseType>>,
    I::BaseType: PartialOrd,
    Msg: Fn(&I::BaseType, &I::BaseType, &I::BaseType) -> String,
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
    fn test_between() {
        let validated = [
            -1.3,
            -0.3,
            0.7,
            1.7,
            f64::NEG_INFINITY,
            f64::INFINITY,
            f64::NAN,
        ]
        .iter()
        .validate()
        .between(&-0.5, &1.5, |elmt, l, u| format!("{elmt}-{l}-{u}"))
        .collect::<Vec<VResult<_>>>();
        assert_eq!(
            validated[0..validated.len() - 1],
            [
                Err(ValidErr::OutOfBounds(&-1.3, "-1.3--0.5-1.5".to_string())),
                Ok(&-0.3),
                Ok(&0.7),
                Err(ValidErr::OutOfBounds(&1.7, "1.7--0.5-1.5".to_string())),
                Err(ValidErr::OutOfBounds(
                    &f64::NEG_INFINITY,
                    "-inf--0.5-1.5".to_string()
                )),
                Err(ValidErr::OutOfBounds(
                    &f64::INFINITY,
                    "inf--0.5-1.5".to_string()
                ))
            ]
        );
        let nan_out_of_bounds = &validated[validated.len() - 1];
        match nan_out_of_bounds {
            Ok(_) => panic!("non ordered item validated as in bounds"),
            Err(ValidErr::OutOfBounds(oob, msg)) => {
                assert!(oob.is_nan());
                assert_eq!(msg, "NaN--0.5-1.5")
            }
            _ => panic!("unexpected value in at least"),
        }
    }

    #[test]
    fn test_between_is_range_inclusive() {
        let results: Vec<_> = (0..=4)
            .validate()
            .between(1, 3, |_, _, _| "".to_string())
            .collect();
        assert_eq!(
            results,
            [
                Err(ValidErr::OutOfBounds(0, "".to_string())),
                Ok(1),
                Ok(2),
                Ok(3),
                Err(ValidErr::OutOfBounds(4, "".to_string()))
            ]
        )
    }

    #[test]
    fn test_between_is_capable_of_allowing_single_value() {
        let results: Vec<_> = (0..=2)
            .validate()
            .between(1, 1, |_, _, _| "".to_string())
            .collect();
        assert_eq!(
            results,
            [
                Err(ValidErr::OutOfBounds(0, "".to_string())),
                Ok(1),
                Err(ValidErr::OutOfBounds(2, "".to_string()))
            ]
        )
    }

    #[test]
    fn test_out_of_bounds_macro_user_input_only() {
        match (1..=1)
            .validate()
            .between(0, 0, out_of_bounds!("test"))
            .next()
        {
            Some(Err(ValidErr::OutOfBounds(1, msg))) => {
                assert_eq!(msg, "test")
            }
            _ => panic!("bad value for out of bounds"),
        }
    }

    #[test]
    fn test_out_of_bounds_macro_auto() {
        match (1..=1)
            .validate()
            .between(-1, 0, out_of_bounds!("test" plus_auto))
            .next()
        {
            Some(Err(ValidErr::OutOfBounds(1, msg))) => {
                assert_eq!(
                    msg,
                    "testfound '1' which is out of the iteration bounds [-1, 0]"
                )
            }
            _ => panic!("bad value for out of bounds"),
        }
    }

    #[derive(Debug, PartialEq, PartialOrd)]
    struct TestStruct(i32);

    #[test]
    fn test_out_of_bounds_macro_auto_debug() {
        match [TestStruct(5)]
            .into_iter()
            .validate()
            .between(
                TestStruct(0),
                TestStruct(1),
                out_of_bounds!("test" plus_auto_debug),
            )
            .next()
        {
            Some(Err(ValidErr::OutOfBounds(TestStruct(5), msg))) => {
                assert_eq!(msg, "testfound 'TestStruct(5)' which is out of the iteration bounds [TestStruct(0), TestStruct(1)]")
            }
            _ => panic!("bad match in between test"),
        }
    }
}
