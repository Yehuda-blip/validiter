/// The [`Atleast`] ValidIter adapter, for more info see [`at_least`](crate::ValidIter::at_least).
#[derive(Debug, Clone)]
pub struct AtLeastIter<I, T, E, Factory>
where
    I: Iterator<Item = Result<T, E>>,
    Factory: Fn(usize) -> E,
{
    iter: I,
    min_count: usize,
    counter: usize,
    enumeration_counter: usize,
    factory: Factory,
}

impl<I, T, E, Factory> AtLeastIter<I, T, E, Factory>
where
    I: Iterator<Item = Result<T, E>>,
    Factory: Fn(usize) -> E,
{
    pub(crate) fn new(
        iter: I,
        min_count: usize,
        factory: Factory,
    ) -> AtLeastIter<I, T, E, Factory> {
        AtLeastIter {
            iter,
            min_count,
            counter: 0,
            enumeration_counter: 0,
            factory,
        }
    }
}

impl<I, T, E, Factory> Iterator for AtLeastIter<I, T, E, Factory>
where
    I: Iterator<Item = Result<T, E>>,
    Factory: Fn(usize) -> E,
{
    type Item = Result<T, E>;

    fn next(&mut self) -> Option<Self::Item> {
        let item = match self.iter.next() {
            Some(Ok(val)) => {
                self.counter += 1;
                Some(Ok(val))
            }
            None => match self.counter >= self.min_count {
                true => None,
                false => {
                    self.counter = self.min_count;
                    Some(Err((self.factory)(self.enumeration_counter)))
                }
            },
            other => other,
        };
        self.enumeration_counter += 1;
        item
    }
}

pub trait AtLeast<T, E, Factory>: Iterator<Item = Result<T, E>> + Sized
where
    Factory: Fn(usize) -> E,
{
    fn at_least(self, min_count: usize, factory: Factory) -> AtLeastIter<Self, T, E, Factory>;
}

impl<I, T, E, Factory> AtLeast<T, E, Factory> for I
where
    I: Iterator<Item = Result<T, E>>,
    Factory: Fn(usize) -> E,
{
    fn at_least(self, min_count: usize, factory: Factory) -> AtLeastIter<Self, T, E, Factory> {
        AtLeastIter::new(self, min_count, factory)
    }
}

#[cfg(test)]
mod tests {
    use crate::AtLeast;

    #[derive(Debug, PartialEq)]
    enum TestErr {
        NotEnough(usize),
        NotOdd(i32),
    }

    const fn not_enough(index: usize) -> TestErr {
        TestErr::NotEnough(index)
    }

    #[test]
    fn test_at_least_on_failure() {
        assert_eq!((0..10).map(|i| Ok(i)).at_least(100, not_enough).count(), 11);
        (0..10)
            .map(|i| Ok(i))
            .at_least(100, not_enough)
            .enumerate()
            .for_each(|(i, res_i)| match res_i {
                Ok(_) if i < 10 => {}
                Err(TestErr::NotEnough(len)) if i == 10 => {
                    assert_eq!(len, i)
                }
                _ => panic!("unexpected value in at least adapter"),
            })
    }

    #[test]
    fn test_at_least_on_success() {
        assert_eq!((0..10).map(|i| Ok(i)).at_least(5, not_enough).count(), 10);
        (0..10)
            .map(|i| Ok(i))
            .at_least(5, not_enough)
            .for_each(|res_i| match res_i {
                Ok(_) => {}
                _ => panic!("unexpected error in at least adapter"),
            })
    }

    #[test]
    fn test_at_least_successful_bounds() {
        let tightly_bound_success = (0..10)
            .map(|i| Ok(i))
            .at_least(10, not_enough)
            .collect::<Result<Vec<_>, _>>();
        assert!(matches!(tightly_bound_success, Ok(_)));

        let empty_success = (0..0)
            .map(|i| Ok(i))
            .at_least(0, not_enough)
            .collect::<Result<Vec<_>, _>>();
        assert!(matches!(empty_success, Ok(_)));
    }

    #[test]
    fn test_at_least_unsuccessful_bounds() {
        let tightly_bound_failure = (0..10)
            .map(|i| Ok(i))
            .at_least(11, not_enough)
            .collect::<Result<Vec<_>, _>>();
        match tightly_bound_failure {
            Ok(_) => panic!("collection should fail"),
            Err(TestErr::NotEnough(10)) => {}
            _ => panic!("bad variant"),
        }

        let empty_failure = (0..0)
            .map(|i| Ok(i))
            .at_least(1, not_enough)
            .collect::<Result<Vec<_>, _>>();
        match empty_failure {
            Ok(_) => panic!("collection should fail"),
            Err(TestErr::NotEnough(0)) => {}
            _ => panic!("bad variant"),
        }
    }

    #[test]
    fn test_at_least_all_elements_are_present_and_in_order_on_failure() {
        (0..10)
            .map(|i| Ok(i))
            .at_least(11, not_enough)
            .enumerate()
            .for_each(|(i, res_i)| match res_i {
                Ok(int) if int == i as i32 && i < 10 => {}
                Err(TestErr::NotEnough(10)) if i == 10 => {}
                _ => panic!("bad iteration after at least adapter failure"),
            })
    }

    #[test]
    fn test_at_least_all_elements_are_present_and_in_order_on_success() {
        (0..10)
            .map(|i| Ok(i))
            .at_least(10, not_enough)
            .enumerate()
            .for_each(|(i, res_i)| match res_i {
                Ok(int) if int == i as i32 && i < 10 => {}
                _ => panic!("bad iteration after at least adapter success"),
            })
    }

    #[test]
    fn test_at_least_does_not_validate_on_short_circuiting_before_last_element() {
        (0..10)
            .map(|i| Ok(i))
            .at_least(100, not_enough)
            .take(10)
            .for_each(|res_i| match res_i {
                Ok(_) => {}
                _ => panic!("failed the iteration when last error element was truncated"),
            })
    }

    #[test]
    fn test_at_least_validates_on_short_circuiting_after_last_element() {
        (0..10)
            .map(|i| Ok(i))
            .at_least(100, not_enough)
            .take(11)
            .enumerate()
            .for_each(|(i, res_i)| {
                match res_i {
                    Ok(_) if i < 10 => {},
                    Err(TestErr::NotEnough(10)) if i == 10 => {}
                    _ => panic!("did not fail the iteration in short circuit when last error element was not truncated")
                }
            })
    }

    #[test]
    fn test_at_least_counting_iterator_correctly_skips_errors() {
        let results = (0..1)
            .map(|i| {
                if i % 2 == 1 {
                    return Ok(i);
                } else {
                    Err(TestErr::NotOdd(i))
                }
            })
            .at_least(1, not_enough)
            .collect::<Vec<_>>();
        assert_eq!(
            results,
            vec![Err(TestErr::NotOdd(0)), Err(TestErr::NotEnough(1))]
        )
    }
}
