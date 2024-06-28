use std::rc::Rc;

use crate::{ValidIter, ValidErr, VResult};


/// The [`Between`] ValidIter adapter, for more info see [`between`](crate::ValidIter::between).
#[derive(Debug, Clone)]
pub struct Between<I>
where
    I: ValidIter + Iterator<Item = VResult<I::BaseType>>,
    I::BaseType: PartialOrd,
{
    iter: I,
    lower_bound: I::BaseType,
    upper_bound: I::BaseType,
    desc: Rc<str>,
}

impl<I> Between<I>
where
    I: Sized + ValidIter + Iterator<Item = VResult<I::BaseType>>,
    I::BaseType: PartialOrd,
{
    pub(crate) fn new(
        iter: I,
        lower_bound: I::BaseType,
        upper_bound: I::BaseType,
        desc: &str,
    ) -> Between<I> {
        Between {
            iter,
            lower_bound,
            upper_bound,
            desc: Rc::from(desc),
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
                false => Some(Err(ValidErr::WithElement(val, Rc::clone(&self.desc)))),
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
    use std::rc::Rc;

    use crate::{Unvalidatable, ValidIter, VResult, ValidErr};

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
        .between(&-0.5, &1.5, "test")
        .collect::<Vec<VResult<_>>>();
        assert_eq!(
            validated[0..validated.len() - 1],
            [
                Err(ValidErr::WithElement(&-1.3, Rc::from("test"))),
                Ok(&-0.3),
                Ok(&0.7),
                Err(ValidErr::WithElement(&1.7, Rc::from("test"))),
                Err(ValidErr::WithElement(&f64::NEG_INFINITY, Rc::from("test"))),
                Err(ValidErr::WithElement(&f64::INFINITY, Rc::from("test")))
            ]
        );
        let nan_out_of_bounds = &validated[validated.len() - 1];
        match nan_out_of_bounds {
            Ok(_) => panic!("non ordered item validated as in bounds"),
            Err(ValidErr::WithElement(oob, msg)) => assert!(oob.is_nan() && msg.as_ref() == "test"),
            _ => panic!("unexpected value in at least"),
        }
    }

    #[test]
    fn test_between_is_range_inclusive() {
        let results: Vec<_> = (0..=4).validate().between(1, 3, "oob").collect();
        assert_eq!(
            results,
            [
                Err(ValidErr::WithElement(0, Rc::from("oob"))),
                Ok(1),
                Ok(2),
                Ok(3),
                Err(ValidErr::WithElement(4, Rc::from("oob")))
            ]
        )
    }

    #[test]
    fn test_between_is_capable_of_allowing_single_value() {
        let results: Vec<_> = (0..=2).validate().between(1, 1, "oob").collect();
        assert_eq!(
            results,
            [
                Err(ValidErr::WithElement(0, Rc::from("oob"))),
                Ok(1),
                Err(ValidErr::WithElement(2, Rc::from("oob")))
            ]
        )
    }

    #[test]
    fn test_between_igonres_errors() {
        let results = (1..=1)
            .validate()
            .ensure(|i| i == &0, "ensure")
            .between(0, 0, "between")
            .collect::<Vec<_>>();
        assert_eq!(
            results,
            vec![Err(ValidErr::WithElement(1, Rc::from("ensure")))]
        )
    }
}
