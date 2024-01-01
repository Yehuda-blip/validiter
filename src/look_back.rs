use crate::{valid_iter::ValidIter, valid_result::ValidErr};

use super::valid_result::VResult;

pub struct LookBack<I, A, M, F, const N: usize>
where
    I: ValidIter + Iterator<Item = VResult<I::BaseType>>,
    A: Default,
    M: FnMut(&I::BaseType) -> A,
    F: FnMut(&A, &I::BaseType) -> bool,
{
    iter: I,
    pos: usize,
    value_store: [A; N],
    extractor: M,
    validation: F,
}

impl<I, A, M, F, const N: usize> LookBack<I, A, M, F, N>
where
    I: ValidIter + Iterator<Item = VResult<I::BaseType>>,
    A: Default,
    M: FnMut(&I::BaseType) -> A,
    F: FnMut(&A, &I::BaseType) -> bool,
{
    pub fn new(iter: I, extractor: M, validation: F) -> LookBack<I, A, M, F, N>
    where
        I: Sized,
    {
        Self {
            iter,
            pos: 0,
            //https://stackoverflow.com/a/67180898/16887886
            value_store: [(); N].map(|_| A::default()),
            extractor,
            validation,
        }
    }
}

impl<I, A, M, F, const N: usize> Iterator for LookBack<I, A, M, F, N>
where
    I: ValidIter + Iterator<Item = VResult<I::BaseType>>,
    A: Default,
    M: FnMut(&I::BaseType) -> A,
    F: FnMut(&A, &I::BaseType) -> bool,
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
                        false => Some(Err(ValidErr::Incosistent(val))),
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
    M: FnMut(&I::BaseType) -> A,
    F: FnMut(&A, &I::BaseType) -> bool,
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
            .look_back::<3, _, _, _>(|i| *i, |prev, i| prev < i)
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
            .look_back::<3, _, _, _>(|i| *i, |prev, i| prev < i)
            .collect();

        assert_eq!(
            lookback_err,
            [
                Ok(2),
                Ok(3),
                Ok(4),
                Err(ValidErr::Incosistent(2)),
                Err(ValidErr::Incosistent(0)),
                Err(ValidErr::Incosistent(1)),
                Err(ValidErr::Incosistent(2)),
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
            .look_back::<0, _, _, _>(|i| *i, |prev, i| prev < i)
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
            .look_back::<7, _, _, _>(|i| *i, |prev, i| prev < i)
            .any(|res| res.is_err())
        {
            panic!("look back failed when lookback is out of bounds")
        }
    }
}
