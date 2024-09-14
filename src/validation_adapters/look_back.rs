/// The [`LookBack`] ValidIter adapter, for more info see
///  [`look_back`](crate::ValidIter::look_back) and [`look_back_n`](crate::ValidIter::look_back_n).
#[derive(Debug, Clone)]
pub struct LookBackIter<I, T, E, A, M, F>
where
    I: Iterator<Item = Result<T, E>>,
    M: Fn(&T) -> A,
    F: Fn(&T, &A) -> bool,
{
    iter: I,
    steps: usize,
    pos: usize,
    value_store: Vec<A>,
    extractor: M,
    validation: F,
    factory: fn(T, &A) -> E,
}

impl<I, T, E, A, M, F> LookBackIter<I, T, E, A, M, F>
where
I: Iterator<Item = Result<T, E>>,
M: Fn(&T) -> A,
F: Fn(&T, &A) -> bool,
{
    pub(crate) fn new(iter: I, steps: usize, extractor: M, validation: F, factory: fn(T, &A) -> E) -> LookBackIter<I, T, E, A, M, F> {
        Self {
            iter,
            steps,
            pos: 0,
            value_store: Vec::with_capacity(steps),
            extractor,
            validation,
            factory
        }
    }
}

impl<I, T, E, A, M, F> Iterator for LookBackIter<I, T, E, A, M, F>
where
I: Iterator<Item = Result<T, E>>,
M: Fn(&T) -> A,
F: Fn(&T, &A) -> bool,
{
    type Item = Result<T, E>;

    fn next(&mut self) -> Option<Self::Item> {
        // prevent modulo 0 div
        if self.steps == 0 {
            return self.iter.next();
        }

        match self.iter.next() {
            Some(Ok(val)) => {
                if self.pos >= self.steps {
                    let cycle_index = self.pos % self.steps;
                    let former = &self.value_store[cycle_index];
                    let vresult = (self.validation)(&val, former);
                    match vresult {
                        true => {
                            self.value_store[cycle_index] = (self.extractor)(&val);
                            self.pos += 1;
                            Some(Ok(val))
                        }
                        false => Some(Err((self.factory)(val, former))),
                    }
                } else {
                    self.value_store.push((self.extractor)(&val));
                    self.pos += 1;
                    Some(Ok(val))
                }
            }
            other => other,
        }
    }
}

pub trait LookBack<T, E, A, M, F>: Iterator<Item = Result<T, E>> + Sized
where
M: Fn(&T) -> A,
F: Fn(&T, &A) -> bool,
{
    fn look_back(self, steps: usize, extractor: M, validation: F, factory: fn(T, &A) -> E) -> LookBackIter<Self, T, E, A, M, F>;
}

impl<I, T, E, A, M, F> LookBack<T, E, A, M, F> for I
where
I: Iterator<Item = Result<T, E>>,
M: Fn(&T) -> A,
F: Fn(&T, &A) -> bool,
{
    fn look_back(self, steps: usize, extractor: M, validation: F, factory: fn(T, &A) -> E) -> LookBackIter<Self, T, E, A, M, F> {
        LookBackIter::new(self, steps, extractor, validation, factory)
    }
}

#[cfg(test)]
mod tests {
    use crate::LookBack;

    #[derive(Debug, PartialEq)]
    enum TestErr<T> {
        LookBackFailed(T, String),
        Is0Or3(T)
    }

    fn lbfailed<T, A>(item: T, against: &A) -> TestErr<T> where A: std::fmt::Display {
        TestErr::LookBackFailed(item, format!("{against}"))
    }

    #[test]
    fn test_lookback_ok() {
        if (0..10)
            .map(|i| Ok(i))
            .look_back(3, |i| *i, |i, prev| prev < i, lbfailed)
            .any(|res| res.is_err())
        {
            panic!("look back failed on ok iteration")
        }
    }

    #[test]
    fn test_lookback_err() {
        let lookback_err: Vec<Result<_, _>> = (2..=4)
            .chain(2..=2)
            .chain(0..6)
            .map(|i| Ok(i))
            .look_back(3, |i| *i, |i, prev| prev < i, lbfailed)
            .collect();

        assert_eq!(
            lookback_err,
            [
                Ok(2),
                Ok(3),
                Ok(4),
                Err(TestErr::LookBackFailed(2, "2".to_string())),
                Err(TestErr::LookBackFailed(0, "2".to_string())),
                Err(TestErr::LookBackFailed(1, "2".to_string())),
                Err(TestErr::LookBackFailed(2, "2".to_string())),
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
            .map(|i| Ok(i))
            .look_back(0, |i| *i, |prev, i| prev < i, lbfailed)
            .any(|res| res.is_err())
        {
            panic!("look back failed when it should not be validating anything")
        }
    }

    #[test]
    fn test_lookback_does_nothing_when_lookback_is_larger_than_iter() {
        if (0..5)
            .chain(0..=0)
            .map(|i| Ok(i))
            .look_back(7, |i| *i, |prev, i| prev < i, lbfailed)
            .any(|res| res.is_err())
        {
            panic!("look back failed when lookback is out of bounds")
        }
    }

    #[test]
    fn test_lookback_bounds() {
        if (0..5)
            .map(|i| Ok(i))
            .look_back(5, |i| *i, |prev, i| prev == i, lbfailed)
            .any(|res| res.is_err())
        {
            panic!("failed on too early look back")
        }

        if !(0..5)
            .map(|i| Ok(i))
            .look_back(4, |i| *i, |prev, i| prev == i, lbfailed)
            .any(|res| res.is_err())
        {
            panic!("did not fail on count-1 look back")
        }

        if (0..=0)
            .map(|i| Ok(i))
            .look_back(1,|i| *i, |prev, i| prev == i, lbfailed)
            .any(|res| res.is_err())
        {
            panic!("failed on look back when count is 1")
        }

        if (0..0)
            .map(|i| Ok(i))
            .look_back(0, |i| *i, |prev, i| prev == i, lbfailed)
            .any(|res| res.is_err())
        {
            panic!("failed on look back when count is 0")
        }
    }

    #[test]
    fn test_lookback_ignores_its_errors() {
        let results: Vec<Result<_,_>> = [0, 0, 1, 2, 0]
            .iter()
            .map(|i| Ok(i))
            .look_back(2, |i| **i, |prev, i| i == *prev, lbfailed)
            .collect();
        assert_eq!(
            results,
            [
                Ok(&0),
                Ok(&0),
                Err(TestErr::LookBackFailed(&1, "0".to_string())),
                Err(TestErr::LookBackFailed(&2, "0".to_string())),
                Ok(&0)
            ]
        )
    }

    #[test]
    fn test_lookback_ok_then_err_then_ok_then_err_then_ok() {
        let results: Vec<Result<_, _>> = [0, 1, 0, 1, 1, 0, 1, 1, 0, 1]
            .iter()
            .map(|i| Ok(i))
            .look_back(2, |i| **i, |i, prev| *i % 2 == *prev % 2, lbfailed)
            .collect();
        assert_eq!(
            results,
            [
                Ok(&0),
                Ok(&1),
                Ok(&0),
                Ok(&1),
                Err(TestErr::LookBackFailed(&1, "0".to_string())),
                Ok(&0),
                Ok(&1),
                Err(TestErr::LookBackFailed(&1, "0".to_string())),
                Ok(&0),
                Ok(&1),
            ]
        )
    }

    #[test]
    fn test_lookback_ignores_errors() {
        let results = (0..=5)
            .map(|i| if i != 0 && i != 3 {return Ok(i)} else {return Err(TestErr::Is0Or3(i))})
            .look_back(1, |i| i % 2, |j, parity| j % 2 != *parity, lbfailed)
            .collect::<Vec<_>>();

        assert_eq!(
            results,
            vec![
                Err(TestErr::Is0Or3(0)),
                Ok(1),
                Ok(2),
                Err(TestErr::Is0Or3(3)),
                Err(TestErr::LookBackFailed(4, "0".to_string())),
                Ok(5)
            ]
        )
    }
}
