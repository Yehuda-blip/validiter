use std::iter::Enumerate;

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
    /// Applies a boolean test too each element, and fails the
    /// iteration if any element violates the constraint.
    ///
    /// `ensure(validation, factory)` is the general validation tool, it takes
    /// a boolean test as an argument and applies it to each of the
    /// elements in the iteration. If the test returns `true`, the element
    /// is wrapped in `Ok(element)`. Otherwise, `factory` gets called on it
    /// and the index of the error.
    ///
    /// Values already wrapped in `Result::Err` are ignored.
    ///
    /// # Examples
    ///
    /// Basic usage:
    /// ```
    /// use validiter::Ensure;
    /// #[derive(Debug, PartialEq)]
    /// struct Odd(usize, i32);
    /// let mut iter = (0..=3).map(|v| Ok(v)).ensure(|i| i % 2 == 0, |i, v| Odd(i, v));
    /// 
    /// assert_eq!(iter.next(), Some(Ok(0)));
    /// assert_eq!(iter.next(), Some(Err(Odd(1, 1))));
    /// assert_eq!(iter.next(), Some(Ok(2)));
    /// assert_eq!(iter.next(), Some(Err(Odd(3, 3))));
    /// ```
    ///
    /// You might want to chain `ensure` validations to create
    /// a more complex test:
    /// ```
    ///  # use validiter::Ensure;
    ///  # #[derive(Debug, PartialEq)]
    ///  enum IterError {
    ///     Odd,
    ///     NonPositive
    ///  }
    ///  
    ///  let mut iter = (0..=3)
    ///              .map(|v| Ok(v))
    ///              .ensure(|i| i % 2 == 0, |_, _| IterError::Odd)
    ///              .ensure(|i| *i > 0, |_, _| IterError::NonPositive);
    /// 
    ///  assert_eq!(iter.next(), Some(Err(IterError::NonPositive)));
    ///  assert_eq!(iter.next(), Some(Err(IterError::Odd)));
    ///  assert_eq!(iter.next(), Some(Ok(2)));
    ///  assert_eq!(iter.next(), Some(Err(IterError::Odd)));
    /// ```
    ///
    /// `ensure` ignores error elements:
    /// ```
    /// # use validiter::Ensure;
    /// 
    /// let mut iter = [Err(0)]
    ///                     .into_iter()
    ///                     .ensure(|i| *i == 0, |_, v| v);
    ///
    /// assert_eq!(iter.next(), Some(Err(0)));
    /// ```
    ///
    /// [`Err(ValidErr::Invalid(element))`](crate::valid_result::ValidErr)
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
