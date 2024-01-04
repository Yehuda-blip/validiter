use crate::{valid_iter::ValidIter, valid_result::ValidErr};

#[derive(Debug, Clone)]
pub struct LiftErrs<OkType, ErrType, I>
where
    I: Iterator<Item = Result<OkType, ValidErr<ErrType>>> + Sized,
{
    iter: I,
}

impl<OkType, ErrType, I> LiftErrs<OkType, ErrType, I>
where
    I: Iterator<Item = Result<OkType, ValidErr<ErrType>>> + Sized,
{
    pub(crate) fn new(iter: I) -> Self {
        Self { iter }
    }
}

impl<OkType, ErrType, I> Iterator for LiftErrs<OkType, ErrType, I>
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

impl<OkType, ErrType, I> ValidIter for LiftErrs<OkType, ErrType, I>
where
    I: Iterator<Item = Result<OkType, ValidErr<ErrType>>> + Sized,
{
    type BaseType = OkType;
}

pub trait ErrLiftable<OkType, ErrType>:
    Iterator<Item = Result<OkType, ValidErr<ErrType>>> + Sized
{
    /// Turns an iterator over `Result<OkType, ValidErr<ErrType>>>`
    /// into a `ValidIter` over `VResult<OkType>` by dropping all
    /// the `ValidErr<ErrType>` elements and replacing them with
    /// `ValidErr<OkType>::Lifted`.
    ///
    /// `lift_errs` is useful in 2 scenarios:
    /// 1. When some opertion on the iterator
    /// causes a change in the underlying type of element (usually,
    /// collecting an iterator).
    /// 2. When as a result of err mapping, the underlying type is
    /// a `VResult` but the iterator itself is not a `ValidIter`.
    ///
    /// It's purpose is similar to the `validate` method - sending
    /// an iterator to the `ValidIter` type.
    ///
    /// This would be better explained with an example:
    /// # Examples
    /// ```
    /// # use crate::validiter::{valid_iter::ValidIter, lift_errs::ErrLiftable, valid_result::ValidErr};
    /// #
    /// // is this csv a matrix of positive values?
    /// let csv = "1.2, 3.0
    ///            4.2, -0.5";
    /// let mat = csv
    ///             .lines()
    ///             .map( |line| {
    ///                 line.split(",")
    ///                 .map(|s| s.trim())
    ///                 .map(|s| s.parse::<f64>().map_err(|_| ValidErr::<f64>::Mapped))
    ///                 // the iterator is over VResult<f64>, but map is not a ValidIter!
    ///                 .lift_errs()
    ///                 .ensure(|f| *f >= 0.0)
    ///                 .collect::<Result<Vec<f64>, ValidErr<f64>>>()
    ///             })
    ///             // OkType is a vector, but ErrType is f64!
    ///             .lift_errs()
    ///             .collect::<Result<Vec<_>, _>>(); // now ErrType is also a Vec<f64>
    ///
    /// assert_eq!(mat, Err(ValidErr::Lifted)); // the element at pos [1][1] would have been negative, failing the ensure validation
    ///
    /// ```
    fn lift_errs(self) -> LiftErrs<OkType, ErrType, Self> {
        LiftErrs::new(self)
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
            .lift_errs()
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
            .lift_errs()
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
            .lift_errs()
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
