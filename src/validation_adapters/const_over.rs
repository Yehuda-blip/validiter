use std::iter::Enumerate;

/// The [`ConstOver`] ValidIter adapter, for more info see [`const_over`](crate::ValidIter::const_over).
#[derive(Debug, Clone)]
pub struct ConstOverIter<I, T, E, A, M, Factory>
where
    I: Iterator<Item = Result<T, E>>,
    A: PartialEq,
    M: Fn(&T) -> A,
    Factory: Fn(usize, T, A, &A) -> E,
{
    iter: Enumerate<I>,
    stored_value: Option<A>,
    extractor: M,
    factory: Factory,
}

impl<I, T, E, A, M, Factory> ConstOverIter<I, T, E, A, M, Factory>
where
    I: Iterator<Item = Result<T, E>>,
    A: PartialEq,
    M: Fn(&T) -> A,
    Factory: Fn(usize, T, A, &A) -> E,
{
    pub(crate) fn new(
        iter: I,
        extractor: M,
        factory: Factory,
    ) -> ConstOverIter<I, T, E, A, M, Factory> {
        Self {
            iter: iter.enumerate(),
            stored_value: None,
            extractor,
            factory,
        }
    }
}

impl<I, T, E, A, M, Factory> Iterator for ConstOverIter<I, T, E, A, M, Factory>
where
    I: Iterator<Item = Result<T, E>>,
    A: PartialEq,
    M: Fn(&T) -> A,
    Factory: Fn(usize, T, A, &A) -> E,
{
    type Item = Result<T, E>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some((i, Ok(val))) => {
                let extraction = (self.extractor)(&val);
                match &self.stored_value {
                    Some(expected_const) => match extraction == *expected_const {
                        true => Some(Ok(val)),
                        false => Some(Err((self.factory)(i, val, extraction, expected_const))),
                    },
                    None => {
                        self.stored_value = Some(extraction);
                        Some(Ok(val))
                    }
                }
            }
            Some((_, Err(e))) => Some(Err(e)),
            None => None,
        }
    }
}

pub trait ConstOver<T, E, A, M, Factory>: Iterator<Item = Result<T, E>> + Sized
where
    A: PartialEq,
    M: Fn(&T) -> A,
    Factory: Fn(usize, T, A, &A) -> E,
{
    fn const_over(self, extractor: M, factory: Factory)
        -> ConstOverIter<Self, T, E, A, M, Factory>;
}

impl<I, T, E, A, M, Factory> ConstOver<T, E, A, M, Factory> for I
where
    I: Iterator<Item = Result<T, E>>,
    A: PartialEq,
    M: Fn(&T) -> A,
    Factory: Fn(usize, T, A, &A) -> E,
{
    fn const_over(
        self,
        extractor: M,
        factory: Factory,
    ) -> ConstOverIter<Self, T, E, A, M, Factory> {
        ConstOverIter::new(self, extractor, factory)
    }
}

#[cfg(test)]
mod tests {
    use std::iter::repeat;

    use crate::ConstOver;

    #[derive(Debug, PartialEq)]
    enum TestErr<T, A>
    where
        A: std::fmt::Display,
    {
        BrokenConst(usize, T, A, String),
        Not0Or2(T),
    }

    fn broken_const<T, A>(index: usize, item: T, eval: A, expected: &A) -> TestErr<T, A>
    where
        A: std::fmt::Display,
    {
        TestErr::BrokenConst(index, item, eval, format!("{expected}"))
    }

    #[test]
    fn test_const_over_ok() {
        if repeat(1)
            .take(5)
            .map(|i| Ok(i))
            .const_over(|i| *i, broken_const)
            .any(|res| res.is_err())
        {
            panic!("const over failed on constant iteration")
        }
    }

    #[test]
    fn test_const_over_err() {
        let results: Vec<_> = [0, 0, 0, 1]
            .into_iter()
            .map(|i| Ok(i))
            .const_over(|i| *i, broken_const)
            .collect();
        assert_eq!(
            results,
            [
                Ok(0),
                Ok(0),
                Ok(0),
                Err(TestErr::BrokenConst(3, 1, 1, "0".to_string()))
            ]
        )
    }

    #[test]
    fn test_const_over_bounds() {
        if (0..0)
            .map(|i| Ok(i))
            .const_over(|i| *i, broken_const)
            .any(|res| res.is_err())
        {
            panic!("const over failed on empty iter")
        }

        if (0..1)
            .map(|i| Ok(i))
            .const_over(|i| *i, broken_const)
            .any(|res| res.is_err())
        {
            panic!("const over failed on count == 1 iter")
        }
    }

    #[test]
    fn test_const_over_all_elements_are_present_and_in_order() {
        let results: Vec<_> = [[0], [0], [0], [1], [0], [2]]
            .into_iter()
            .map(|i| Ok(i))
            .const_over(|slice| slice[0], broken_const)
            .collect();
        assert_eq!(
            results,
            [
                Ok([0]),
                Ok([0]),
                Ok([0]),
                Err(TestErr::BrokenConst(3, [1], 1, "0".to_string())),
                Ok([0]),
                Err(TestErr::BrokenConst(5, [2], 2, "0".to_string()))
            ]
        )
    }

    #[test]
    fn test_const_over_ignores_errors() {
        let results = (0..=4)
            .map(|i| {
                if i != 0 && i != 2 {
                    return Ok(i);
                } else {
                    return Err(TestErr::Not0Or2(i));
                }
            })
            .const_over(|i| i % 2, broken_const)
            .collect::<Vec<_>>();

        assert_eq!(
            results,
            vec![
                Err(TestErr::Not0Or2(0)),
                Ok(1),
                Err(TestErr::Not0Or2(2)),
                Ok(3),
                Err(TestErr::BrokenConst(4, 4, 0, "1".to_string()))
            ]
        )
    }
}
