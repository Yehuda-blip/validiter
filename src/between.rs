use crate::{valid_iter::ValidIter, valid_result::ValidErr};

use super::valid_result::VResult;

#[derive(Debug, Clone)]
pub struct Between<I>
where
    I: ValidIter + Iterator<Item = VResult<I::BaseType>>,
    I::BaseType: PartialOrd,
{
    iter: I,
    lower_bound: I::BaseType,
    upper_bound: I::BaseType,
}

impl<I> Between<I>
where
    I: Sized + ValidIter + Iterator<Item = VResult<I::BaseType>>,
    I::BaseType: PartialOrd,
{
    pub(crate) fn new(iter: I, lower_bound: I::BaseType, upper_bound: I::BaseType) -> Between<I> {
        Between {
            iter,
            lower_bound,
            upper_bound,
        }
    }
}

impl<I> Iterator for Between<I>
where
    I: ValidIter + Iterator<Item = VResult<I::BaseType>>,
    I::BaseType: PartialOrd,
{
    type Item = VResult<I::BaseType>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(Ok(val)) => match self.lower_bound <= val && val <= self.upper_bound {
                true => Some(Ok(val)),
                false => Some(Err(ValidErr::OutOfBounds(val))),
            },
            other => other,
        }
    }
}

impl<I> ValidIter for Between<I>
where
    I: ValidIter + Iterator<Item = VResult<I::BaseType>>,
    I::BaseType: PartialOrd,
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
        .between(&-0.5, &1.5)
        .collect::<Vec<VResult<_>>>();
        assert_eq!(
            validated[0..validated.len() - 1],
            [
                Err(ValidErr::OutOfBounds(&-1.3)),
                Ok(&-0.3),
                Ok(&0.7),
                Err(ValidErr::OutOfBounds(&1.7)),
                Err(ValidErr::OutOfBounds(&f64::NEG_INFINITY)),
                Err(ValidErr::OutOfBounds(&f64::INFINITY))
            ]
        );
        let nan_out_of_bounds = &validated[validated.len() - 1];
        match nan_out_of_bounds {
            Ok(_) => panic!("non ordered item validated as in bounds"),
            Err(ValidErr::OutOfBounds(oob)) => assert!(oob.is_nan()),
            _ => panic!("unexpected value in at least"),
        }
    }

    #[test]
    fn test_between_is_range_inclusive() {
        let results: Vec<_> = (0..=4).validate().between(1, 3).collect();
        assert_eq!(
            results,
            [
                Err(ValidErr::OutOfBounds(0)),
                Ok(1),
                Ok(2),
                Ok(3),
                Err(ValidErr::OutOfBounds(4))
            ]
        )
    }

    #[test]
    fn test_between_is_capable_of_allowing_single_value() {
        let results: Vec<_> = (0..=2).validate().between(1, 1).collect();
        assert_eq!(
            results,
            [
                Err(ValidErr::OutOfBounds(0)),
                Ok(1),
                Err(ValidErr::OutOfBounds(2))
            ]
        )
    }
}
