use crate::{valid_iter::ValidIter, valid_result::ValidErr};


/// The [`CastErrs`] ValidIter adapter, for more info see [`cast_errs`](crate::cast_errs).
#[derive(Debug, Clone)]
pub struct CastErrs<OkType, ErrType, I>
where
    I: Iterator<Item = Result<OkType, ValidErr<ErrType>>> + Sized,
{
    iter: I,
}

impl<OkType, ErrType, I> CastErrs<OkType, ErrType, I>
where
    I: Iterator<Item = Result<OkType, ValidErr<ErrType>>> + Sized,
{
    pub(crate) fn new(iter: I) -> Self {
        Self { iter }
    }
}

impl<OkType, ErrType, I> Iterator for CastErrs<OkType, ErrType, I>
where
    I: Iterator<Item = Result<OkType, ValidErr<ErrType>>> + Sized,
{
    type Item = Result<OkType, ValidErr<OkType>>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(Err(ValidErr::Description(desc))) => Some(Err(ValidErr::Description(desc))),
            Some(Err(ValidErr::WithElement(_, desc))) => Some(Err(ValidErr::Description(desc))),
            Some(Ok(ok_type)) => Some(Ok(ok_type)),
            None => None,
        }
    }
}

impl<OkType, ErrType, I> ValidIter for CastErrs<OkType, ErrType, I>
where
    I: Iterator<Item = Result<OkType, ValidErr<ErrType>>> + Sized,
{
    type BaseType = OkType;
}

/// The trait defining iterators that can be transformed into
/// a [`ValidIter`](ValidIter) without calling [`validate`](crate::Unvalidatable::validate). 
/// 
/// This trait was not written to be implemented, but is not sealed. If you want 
/// to allow converting some specific type to a [`ValidIter`], consider using this
/// trait.
pub trait ErrCastable<OkType, ErrType>:
    Iterator<Item = Result<OkType, ValidErr<ErrType>>> + Sized
{
    /// Turns an iterator over `Result<OkType, ValidErr<ErrType>>>`
    /// into a [`ValidIter`] over [`VResult<OkType>`](crate::valid_result::VResult) by dropping all
    /// the [`ValidErr<ErrType>`] elements and replacing them with
    /// [`ValidErr<OkType>::Casted`].
    ///
    /// `cast_errs` is useful in 2 scenarios:
    /// 1. When some opertion on the iterator
    /// causes a change in the underlying type of element (usually,
    /// collecting an iterator).
    /// 2. When as a result of err mapping, the underlying type is
    /// a [`VResult`](crate::valid_result::VResult) but the iterator itself is not a [`ValidIter`].
    ///
    /// It's purpose is similar to the [`validate`](crate::valid_iter::Unvalidatable::validate) method - sending
    /// an iterator to the [`ValidIter`] type.
    ///
    /// This would be better explained with an example:
    /// # Examples
    /// ```
    /// # use crate::validiter::{ValidIter, ErrCastable, ValidErr};
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
    ///                 .cast_errs()
    ///                 .ensure(|f| *f >= 0.0)
    ///                 .collect::<Result<Vec<f64>, ValidErr<f64>>>()
    ///             })
    ///             // OkType is a vector, but ErrType is f64!
    ///             .cast_errs()
    ///             .collect::<Result<Vec<_>, _>>(); // now ErrType is also a Vec<f64>
    ///
    /// assert_eq!(mat, Err(ValidErr::Casted)); // the element at pos [1][1] would have been negative, failing the ensure validation
    ///
    /// ```
    ///
    /// [`VResult<OkType>`](crate::valid_result::VResult)
    /// [`validate`](crate::valid_iter::Unvalidatable::validate)
    fn cast_errs(self) -> CastErrs<OkType, ErrType, Self> {
        CastErrs::new(self)
    }
}

impl<OkType, ErrType, I> ErrCastable<OkType, ErrType> for I where
    I: Iterator<Item = Result<OkType, ValidErr<ErrType>>> + Sized
{
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::{Unvalidatable, ValidIter, valid_result::ValidErr};

    use super::ErrCastable;

    // third line contains uppercase 'B'
    const TEST_STR: &str = "abcd
    abcd
    aBcd
    abcd";

    #[test]
    fn test_cast_allows_collection_of_validation_errors() {
        let error = TEST_STR
            .split_whitespace()
            .map(|line| {
                line.chars()
                    .validate()
                    .ensure(|c| c.is_lowercase(), "inner-ensure")
                    .collect::<Result<Vec<char>, _>>()
            })
            .cast_errs()
            .collect::<Result<Vec<Vec<char>>, _>>();
        assert_eq!(error, Err(ValidErr::Description(Rc::from("inner-ensure"))));
    }

    #[test]
    fn test_cast_after_error_handling_on_inner_level_allows_collecting_ok() {
        let ok = TEST_STR
            .split_whitespace()
            .map(|line| {
                line.chars()
                    .validate()
                    .ensure(|c| c.is_lowercase(), "inner-ensure")
                    .filter(|vec| vec.is_ok())
                    .collect::<Result<Vec<char>, _>>()
            })
            .cast_errs()
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
    fn test_cast_after_error_handling_on_outer_level_allows_collecting_ok() {
        let ok = TEST_STR
            .split_whitespace()
            .map(|line| {
                line.chars()
                    .validate()
                    .ensure(|c| c.is_lowercase(), "inner-ensure")
                    .collect::<Result<Vec<char>, _>>()
            })
            .cast_errs()
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
