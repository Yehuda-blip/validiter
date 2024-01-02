use crate::{valid_iter::ValidIter, valid_result::ValidErr};

pub struct ErrLift<OkType, ErrType, I>
where
    I: Iterator<Item = Result<OkType, ValidErr<ErrType>>> + Sized,
{
    iter: I,
}

impl<OkType, ErrType, I> ErrLift<OkType, ErrType, I>
where
    I: Iterator<Item = Result<OkType, ValidErr<ErrType>>> + Sized,
{
    pub(crate) fn new(iter: I) -> Self {
        Self { iter }
    }
}

impl<OkType, ErrType, I> Iterator for ErrLift<OkType, ErrType, I>
where
    I: Iterator<Item = Result<OkType, ValidErr<ErrType>>> + Sized,
{
    type Item = Result<OkType, ValidErr<OkType>>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(Err(_err_type)) => Some(Err(ValidErr::Lifted)),
            Some(Ok(ok_type)) => Some(Ok(ok_type)),
            None => None,
        }
    }
}

impl<OkType, ErrType, I> ValidIter for ErrLift<OkType, ErrType, I>
where
    I: Iterator<Item = Result<OkType, ValidErr<ErrType>>> + Sized,
{
    type BaseType = OkType;
}

pub trait ErrLiftable<OkType, ErrType>:
    Iterator<Item = Result<OkType, ValidErr<ErrType>>> + Sized
{
    fn err_lift(self) -> ErrLift<OkType, ErrType, Self> {
        ErrLift::new(self)
    }
}

impl<OkType, ErrType, I> ErrLiftable<OkType, ErrType> for I where
    I: Iterator<Item = Result<OkType, ValidErr<ErrType>>> + Sized
{
}

#[cfg(test)]
mod tests {
    use crate::{
        valid_iter::{Unvalidatable, ValidIter},
        valid_result::ValidErr,
    };

    use super::ErrLiftable;

    // third line contains uppercase 'B'
    const TEST_STR: &str = "abcd
    abcd
    aBcd
    abcd";

    #[test]
    fn test_lift_allows_collection_of_validation_errors() {
        let error = TEST_STR
            .split_whitespace()
            .map(|line| {
                line.chars()
                    .validate()
                    .ensure(|c| c.is_lowercase())
                    .collect::<Result<Vec<char>, _>>()
            })
            .err_lift()
            .collect::<Result<Vec<Vec<char>>, _>>();
        assert_eq!(error, Err(ValidErr::Lifted));
    }

    #[test]
    fn test_lift_after_error_handling_on_inner_level_allows_collecting_ok() {
        let ok = TEST_STR
            .split_whitespace()
            .map(|line| {
                line.chars()
                    .validate()
                    .ensure(|c| c.is_lowercase())
                    .filter(|vec| vec.is_ok())
                    .collect::<Result<Vec<char>, _>>()
            })
            .err_lift()
            .collect::<Result<Vec<Vec<char>>, _>>();
        assert_eq!(
            ok,
            Ok(vec![
                vec!['a', 'b', 'c', 'd'],
                vec!['a', 'b', 'c', 'd'],
                vec!['a', 'c', 'd'],
                vec!['a', 'b', 'c', 'd']
            ]),
        );
    }

    #[test]
    fn test_lift_after_error_handling_on_outer_level_allows_collecting_ok() {
        let ok = TEST_STR
            .split_whitespace()
            .map(|line| {
                line.chars()
                    .validate()
                    .ensure(|c| c.is_lowercase())
                    .collect::<Result<Vec<char>, _>>()
            })
            .err_lift()
            .filter(|vector| vector.is_ok())
            .collect::<Result<Vec<Vec<char>>, _>>();
        assert_eq!(
            ok,
            Ok(vec![
                vec!['a', 'b', 'c', 'd'],
                vec!['a', 'b', 'c', 'd'],
                vec!['a', 'b', 'c', 'd']
            ]),
        );
    }
}
