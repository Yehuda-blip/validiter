use std::iter::Enumerate;

/// The [`AtMost`] ValidIter adapter, for more info see [`at_most`](crate::ValidIter::at_most).
#[derive(Debug, Clone)]
pub struct AtMostIter<I, T, E, Factory>
where
    I: Iterator<Item = Result<T, E>>,
    Factory: Fn(usize, T) -> E,
{
    iter: Enumerate<I>,
    max_count: usize,
    counter: usize,
    factory: Factory,
}

impl<I, T, E, Factory> AtMostIter<I, T, E, Factory>
where
    I: Iterator<Item = Result<T, E>>,
    Factory: Fn(usize, T) -> E,
{
    pub(crate) fn new(iter: I, max_count: usize, factory: Factory) -> AtMostIter<I, T, E, Factory> {
        AtMostIter {
            iter: iter.enumerate(),
            max_count,
            counter: 0,
            factory,
        }
    }
}

impl<I, T, E, Factory> Iterator for AtMostIter<I, T, E, Factory>
where
    I: Iterator<Item = Result<T, E>>,
    Factory: Fn(usize, T) -> E,
{
    type Item = Result<T, E>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some((i, Ok(val))) => match self.counter >= self.max_count {
                true => Some(Err((self.factory)(i, val))),
                false => {
                    self.counter += 1;
                    Some(Ok(val))
                }
            },
            Some((_, Err(err))) => Some(Err(err)),
            None => None,
        }
    }
}

pub trait AtMost<T, E, Factory>: Iterator<Item = Result<T, E>> + Sized
where
    Factory: Fn(usize, T) -> E,
{
    fn at_most(self, min_count: usize, factory: Factory) -> AtMostIter<Self, T, E, Factory>;
}

impl<I, T, E, Factory> AtMost<T, E, Factory> for I
where
    I: Iterator<Item = Result<T, E>>,
    Factory: Fn(usize, T) -> E,
{
    fn at_most(self, min_count: usize, factory: Factory) -> AtMostIter<Self, T, E, Factory> {
        AtMostIter::new(self, min_count, factory)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq)]
    enum TestErr<T> {
        TooMany(usize, T),
        IsOdd(T),
    }

    const fn too_many<T>(violating_index: usize, item: T) -> TestErr<T> {
        TestErr::TooMany(violating_index, item)
    }

    #[test]
    fn test_at_most() {
        (0..10)
            .map(|i| Ok(i))
            .at_most(5, too_many)
            .for_each(|res_i| match res_i {
                Ok(i) => assert!(i < 5),
                Err(TestErr::TooMany(i, v)) => {
                    assert_eq!(v as usize, i);
                    assert!(i >= 5)
                }
                e => panic!("bad error for too many {e:?}"),
            })
    }

    #[test]
    fn test_at_most_has_correct_bounds() {
        let failed_collection = (0..10)
            .map(|i| Ok(i))
            .at_most(9, too_many)
            .collect::<Result<Vec<_>, _>>();
        assert!(matches!(failed_collection, Err(TestErr::TooMany(9, 9))));

        let collection = (0..10)
            .map(|i| Ok(i))
            .at_most(10, too_many)
            .collect::<Result<Vec<_>, _>>();
        assert!(matches!(collection, Ok(_)));

        let empty_collection = (0..0)
            .map(|i| Ok(i))
            .at_most(0, too_many)
            .collect::<Result<Vec<_>, _>>();
        assert!(matches!(empty_collection, Ok(_)));
    }

    #[test]
    fn test_at_most_all_elements_are_present_and_in_order() {
        (0..10)
            .map(|i| Ok(i))
            .at_most(5, too_many)
            .enumerate()
            .for_each(|(i, res_i)| match i < 5 {
                true => match res_i {
                    Ok(int) if int == i as i32 => {}
                    _ => panic!("bad match for item {}: {:?}", i, res_i),
                },
                false => match res_i {
                    Err(TestErr::TooMany(_, int)) if int == i as i32 => {}
                    _ => panic!("bad match for item {}: {:?}", i, res_i),
                },
            })
    }

    #[test]
    fn test_at_most_by_ref() {
        [0, 1, 2, 3]
            .iter()
            .map(|i| Ok(i))
            .at_most(2, too_many)
            .enumerate()
            .for_each(|(i, res_i)| match i < 2 {
                true => assert!(matches!(res_i, Ok(_))),
                false => assert!(matches!(res_i, Err(TestErr::TooMany(_, _)))),
            })
    }

    #[test]
    fn test_at_most_counting_validator_correctly_skips_errors() {
        let results = (0..5)
            .map(|i| {
                if i % 2 == 0 {
                    return Ok(i);
                } else {
                    return Err(TestErr::IsOdd(i));
                }
            })
            .at_most(2, too_many)
            .collect::<Vec<_>>();
        assert_eq!(
            results,
            vec![
                Ok(0),
                Err(TestErr::IsOdd(1)),
                Ok(2),
                Err(TestErr::IsOdd(3)),
                Err(TestErr::TooMany(4, 4))
            ]
        )
    }
}
