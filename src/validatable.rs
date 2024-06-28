use super::{valid_iter::ValidIter, valid_result::VResult};


/// The trait that allows sending iterators to the [`ValidIter`] type.
/// While it is not sealed, you should probably not implement it
/// unless you're feeling experimental.
///
/// When you use this trait, all [`Sized`] iterators have the method [`validate`](Unvalidatable::validate), and
/// can turn to [`ValidIter`] iterators.
///
pub trait Unvalidatable: Iterator + Sized {
    /// Turns an iterator over `T` into a [`ValidIter`] over [`VResult<T>`].
    ///
    /// In order to call validation adapters on an iterator, you must
    /// first call `validate`, because only a [`ValidIter`] can be validated.
    ///
    /// # Examples
    /// ```compile_fail
    /// // this does not compile
    /// let mut iter = (1..).at_least(3, "not enough!");
    /// ```
    /// ```
    /// // this compiles
    /// # use crate::validiter::{Unvalidatable, ValidIter, ValidErr};
    /// #
    /// let mut iter = (1..).validate().at_least(3, "not enough!");
    /// ```
    ///
    /// `validate` could technically be called on a [`ValidIter`] if
    /// you want to write some meta-validation:
    /// ```
    /// # use crate::validiter::Unvalidatable;
    /// #
    /// let mut meta_validiter = (1..)
    ///                             .validate()
    ///                             // ...validations
    ///                             .validate();
    ///
    /// assert_eq!(meta_validiter.next(), Some(Ok(Ok(1))));
    /// ```
    fn validate(self) -> Validatable<Self> {
        Validatable::new(self)
    }
}

impl<T> Unvalidatable for T where T: Iterator + Sized {}


/// The trait defining a validatable [`Iterator`]. For more information, see [`validate`](crate::Unvalidatable::validate)
#[derive(Debug, Clone)]
pub struct Validatable<I: Iterator> {
    iter: I,
}

impl<I: Iterator> Validatable<I> {
    pub(crate) fn new(iter: I) -> Validatable<I> {
        Self { iter }
    }
}

impl<I: Iterator> Iterator for Validatable<I> {
    type Item = VResult<I::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(val) => Some(Ok(val)),
            None => None,
        }
    }
}

impl<I: Iterator> ValidIter for Validatable<I> {
    type BaseType = I::Item;
}
