/// The [`AtMost`] ValidIter adapter, for more info see [`at_most`](crate::ValidIter::at_most).
#[derive(Debug, Clone)]
struct AtMostIter<I, T, E>
where
    I: Iterator<Item = Result<T, E>>,
{
    iter: I,
    max_count: usize,
    counter: usize,
    factory: fn(T) -> E,
}

impl<I, T, E> AtMostIter<I, T, E>
where
    I: Iterator<Item = Result<T, E>>,
{
    pub(crate) fn new(iter: I, max_count: usize, factory: fn(T) -> E) -> AtMostIter<I, T, E> {
        AtMostIter {
            iter,
            max_count,
            counter: 0,
            factory,
        }
    }
}

impl<I, T, E> Iterator for AtMostIter<I, T, E>
where
    I: Iterator<Item = Result<T, E>>,
{
    type Item = Result<T, E>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(Ok(val)) => match self.counter >= self.max_count {
                true => Some(Err((self.factory)(val))),
                false => {
                    self.counter += 1;
                    Some(Ok(val))
                }
            },
            other => other,
        }
    }
}

pub trait AtMost<T, E>: Iterator<Item = Result<T, E>> + Sized {
    fn at_most(self, min_count: usize, factory: fn(T) -> E) -> AtMostIter<Self, T, E>;
}

impl<I, T, E> AtMost<T, E> for I
where
    I: Iterator<Item = Result<T, E>>,
{
    fn at_most(self, min_count: usize, factory: fn(T) -> E) -> AtMostIter<Self, T, E> {
        AtMostIter::new(self, min_count, factory)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq)]
    enum TestErr<T> {
        TooMany(T),
        IsOdd(T),
    }

    const fn too_many<T>(item: T) -> TestErr<T> {
        TestErr::TooMany(item)
    }

    #[test]
    fn test_at_most() {
        (0..10)
            .map(|i| Ok(i))
            .at_most(5, too_many)
            .for_each(|res_i| match res_i {
                Ok(i) => assert!(i < 5),
                Err(TestErr::TooMany(i)) => assert!(i >= 5),
                e => panic!("bad error for too many {e:?}"),
            })
    }

    #[test]
    fn test_at_most_has_correct_bounds() {
        let failed_collection = (0..10)
            .map(|i| Ok(i))
            .at_most(9, too_many)
            .collect::<Result<Vec<_>, _>>();
        assert!(matches!(failed_collection, Err(TestErr::TooMany(9))));

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
                    Err(TestErr::TooMany(int)) if int == i as i32 => {}
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
                false => assert!(matches!(res_i, Err(TestErr::TooMany(_)))),
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
                Err(TestErr::TooMany(4))
            ]
        )
    }
}
