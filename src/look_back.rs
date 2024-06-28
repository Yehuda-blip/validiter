use std::rc::Rc;

use crate::{valid_iter::ValidIter, valid_result::ValidErr};

use super::valid_result::VResult;

#[derive(Debug, Clone)]
pub struct LookBack<I, A, M, F, const N: usize>
where
    I: ValidIter + Iterator<Item = VResult<I::BaseType>>,
    A: Default,
    M: Fn(&I::BaseType) -> A,
    F: Fn(&A, &I::BaseType) -> bool,
{
    iter: I,
    pos: usize,
    value_store: [A; N],
    extractor: M,
    validation: F,
    desc: Rc<str>,
}

impl<I, A, M, F, const N: usize> LookBack<I, A, M, F, N>
where
    I: Sized + ValidIter + Iterator<Item = VResult<I::BaseType>>,
    A: Default,
    M: Fn(&I::BaseType) -> A,
    F: Fn(&A, &I::BaseType) -> bool,
{
    pub fn new(iter: I, extractor: M, validation: F, desc: &str) -> LookBack<I, A, M, F, N> {
        Self {
            iter,
            pos: 0,
            //https://stackoverflow.com/a/67180898/16887886
            value_store: [(); N].map(|_| A::default()),
            extractor,
            validation,
            desc: Rc::from(desc),
        }
    }
}

impl<I, A, M, F, const N: usize> Iterator for LookBack<I, A, M, F, N>
where
    I: ValidIter + Iterator<Item = VResult<I::BaseType>>,
    A: Default,
    M: Fn(&I::BaseType) -> A,
    F: Fn(&A, &I::BaseType) -> bool,
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
                        false => Some(Err(ValidErr::WithElement(val, Rc::clone(&self.desc)))),
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

impl<I, A, M, F, const N: usize> ValidIter for LookBack<I, A, M, F, N>
where
    I: ValidIter + Iterator<Item = VResult<I::BaseType>>,
    A: Default,
    M: Fn(&I::BaseType) -> A,
    F: Fn(&A, &I::BaseType) -> bool,
{
    type BaseType = I::BaseType;
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::{
        valid_iter::{Unvalidatable, ValidIter},
        valid_result::{VResult, ValidErr},
    };

    #[test]
    fn test_lookback_ok() {
        if (0..10)
            .validate()
            .look_back_n::<3, _, _, _>(|i| *i, |prev, i| prev < i, "lb")
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
            .look_back_n::<3, _, _, _>(|i| *i, |prev, i| prev < i, "lb")
            .collect();

        assert_eq!(
            lookback_err,
            [
                Ok(2),
                Ok(3),
                Ok(4),
                Err(ValidErr::WithElement(2, Rc::from("lb"))),
                Err(ValidErr::WithElement(0, Rc::from("lb"))),
                Err(ValidErr::WithElement(1, Rc::from("lb"))),
                Err(ValidErr::WithElement(2, Rc::from("lb"))),
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
            .look_back_n::<0, _, _, _>(|i| *i, |prev, i| prev < i, "lb")
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
            .look_back_n::<7, _, _, _>(|i| *i, |prev, i| prev < i, "lb")
            .any(|res| res.is_err())
        {
            panic!("look back failed when lookback is out of bounds")
        }
    }

    #[test]
    fn test_lookback_bounds() {
        if (0..5)
            .validate()
            .look_back_n::<5, _, _, _>(|i| *i, |prev, i| prev == i, "lb")
            .any(|res| res.is_err())
        {
            panic!("failed on too early look back")
        }

        if !(0..5)
            .validate()
            .look_back_n::<4, _, _, _>(|i| *i, |prev, i| prev == i, "lb")
            .any(|res| res.is_err())
        {
            panic!("did not fail on count-1 look back")
        }

        if (0..=0)
            .validate()
            .look_back_n::<1, _, _, _>(|i| *i, |prev, i| prev == i, "lb")
            .any(|res| res.is_err())
        {
            panic!("failed on look back when count is 1")
        }

        if (0..0)
            .validate()
            .look_back_n::<0, _, _, _>(|i| *i, |prev, i| prev == i, "lb")
            .any(|res| res.is_err())
        {
            panic!("failed on look back when count is 0")
        }
    }

    #[test]
    fn test_default_lookback_is_1() {
        if (0..4)
            .validate()
            .look_back(|i| *i, |prev, i| i - 1 == *prev, "lb")
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
            .look_back_n::<2, _, _, _>(|i| **i, |prev, i| *i == prev, "lb")
            .collect();
        assert_eq!(
            results,
            [
                Ok(&0),
                Ok(&0),
                Err(ValidErr::WithElement(&1, Rc::from("lb"))),
                Err(ValidErr::WithElement(&2, Rc::from("lb"))),
                Ok(&0)
            ]
        )
    }

    #[test]
    fn test_lookback_ok_then_err_then_ok_then_err_then_ok() {
        let results: Vec<VResult<_>> = [0, 1, 0, 1, 1, 0, 1, 1, 0, 1]
            .iter()
            .validate()
            .look_back_n::<2, _, _, _>(|i| **i, |prev, i| *i % 2 == prev % 2, "lb")
            .collect();
        assert_eq!(
            results,
            [
                Ok(&0),
                Ok(&1),
                Ok(&0),
                Ok(&1),
                Err(ValidErr::WithElement(&1, Rc::from("lb"))),
                Ok(&0),
                Ok(&1),
                Err(ValidErr::WithElement(&1, Rc::from("lb"))),
                Ok(&0),
                Ok(&1),
            ]
        )
    }

    #[test]
    fn test_lookback_ignores_errors() {
        let results = (0..=5)
            .validate()
            .ensure(|i| *i != 0 && *i != 3, "ensure")
            .look_back(|i| i % 2, |parity, j| j % 2 != *parity, "look-back")
            .collect::<Vec<_>>();

        assert_eq!(
            results,
            vec![
                Err(ValidErr::WithElement(0, Rc::from("ensure"))),
                Ok(1),
                Ok(2),
                Err(ValidErr::WithElement(3, Rc::from("ensure"))),
                Err(ValidErr::WithElement(4, Rc::from("look-back"))),
                Ok(5)
            ]
        )
    }
}
