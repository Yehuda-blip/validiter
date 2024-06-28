use std::rc::Rc;

use crate::{
    valid_iter::ValidIter,
    valid_result::{VResult, ValidErr},
};


/// The [`ConstOver`] ValidIter adapter, for more info see [`const_over`](crate::ValidIter::const_over).
#[derive(Debug, Clone)]
pub struct ConstOver<I, A, M>
where
    I: ValidIter + Iterator<Item = VResult<I::BaseType>>,
    A: PartialEq,
    M: Fn(&I::BaseType) -> A,
{
    iter: I,
    stored_value: Option<A>,
    extractor: M,
    desc: Rc<str>,
}

impl<I, A, M> ConstOver<I, A, M>
where
    I: ValidIter + Iterator<Item = VResult<I::BaseType>>,
    A: PartialEq,
    M: Fn(&I::BaseType) -> A,
{
    pub(crate) fn new(iter: I, extractor: M, desc: &str) -> ConstOver<I, A, M> {
        Self {
            iter,
            stored_value: None,
            extractor,
            desc: Rc::from(desc),
        }
    }
}

impl<I, A, M> Iterator for ConstOver<I, A, M>
where
    I: ValidIter + Iterator<Item = VResult<I::BaseType>>,
    A: PartialEq,
    M: Fn(&I::BaseType) -> A,
{
    type Item = VResult<I::BaseType>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(Ok(val)) => match &self.stored_value {
                Some(expected_const) => match (self.extractor)(&val) == *expected_const {
                    true => Some(Ok(val)),
                    false => Some(Err(ValidErr::WithElement(val, Rc::clone(&self.desc)))),
                },
                None => {
                    self.stored_value = Some((self.extractor)(&val));
                    Some(Ok(val))
                }
            },
            other => other,
        }
    }
}

impl<I, A, M> ValidIter for ConstOver<I, A, M>
where
    I: ValidIter + Iterator<Item = VResult<I::BaseType>>,
    A: PartialEq,
    M: Fn(&I::BaseType) -> A,
{
    type BaseType = I::BaseType;
}

#[cfg(test)]
mod tests {
    use std::{iter::repeat, rc::Rc};

    use crate::{
        valid_iter::{Unvalidatable, ValidIter},
        valid_result::ValidErr,
    };

    #[test]
    fn test_const_over_ok() {
        if repeat(1)
            .take(5)
            .validate()
            .const_over(|i| *i, "co")
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
            .const_over(|i| *i, "co")
            .collect();
        assert_eq!(
            results,
            [
                Ok(0),
                Ok(0),
                Ok(0),
                Err(ValidErr::WithElement(1, Rc::from("co")))
            ]
        )
    }

    #[test]
    fn test_const_over_bounds() {
        if (0..0)
            .validate()
            .const_over(|i| *i, "co")
            .any(|res| res.is_err())
        {
            panic!("const over failed on empty iter")
        }

        if (0..1)
            .validate()
            .const_over(|i| *i, "co")
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
            .const_over(|slice| slice[0], "co")
            .collect();
        assert_eq!(
            results,
            [
                Ok([0]),
                Ok([0]),
                Ok([0]),
                Err(ValidErr::WithElement([1], Rc::from("co"))),
                Ok([0]),
                Err(ValidErr::WithElement([2], Rc::from("co")))
            ]
        )
    }

    #[test]
    fn test_const_over_ignores_errors() {
        let results = (0..=4)
            .validate()
            .ensure(|i| *i != 0 && *i != 2, "ensure")
            .const_over(|i| i % 2, "const-over")
            .collect::<Vec<_>>();

        assert_eq!(
            results,
            vec![
                Err(ValidErr::WithElement(0, Rc::from("ensure"))),
                Ok(1),
                Err(ValidErr::WithElement(2, Rc::from("ensure"))),
                Ok(3),
                Err(ValidErr::WithElement(4, Rc::from("const-over")))
            ]
        )
    }
}
