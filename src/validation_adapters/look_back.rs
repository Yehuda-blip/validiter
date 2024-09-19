use std::iter::Enumerate;

/// The [`LookBack`] ValidIter adapter, for more info see
///  [`look_back`](crate::ValidIter::look_back) and [`look_back_n`](crate::ValidIter::look_back_n).
#[derive(Debug, Clone)]
pub struct LookBackIter<I, T, E, A, M, F, Factory>
where
    I: Iterator<Item = Result<T, E>>,
    M: Fn(&T) -> A,
    F: Fn(&T, &A) -> bool,
    Factory: Fn(usize, T, &A) -> E,
{
    iter: Enumerate<I>,
    steps: usize,
    pos: usize,
    value_store: Vec<A>,
    extractor: M,
    validation: F,
    factory: Factory,
}

impl<I, T, E, A, M, F, Factory> LookBackIter<I, T, E, A, M, F, Factory>
where
    I: Iterator<Item = Result<T, E>>,
    M: Fn(&T) -> A,
    F: Fn(&T, &A) -> bool,
    Factory: Fn(usize, T, &A) -> E,
{
    pub(crate) fn new(
        iter: I,
        steps: usize,
        extractor: M,
        validation: F,
        factory: Factory,
    ) -> LookBackIter<I, T, E, A, M, F, Factory> {
        Self {
            iter: iter.enumerate(),
            steps,
            pos: 0,
            value_store: Vec::with_capacity(steps),
            extractor,
            validation,
            factory,
        }
    }
}

impl<I, T, E, A, M, F, Factory> Iterator for LookBackIter<I, T, E, A, M, F, Factory>
where
    I: Iterator<Item = Result<T, E>>,
    M: Fn(&T) -> A,
    F: Fn(&T, &A) -> bool,
    Factory: Fn(usize, T, &A) -> E,
{
    type Item = Result<T, E>;

    fn next(&mut self) -> Option<Self::Item> {
        // prevent modulo 0 div
        if self.steps == 0 {
            if let Some((_, item)) = self.iter.next() {
                return Some(item);
            } else {
                return None;
            };
        }

        match self.iter.next() {
            Some((i, Ok(val))) => {
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
                        false => Some(Err((self.factory)(i, val, former))),
                    }
                } else {
                    self.value_store.push((self.extractor)(&val));
                    self.pos += 1;
                    Some(Ok(val))
                }
            }
            Some((_, err)) => Some(err),
            None => None,
        }
    }
}

pub trait LookBack<T, E, A, M, F, Factory>: Iterator<Item = Result<T, E>> + Sized
where
    M: Fn(&T) -> A,
    F: Fn(&T, &A) -> bool,
    Factory: Fn(usize, T, &A) -> E,
{
    /// Fails an iteration if it does not conform to some cycling
    /// of properties.
    ///
    /// `look_back(steps, extractor, validation, factory)` takes 4
    /// arguments:
    /// 1. `n` - a `usize` describing a cycle length
    /// 2. `extractor` - a mapping of iterator elements to some extracted
    /// value.
    /// 3. `test` - a test which accepts the value extracted from
    /// the nth preceding element, and tests the current element based
    /// on this value.
    /// 4. An error factory.
    ///
    /// Each iterator element wrapped in `Ok(element)` gets processed in
    /// these 2 ways:
    /// 1. Assuming there was a previous nth element (we'll call it `p_nth`),
    /// the current element is tested for `validation(element, extractor(p_nth))`.
    /// 2. If the element passed the test, it is wrapped in `Ok(element)`.
    /// otherwise `factory` gets called on the index of the error, the failing element,
    /// and a reference to the extracted value that failed the element.
    ///
    /// # Examples
    ///
    /// Basic usage:
    /// ```
    /// # use validiter::LookBack;
    /// let mut iter = (0..=2).chain(2..=4).map(|v| Ok(v)).look_back(
    ///     2,
    ///     |i| *i,
    ///     |prev, i| prev % 2 == i % 2,
    ///     |index, val, failed_against| (index, val, *failed_against),
    /// );
    /// assert_eq!(iter.next(), Some(Ok(0)));
    /// assert_eq!(iter.next(), Some(Ok(1)));
    /// assert_eq!(iter.next(), Some(Ok(2))); // evaluated with respect to 0
    /// assert_eq!(iter.next(), Some(Err((3, 2, 1)))); // at index 3, 2 is evaluated with respect to 1
    /// assert_eq!(iter.next(), Some(Ok(3))); // also evaluated with respect to 1
    /// assert_eq!(iter.next(), Some(Ok(4))); // evaluted with respect to 2
    /// ```
    ///
    /// `look_back` can be used to force a monotonic iteration with relation to some
    /// property, with the most obvious example being the 'monotonic increasing' one:
    /// ```
    /// # use validiter::LookBack;
    ///     (1..)
    ///         .map(|i| Ok((i as f64).log(std::f64::consts::E)))
    ///         .look_back(1, |val| *val, |val, prev| val > prev, |_, _, _| ())
    ///         .take(10)
    ///         .for_each(|f| {
    ///             f.expect("log e is not monotonic!");
    ///         });
    /// ```
    ///
    ///
    /// `look_back` could be used to force an iteration to cycle through
    /// a sequence of predetermined properties:
    /// ```
    /// # use validiter::LookBack;
    /// let sequence = "abc";
    /// let s = "abfbc";
    /// 
    /// let mut iter = sequence.chars().chain(s.chars()).map(|c| Ok(c)).look_back(
    ///     3,
    ///     |c| *c,
    ///     |p_nth, c| p_nth == c,
    ///     |_, _, _| (),
    /// );
    /// 
    /// assert_eq!(iter.next(), Some(Ok('a')));
    /// assert_eq!(iter.next(), Some(Ok('b')));
    /// assert_eq!(iter.next(), Some(Ok('c')));
    /// assert_eq!(iter.next(), Some(Ok('a')));
    /// assert_eq!(iter.next(), Some(Ok('b')));
    /// assert_eq!(iter.next(), Some(Err(())));
    /// assert_eq!(iter.next(), Some(Err(())));
    /// assert_eq!(iter.next(), Some(Ok('c')));
    /// ```
    ///
    fn look_back(
        self,
        steps: usize,
        extractor: M,
        test: F,
        factory: Factory,
    ) -> LookBackIter<Self, T, E, A, M, F, Factory> {
        LookBackIter::new(self, steps, extractor, test, factory)
    }
}

impl<I, T, E, A, M, F, Factory> LookBack<T, E, A, M, F, Factory> for I
where
    I: Iterator<Item = Result<T, E>>,
    M: Fn(&T) -> A,
    F: Fn(&T, &A) -> bool,
    Factory: Fn(usize, T, &A) -> E,
{
}

#[cfg(test)]
mod tests {
    use crate::LookBack;

    #[derive(Debug, PartialEq)]
    enum TestErr<T> {
        LookBackFailed(usize, T, String),
        Is0Or3(T),
    }

    fn lbfailed<T, A>(err_index: usize, item: T, against: &A) -> TestErr<T>
    where
        A: std::fmt::Display,
    {
        TestErr::LookBackFailed(err_index, item, format!("{against}"))
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
                Err(TestErr::LookBackFailed(3, 2, "2".to_string())),
                Err(TestErr::LookBackFailed(4, 0, "2".to_string())),
                Err(TestErr::LookBackFailed(5, 1, "2".to_string())),
                Err(TestErr::LookBackFailed(6, 2, "2".to_string())),
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
            .look_back(1, |i| *i, |prev, i| prev == i, lbfailed)
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
        let results: Vec<Result<_, _>> = [0, 0, 1, 2, 0]
            .iter()
            .map(|i| Ok(i))
            .look_back(2, |i| **i, |prev, i| i == *prev, lbfailed)
            .collect();
        assert_eq!(
            results,
            [
                Ok(&0),
                Ok(&0),
                Err(TestErr::LookBackFailed(2, &1, "0".to_string())),
                Err(TestErr::LookBackFailed(3, &2, "0".to_string())),
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
                Err(TestErr::LookBackFailed(4, &1, "0".to_string())),
                Ok(&0),
                Ok(&1),
                Err(TestErr::LookBackFailed(7, &1, "0".to_string())),
                Ok(&0),
                Ok(&1),
            ]
        )
    }

    #[test]
    fn test_lookback_ignores_errors() {
        let results = (0..=5)
            .map(|i| {
                if i != 0 && i != 3 {
                    return Ok(i);
                } else {
                    return Err(TestErr::Is0Or3(i));
                }
            })
            .look_back(1, |i| i % 2, |j, parity| j % 2 != *parity, lbfailed)
            .collect::<Vec<_>>();

        assert_eq!(
            results,
            vec![
                Err(TestErr::Is0Or3(0)),
                Ok(1),
                Ok(2),
                Err(TestErr::Is0Or3(3)),
                Err(TestErr::LookBackFailed(4, 4, "0".to_string())),
                Ok(5)
            ]
        )
    }
}
