use crate::{
    valid_iter::ValidIter,
    valid_result::{VResult, ValidErr},
};

#[derive(Debug, Clone)]
pub struct ConstOver<I, A, M>
where
    I: ValidIter + Iterator<Item = VResult<I::BaseType>>,
    A: PartialEq,
    M: FnMut(&I::BaseType) -> A,
{
    iter: I,
    stored_value: Option<A>,
    extractor: M,
}

impl<I, A, M> ConstOver<I, A, M>
where
    I: ValidIter + Iterator<Item = VResult<I::BaseType>>,
    A: PartialEq,
    M: FnMut(&I::BaseType) -> A,
{
    pub(crate) fn new(iter: I, extractor: M) -> ConstOver<I, A, M> {
        Self {
            iter,
            stored_value: None,
            extractor,
        }
    }
}

impl<I, A, M> Iterator for ConstOver<I, A, M>
where
    I: ValidIter + Iterator<Item = VResult<I::BaseType>>,
    A: PartialEq,
    M: FnMut(&I::BaseType) -> A,
{
    type Item = VResult<I::BaseType>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(Ok(val)) => match &self.stored_value {
                Some(expected_const) => match (self.extractor)(&val) == *expected_const {
                    true => Some(Ok(val)),
                    false => Some(Err(ValidErr::BrokenConstant(val))),
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
    M: FnMut(&I::BaseType) -> A,
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
            .const_over(|i| *i)
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
            .const_over(|i| *i)
            .collect();
        assert_eq!(
            results,
            [Ok(0), Ok(0), Ok(0), Err(ValidErr::BrokenConstant(1))]
        )
    }

    #[test]
    fn test_const_over_bounds() {
        if (0..0).validate().const_over(|i| *i).any(|res| res.is_err()) {
            panic!("const over failed on empty iter")
        }

        if (0..1).validate().const_over(|i| *i).any(|res| res.is_err()) {
            panic!("const over failed on count == 1 iter")
        }
    }

    #[test]
    fn test_const_over_all_elements_are_present_and_in_order() {
        let results: Vec<_> = [[0], [0], [0], [1], [0], [2]]
            .into_iter()
            .validate()
            .const_over(|slice| slice[0])
            .collect();
        assert_eq!(
            results,
            [
                Ok([0]),
                Ok([0]),
                Ok([0]),
                Err(ValidErr::BrokenConstant([1])),
                Ok([0]),
                Err(ValidErr::BrokenConstant([2]))
            ]
        )
    }
}
