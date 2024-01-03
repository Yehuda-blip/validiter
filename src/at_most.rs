use crate::{valid_iter::ValidIter, valid_result::ValidErr};

use super::valid_result::VResult;

#[derive(Debug, Clone)]
pub struct AtMost<I>
where
    I: ValidIter + Iterator<Item = VResult<I::BaseType>>,
{
    iter: I,
    max_count: usize,
    counter: usize,
}

impl<I> AtMost<I>
where
    I: ValidIter + Iterator<Item = VResult<I::BaseType>>,
{
    pub(crate) fn new(iter: I, max_count: usize) -> AtMost<I>
    where
        I: Sized,
    {
        AtMost {
            iter,
            max_count,
            counter: 0,
        }
    }
}

impl<I> Iterator for AtMost<I>
where
    I: ValidIter + Iterator<Item = VResult<I::BaseType>>,
{
    type Item = VResult<I::BaseType>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(Ok(val)) => match self.counter >= self.max_count {
                true => Some(Err(ValidErr::TooMany(val))),
                false => {
                    self.counter += 1;
                    Some(Ok(val))
                }
            },
            other => other,
        }
    }
}

impl<I> ValidIter for AtMost<I>
where
    I: ValidIter + Iterator<Item = VResult<I::BaseType>>,
{
    type BaseType = I::BaseType;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::valid_iter::{Unvalidatable, ValidIter};

    #[test]
    fn test_at_most() {
        (0..10).validate().at_most(5).for_each(|res_i| match res_i {
            Ok(i) => assert!(i < 5),
            Err(err_i) => match err_i {
                ValidErr::TooMany(i) => assert!(i >= 5),
                _ => panic!("incorrect err for at most validator"),
            },
        })
    }

    #[test]
    fn test_at_most_has_correct_bounds() {
        let failed_collection = (0..10).validate().at_most(9).collect::<Result<Vec<_>, _>>();
        assert!(matches!(failed_collection, Err(ValidErr::TooMany(_))));

        let collection = (0..10)
            .validate()
            .at_most(10)
            .collect::<Result<Vec<_>, _>>();
        assert!(matches!(collection, Ok(_)));

        let empty_collection = (0..0).validate().at_most(0).collect::<Result<Vec<_>, _>>();
        assert!(matches!(empty_collection, Ok(_)));
    }

    #[test]
    fn test_at_most_all_elements_are_present_and_in_order() {
        (0..10)
            .validate()
            .at_most(5)
            .enumerate()
            .for_each(|(i, res_i)| match i < 5 {
                true => match res_i {
                    Ok(int) if int == i as i32 => {}
                    _ => panic!("bad match for item {}: {:?}", i, res_i),
                },
                false => match res_i {
                    Err(ValidErr::TooMany(int)) if int == i as i32 => {}
                    _ => panic!("bad match for item {}: {:?}", i, res_i),
                },
            })
    }

    #[test]
    fn test_at_most_by_ref() {
        [0, 1, 2, 3]
            .iter()
            .validate()
            .at_most(2)
            .enumerate()
            .for_each(|(i, res_i)| match i < 2 {
                true => assert!(matches!(res_i, Ok(_))),
                false => assert!(matches!(res_i, Err(ValidErr::TooMany(_)))),
            })
    }
}
