use crate::{
    at_least::AtLeast, between::Between, const_over::ConstOver, ensure::Ensure,
    look_back::LookBack, valid_result::VResult,
};

use super::{at_most::AtMost, validatable::Validatable};

pub trait Unvalidatable: Iterator + Sized {
    fn validate(self) -> Validatable<Self> {
        Validatable::new(self)
    }
}

impl<T> Unvalidatable for T where T: Iterator + Sized {}

pub trait ValidIter: Sized + Iterator<Item = VResult<Self::BaseType>> {
    type BaseType;

    /// Fails a validation iterator if it contains more than `n` elements.
    ///
    /// `at_most(n)` yeilds `Ok(element)` values until `n` elements are yielded,
    /// or the end of the iterator is reached. If values are still in the iteration,
    /// they will be wrapped in `Err(ValidErr::TooMany(element))`.
    ///
    /// Values of type `Err(ValidErr::<some valid err variant>` will not be
    /// counted towards reaching the `n` elements upper bound.
    ///
    /// # Examples
    ///
    /// Basic usage:
    /// ```
    /// # use crate::validiter::{valid_iter::{Unvalidatable, ValidIter}, valid_result::ValidErr};
    /// #
    /// let a = [1, 2, 3];
    /// let mut iter = a.iter().validate().at_most(2);
    ///
    /// assert_eq!(iter.next(), Some(Ok(&1)));
    /// assert_eq!(iter.next(), Some(Ok(&2)));
    /// assert_eq!(iter.next(), Some(Err(ValidErr::TooMany(&3))));
    /// ```
    ///
    /// Generally, `at_most` could be thought of as a not-quite-as-useful
    /// complement to the `at_least` adapter. It could also be used to ensure
    /// that collecting an iterator does not result in an unexpected amount
    /// of values in-memory:
    /// ```
    /// # use crate::validiter::{valid_iter::{Unvalidatable, ValidIter}, valid_result::ValidErr};
    /// #
    /// let mut collection_result: Result<Vec<_>, _> = (0..).take(1_000_000_000).validate().at_most(10).collect::<Result<_, _>>();
    ///
    /// assert_eq!(collection_result, Err(ValidErr::TooMany(10)));
    /// ```
    ///
    /// `at_most` will not account for validation errors already in the iteration:
    /// ```
    /// # use crate::validiter::{valid_iter::{Unvalidatable, ValidIter}, valid_result::ValidErr};
    /// #
    /// let mut iter = (-1..=3).validate().between(0, 10).at_most(4);
    ///
    /// assert_eq!(iter.next(), Some(Err(ValidErr::OutOfBounds(-1))));
    /// assert_eq!(iter.next(), Some(Ok(0)));
    /// assert_eq!(iter.next(), Some(Ok(1)));
    /// assert_eq!(iter.next(), Some(Ok(2)));
    /// assert_eq!(iter.next(), Some(Ok(3))); // the 5th element was not wrapped in Err()!
    /// ```
    fn at_most(self, n: usize) -> AtMost<Self> {
        AtMost::<Self>::new(self, n)
    }

    /// Fails a validation iterator if it does not contain `n` or more elements.
    ///
    /// `at_least(n)` yields `Ok(element)` values until the iteration ends. If the
    /// number of values in the iteration is less than `n`, a new element would be
    /// added to the end of the iteration with the value `Err(ValidErr::TooFew)`.
    ///
    /// The `at_least` adapter cannot handle short-circuiting of iterators, so
    /// iterations such as `(0..10).validate().at_least(100).take(5)` will not
    /// fail.
    ///
    /// Values of type `Err(ValidErr::<some valid err variant>)` will not be
    /// counted towards reaching the `n` elements lower bound.
    ///
    /// # Examples
    ///
    /// Basic usage:
    /// ```
    /// # use crate::validiter::{valid_iter::{Unvalidatable, ValidIter}, valid_result::ValidErr};
    /// #
    /// let a = [1, 2, 3];
    /// let mut iter = a.iter().validate().at_least(4);
    ///
    /// assert_eq!(iter.next(), Some(Ok(&1)));
    /// assert_eq!(iter.next(), Some(Ok(&2)));
    /// assert_eq!(iter.next(), Some(Ok(&3)));
    /// assert_eq!(iter.next(), Some(Err(ValidErr::TooFew)));
    /// assert_eq!(iter.next(), None);
    /// ```
    ///
    /// `at_least` could be used to ensure that a vector created from an iterator
    /// has a value in some index:
    /// ```
    /// # use crate::validiter::{valid_iter::{Unvalidatable, ValidIter}, valid_result::ValidErr};
    /// #
    /// let iter = (0..=2); // iteration is too short, no 4th element!
    ///
    /// let collection: Result<Vec<_>, _> = iter.validate().at_least(4).collect();
    ///
    /// match collection {
    ///     Ok(vec) => {let val = vec[3];}, // doesn't crash, because the collection failed.
    ///     Err(_) => {} // handle error
    /// };
    /// ```
    /// `at_least` will not account for validation errors already in the iteration:
    /// ```
    /// # use crate::validiter::{valid_iter::{Unvalidatable, ValidIter}, valid_result::ValidErr};
    /// #
    /// let mut iter = (0..=2).validate().between(1, 10).at_least(3);
    ///
    /// assert_eq!(iter.next(), Some(Err(ValidErr::OutOfBounds(0))));
    /// assert_eq!(iter.next(), Some(Ok(1)));
    /// assert_eq!(iter.next(), Some(Ok(2)));
    /// assert_eq!(iter.next(), Some(Err(ValidErr::TooFew))); // err element added, because the first element does not count.
    /// ```
    fn at_least(self, n: usize) -> AtLeast<Self> {
        AtLeast::<Self>::new(self, n)
    }

    /// Fails a validation iterator on `PartialOrd` elements if one the elements
    /// is out of the argument bounds.
    ///
    /// `between(lowest, highest)` wraps any value `val` which violates the constraint
    /// `lowest <= val && val <= highest` in a `Err(ValidErr::OutOfBounds(val))`.
    /// Otherwise, `Ok(val)` is yielded.
    ///
    /// Values of type `Err(ValidErr::<some valid err variant>)` are ignored.
    ///
    /// # Examples
    ///
    /// Basic usage:
    /// ```
    /// # use crate::validiter::{valid_iter::{Unvalidatable, ValidIter}, valid_result::ValidErr};
    /// #
    /// let a = [1, 2, 3, 4];
    /// let mut iter = a.iter().validate().between(&2, &3);
    ///
    /// assert_eq!(iter.next(), Some(Err(ValidErr::OutOfBounds(&1))));
    /// assert_eq!(iter.next(), Some(Ok(&2)));
    /// assert_eq!(iter.next(), Some(Ok(&3)));
    /// assert_eq!(iter.next(), Some(Err(ValidErr::OutOfBounds(&4))));
    /// ```
    ///
    /// Partial-Equality is also supported:
    /// ```
    /// # use crate::validiter::{valid_iter::{Unvalidatable, ValidIter}, valid_result::ValidErr};
    /// #
    /// let a = [f64::NAN];
    /// let mut iter = a.iter().validate().between(&2.0, &3.0);
    ///
    /// // we can't compare a NaN, so we'll pattern-match:
    /// match iter.next() {
    ///     // this is the value we get
    ///     Some(Err(ValidErr::OutOfBounds(val))) => {assert!(val.is_nan());}
    ///
    ///    // won't happen, '&f64::NAN' violates '&2.0 <= val && val <= &3.0'
    ///     Some(Ok(_)) => {panic!()}
    ///
    ///    // also won't happen - the next yield is some out-of-bounds err
    ///     _ => {panic!()}
    /// }
    /// ```
    fn between(self, lower_bound: Self::BaseType, upper_bound: Self::BaseType) -> Between<Self>
    where
        Self::BaseType: PartialOrd,
    {
        Between::<Self>::new(self, lower_bound, upper_bound)
    }

    /// Applies a closure constraint too each element, and fails the
    /// iteration if any element violates the constraint.
    ///
    /// `ensure(validation)` is the general validation tool, it takes
    /// a boolean test as an argument and applies it to each of the
    /// elements in the iteration. If the test returns `true`, the element
    /// is wrapped in `Ok(element)`. Otherwise, it is wrapped in
    /// `Err(ValidErr::Invalid(element))`.
    ///
    /// Values of type `Err(ValidErr::<some valid err variant>)` are ignored.
    ///
    /// # Examples
    ///
    /// Basic usage:
    /// ```
    /// # use crate::validiter::{valid_iter::{Unvalidatable, ValidIter}, valid_result::ValidErr};
    /// #
    /// let mut iter = (0..=3).validate().ensure(|i| i % 2 == 0);
    ///
    /// assert_eq!(iter.next(), Some(Ok(0)));
    /// assert_eq!(iter.next(), Some(Err(ValidErr::Invalid(1))));
    /// assert_eq!(iter.next(), Some(Ok(2)));
    /// assert_eq!(iter.next(), Some(Err(ValidErr::Invalid(3))));
    /// ```
    ///
    /// You might want to chain `ensure` validations to create
    /// a more complex test:
    /// ```
    /// # use crate::validiter::{valid_iter::{Unvalidatable, ValidIter}, valid_result::ValidErr};
    /// #
    /// let mut iter = (0..=3)
    ///             .validate()
    ///             .ensure(|i| i % 2 == 0)
    ///             .ensure(|i| *i > 0);
    ///
    /// assert_eq!(iter.next(), Some(Err(ValidErr::Invalid(0))));
    /// assert_eq!(iter.next(), Some(Err(ValidErr::Invalid(1))));
    /// assert_eq!(iter.next(), Some(Ok(2)));
    /// assert_eq!(iter.next(), Some(Err(ValidErr::Invalid(3))));
    /// ```
    /// 
    /// `ensure` ignores error elements:
    /// ```
    /// # use crate::validiter::{valid_iter::{Unvalidatable, ValidIter}, valid_result::ValidErr};
    /// #
    /// let mut iter = (0..=3).validate().between(2, 3).ensure(|i| i % 2 == 0);
    ///
    /// assert_eq!(iter.next(), Some(Err(ValidErr::OutOfBounds(0))));
    /// assert_eq!(iter.next(), Some(Err(ValidErr::OutOfBounds(1)))); // invalid, but not tested
    /// assert_eq!(iter.next(), Some(Ok(2)));
    /// assert_eq!(iter.next(), Some(Err(ValidErr::Invalid(3))));
    /// ```
    ///
    fn ensure<F>(self, validation: F) -> Ensure<Self, F>
    where
        F: FnMut(&Self::BaseType) -> bool,
    {
        Ensure::<Self, F>::new(self, validation)
    }

    /// Tests each element in the iteration based on the previous element.
    /// 
    /// `look_back(extractor, validation)` is sugar for calling 
    /// `look_back_n<1, _, _, _>::(extractor, validation)`. It takes 
    /// 2 closure arguments:
    /// 1. extractor - a mapping of iterator elements to some extracted
    /// value.
    /// 2. validation - a test which accepts the value extracted from 
    /// the previous element, and tests the current element based on
    /// this value.
    /// 
    /// Elements which fail the `validation` test will be wrapped in
    /// `Err(ValidErr::LookBackFailed(element))`.
    /// 
    /// Examples:
    /// 
    /// Basic usage:
    /// ```
    /// # use crate::validiter::{valid_iter::{Unvalidatable, ValidIter}, valid_result::ValidErr};
    /// #
    /// // is the iteration ordered?
    /// let mut iter = (0..=2).chain(1..=1).validate().look_back(|i| *i, |prev, i| prev <= i);
    /// 
    /// assert_eq!(iter.next(), Some(Ok(0))); // first value is never tested
    /// assert_eq!(iter.next(), Some(Ok(1)));
    /// assert_eq!(iter.next(), Some(Ok(2)));
    /// assert_eq!(iter.next(), Some(Err(ValidErr::LookBackFailed(1))));
    /// ```
    /// 
    /// Or maybe a slightly more exotic test:
    /// ```
    /// # use crate::validiter::{valid_iter::{Unvalidatable, ValidIter}, valid_result::ValidErr};
    /// # use std::iter::repeat;
    /// #
    /// // Does the iteration converge?
    /// const EPSILON: f64 = 0.0001;
    /// let mut iter = (0..).map(|i| (-1_f64).powi(i) / 2_f64.powi(i))
    ///                     .validate()
    ///                     .look_back(|i| i.abs(), |prev, i| prev * (1.0 - EPSILON) >= *i )
    ///                     .take(4);
    /// 
    /// assert_eq!(iter.next(), Some(Ok(1.0)));
    /// assert_eq!(iter.next(), Some(Ok(-1.0 / 2.0)));
    /// assert_eq!(iter.next(), Some(Ok(1.0 / 4.0)));
    /// assert_eq!(iter.next(), Some(Ok(-1.0 / 8.0)));
    /// ```
    fn look_back<A, M, F>(self, extractor: M, validation: F) -> LookBack<Self, A, M, F, 1>
    where
        A: Default,
        M: FnMut(&Self::BaseType) -> A,
        F: FnMut(&A, &Self::BaseType) -> bool,
    {
        LookBack::new(self, extractor, validation)
    }


    fn look_back_n<const N: usize, A, M, F>(
        self,
        extractor: M,
        validation: F,
    ) -> LookBack<Self, A, M, F, N>
    where
        A: Default,
        M: FnMut(&Self::BaseType) -> A,
        F: FnMut(&A, &Self::BaseType) -> bool,
    {
        LookBack::new(self, extractor, validation)
    }

    fn const_over<A, M>(self, extractor: M) -> ConstOver<Self, A, M>
    where
        A: PartialEq,
        M: FnMut(&Self::BaseType) -> A,
    {
        ConstOver::new(self, extractor)
    }
}
