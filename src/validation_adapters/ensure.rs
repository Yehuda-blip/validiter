use std::iter::Enumerate;

/// The [`Ensure`] ValidIter adapter, for more info see [`ensure`](crate::ValidIter::ensure).
#[derive(Debug, Clone)]
pub struct EnsureIter<I, T, E, F, Factory>
where
    I: Iterator<Item = Result<T, E>>,
    F: Fn(&T) -> bool,
    Factory: Fn(usize, T) -> E,
{
    iter: Enumerate<I>,
    validation: F,
    factory: Factory,
}

impl<I, T, E, F, Factory> EnsureIter<I, T, E, F, Factory>
where
    I: Iterator<Item = Result<T, E>>,
    F: Fn(&T) -> bool,
    Factory: Fn(usize, T) -> E,
{
    pub(crate) fn new(iter: I, validation: F, factory: Factory) -> EnsureIter<I, T, E, F, Factory> {
        EnsureIter {
            iter: iter.enumerate(),
            validation,
            factory,
        }
    }
}

impl<I, T, E, F, Factory> Iterator for EnsureIter<I, T, E, F, Factory>
where
    I: Iterator<Item = Result<T, E>>,
    F: Fn(&T) -> bool,
    Factory: Fn(usize, T) -> E,
{
    type Item = Result<T, E>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some((i, Ok(val))) => match (self.validation)(&val) {
                true => Some(Ok(val)),
                false => Some(Err((self.factory)(i, val))),
            },
            Some((_, err)) => Some(err),
            None => None,
        }
    }
}

pub trait Ensure<T, E, F, Factory>: Iterator<Item = Result<T, E>> + Sized
where
    F: Fn(&T) -> bool,
    Factory: Fn(usize, T) -> E,
{
    fn ensure(self, test: F, factory: Factory) -> EnsureIter<Self, T, E, F, Factory> {
        EnsureIter::new(self, test, factory)
    }
}

impl<I, T, E, F, Factory> Ensure<T, E, F, Factory> for I
where
    I: Iterator<Item = Result<T, E>>,
    F: Fn(&T) -> bool,
    Factory: Fn(usize, T) -> E,
{
}

#[cfg(test)]
mod tests {
    use super::Ensure;

    #[derive(Debug, PartialEq)]
    enum TestErr {
        IsOdd(usize, i32),
        Err1(usize, i32),
        Err2(usize, i32),
    }

    #[test]
    fn test_ensure() {
        (0..10)
            .map(|v| Ok(v))
            .ensure(|i| i % 2 == 0, |err_index, i| TestErr::IsOdd(err_index, i))
            .enumerate()
            .for_each(|(i, res_i)| match res_i {
                Ok(int) if i % 2 == 0 && i as i32 == int => {}
                Err(TestErr::IsOdd(i, v)) if v % 2 == 1 && i as i32 == v => {}
                _ => panic!("unexpected value in ensure adapter"),
            })
    }

    #[test]
    fn test_ensure_ignores_errors() {
        let v = (0..=0)
            .map(|v| Ok(v))
            .ensure(|i| *i != 0, |err_index, v| TestErr::Err1(err_index, v))
            .ensure(|i| *i != 0, |err_index, v| TestErr::Err2(err_index, v))
            .next();
        assert_eq!(v, Some(Err(TestErr::Err1(0, 0))))
    }
}
