use crate::{
    // valid_result::VResult,
    validation_adapters::at_most::AtMost, ValidErr, WithElement,
};

/// The trait defining a validatable iterator. While it is not sealed,
/// you should probably not implement it unless you're feeling experimental.
pub trait ValidIter<E>: Sized + Iterator<Item = Result<Self::BaseType, E>>
where E: ValidErr<Self::BaseType> {
    type BaseType;

    /// Fails a validation iterator if it contains more than `n` elements.
    ///
    /// `at_most(n, description)` yields `Ok(element)` values until `n` elements are yielded,
    /// or the end of the iterator is reached. If values are still in the iteration,
    /// they will be wrapped in `Err(ValidErr::WithElement(element, Rc(description)))`.
    ///
    /// Elements already wrapped in `Err(ValidErr::<some valid err variant>)` will not be
    /// counted towards reaching the `n` elements upper bound.
    ///
    /// # Examples
    ///
    /// Basic usage:
    /// ```
    /// # use std::rc::Rc;
    /// # use crate::validiter::{Unvalidatable, ValidIter, ValidErr};
    /// #
    /// let a = [1, 2, 3];
    /// let mut iter = a.iter().validate().at_most(2, "no more than 2");
    ///
    /// assert_eq!(iter.next(), Some(Ok(&1)));
    /// assert_eq!(iter.next(), Some(Ok(&2)));
    /// assert_eq!(iter.next(), Some(Err(ValidErr::WithElement(&3, Rc::from("no more than 2")))));
    /// ```
    ///
    /// Generally, `at_most` could be thought of as a not-quite-as-useful
    /// complement to the [`at_least`](crate::ValidIter::at_least) adapter. It could also be used to ensure
    /// that collecting an iterator does not result in an unexpected amount
    /// of values in-memory:
    /// ```
    /// # use std::rc::Rc;
    /// # use crate::validiter::{Unvalidatable, ValidIter, ValidErr};
    /// #
    ///     let collection_result: Result<Vec<_>, _> = (0..)
    ///         .take(1_000_000_000)
    ///         .validate()
    ///         .at_most(10, "not a lot")
    ///         .collect::<Result<_, _>>();
    ///
    ///     assert_eq!(collection_result, Err(ValidErr::WithElement(10, Rc::from("not a lot"))));   
    /// ```
    ///
    /// `at_most` will not account for validation errors already in the iteration:
    /// ```
    /// # use std::rc::Rc;
    /// # use crate::validiter::{Unvalidatable, ValidIter, ValidErr};
    /// #
    /// let mut iter = (-1..=3).validate().between(0, 10, "out of bounds!").at_most(4, "too many!");
    ///
    /// assert_eq!(iter.next(), Some(Err(ValidErr::WithElement(-1, Rc::from("out of bounds!")))));
    /// assert_eq!(iter.next(), Some(Ok(0)));
    /// assert_eq!(iter.next(), Some(Ok(1)));
    /// assert_eq!(iter.next(), Some(Ok(2)));
    /// assert_eq!(iter.next(), Some(Ok(3))); // the 5th element was not wrapped in Err()!
    /// ```
    ///
    /// [`Err(ValidErr::TooMany(element))`](crate::valid_result::ValidErr)
    ///
    fn at_most<W: WithElement<Self::BaseType, E>>(self, n: usize) -> AtMost<Self, E, W> {
        AtMost::<Self, E, W>::new(self, n)
    }

    // /// Fails a validation iterator if it does not contain `n` or more elements.
    // ///
    // /// `at_least(n, description)` yields `Ok(element)` values until the iteration ends. If the
    // /// number of values in the iteration is less than `n`, a new element is
    // /// added to the end of the iteration with the value `Err(ValidErr::Description(Rc(description)))`.
    // ///
    // /// The `at_least` adapter cannot handle short-circuiting of iterators, so
    // /// iterations such as `(0..10).validate().at_least(100).take(5)` will not
    // /// fail.
    // ///
    // /// Elements already wrapped in `Err(ValidErr::<some valid err variant>)` will not be
    // /// counted towards reaching the `n` elements lower bound.
    // ///
    // /// # Examples
    // ///
    // /// Basic usage:
    // /// ```
    // /// # use std::rc::Rc;
    // /// # use crate::validiter::{Unvalidatable, ValidIter, ValidErr};
    // /// #
    // /// let a = [1, 2, 3];
    // /// let mut iter = a.iter().validate().at_least(4, "not enough!");
    // ///
    // /// assert_eq!(iter.next(), Some(Ok(&1)));
    // /// assert_eq!(iter.next(), Some(Ok(&2)));
    // /// assert_eq!(iter.next(), Some(Ok(&3)));
    // /// assert_eq!(iter.next(), Some(Err(ValidErr::Description(Rc::from("not enough!")))));
    // /// assert_eq!(iter.next(), None);
    // /// ```
    // ///
    // /// `at_least` could be used to ensure that a vector created from an iterator
    // /// has a value in some index:
    // /// ```
    // /// # use crate::validiter::{Unvalidatable, ValidIter, ValidErr};
    // /// #
    // /// let iter = (0..=2); // iteration is too short, no 4th element!
    // ///
    // /// let collection: Result<Vec<_>, _> = iter.validate().at_least(4, "no fourth!").collect();
    // ///
    // /// match collection {
    // ///     Ok(vec) => {let val = vec[3];}, // doesn't crash, because the collection failed.
    // ///     Err(_) => {} // handle error
    // /// };
    // /// ```
    // /// `at_least` will not account for validation errors already in the iteration:
    // /// ```
    // /// # use std::rc::Rc;
    // /// # use crate::validiter::{Unvalidatable, ValidIter, ValidErr};
    // /// #
    // /// let mut iter = (0..=2)
    // ///                     .validate()
    // ///                     .between(1, 10, "out of bounds!")
    // ///                     .at_least(3, "not enough!");
    // ///
    // /// assert_eq!(iter.next(), Some(Err(ValidErr::WithElement(0, Rc::from("out of bounds!")))));
    // /// assert_eq!(iter.next(), Some(Ok(1)));
    // /// assert_eq!(iter.next(), Some(Ok(2)));
    // /// assert_eq!(iter.next(), Some(Err(ValidErr::Description(Rc::from("not enough!"))))); // err element added, because the first element does not count.
    // /// ```
    // ///
    // /// [`Err(ValidErr::TooFew)`](crate::valid_result::ValidErr)
    // fn at_least(self, n: usize, description: &str) -> AtLeast<Self> {
    //     AtLeast::<Self>::new(self, n, description)
    // }

    // /// Fails a validation iterator on [`PartialOrd`] elements if one the elements
    // /// is out of the argument bounds.
    // ///
    // /// `between(lowest, highest, description)` wraps any value `val` which violates the constraint
    // /// `lowest <= val && val <= highest` in a `Err(ValidErr::WithElement(val, Rc(description)))`.
    // /// Otherwise, `Ok(val)` is yielded.
    // ///
    // /// Elements already wrapped in type `Err(ValidErr::<some valid err variant>)` are ignored.
    // ///
    // /// # Examples
    // ///
    // /// Basic usage:
    // /// ```
    // /// # use std::rc::Rc;
    // /// # use crate::validiter::{Unvalidatable, ValidIter, ValidErr};
    // /// #
    // /// let a = [1, 2, 3, 4];
    // /// let mut iter = a.iter().validate().between(&2, &3, "out of bounds!");
    // ///
    // /// assert_eq!(iter.next(), Some(Err(ValidErr::WithElement(&1, Rc::from("out of bounds!")))));
    // /// assert_eq!(iter.next(), Some(Ok(&2)));
    // /// assert_eq!(iter.next(), Some(Ok(&3)));
    // /// assert_eq!(iter.next(), Some(Err(ValidErr::WithElement(&4, Rc::from("out of bounds!")))));
    // /// ```
    // ///
    // /// Partial-Equality is also supported:
    // /// ```
    // /// # use std::rc::Rc;
    // /// # use crate::validiter::{Unvalidatable, ValidIter, ValidErr};
    // /// #
    // /// let a = [f64::NAN];
    // /// let mut iter = a.iter().validate().between(&2.0, &3.0, "out of bounds!");
    // ///
    // /// // we can't compare a NaN, so we'll pattern-match:
    // /// match iter.next() {
    // ///     // this is the value we get
    // ///     Some(Err(ValidErr::WithElement(val, msg))) => {
    // ///         assert!(val.is_nan() && msg == Rc::from("out of bounds!"));
    // ///     }
    // ///
    // ///    // won't happen, '&f64::NAN' violates '&2.0 <= val && val <= &3.0'
    // ///     Some(Ok(_)) => {panic!()}
    // ///
    // ///    // also won't happen - the next yield is some out-of-bounds err
    // ///     _ => {panic!()}
    // /// }
    // /// ```
    // ///
    // /// [`Err(ValidErr::OutOfBounds(val))`](crate::valid_result::ValidErr)
    // fn between(
    //     self,
    //     lower_bound: Self::BaseType,
    //     upper_bound: Self::BaseType,
    //     description: &str,
    // ) -> Between<Self>
    // where
    //     Self::BaseType: PartialOrd,
    // {
    //     Between::<Self>::new(self, lower_bound, upper_bound, description)
    // }

    // /// Applies a closure constraint too each element, and fails the
    // /// iteration if any element violates the constraint.
    // ///
    // /// `ensure(validation, description)` is the general validation tool, it takes
    // /// a boolean test as an argument and applies it to each of the
    // /// elements in the iteration. If the test returns `true`, the element
    // /// is wrapped in `Ok(element)`. Otherwise, it is wrapped in
    // /// `Err(ValidErr::WithElement(element, Rc::from(description)))`.
    // ///
    // /// Values of type `Err(ValidErr::<some valid err variant>)` are ignored.
    // ///
    // /// # Examples
    // ///
    // /// Basic usage:
    // /// ```
    // /// # use std::rc::Rc;
    // /// # use crate::validiter::{Unvalidatable, ValidIter, ValidErr};
    // /// #
    // /// let mut iter = (0..=3).validate().ensure(|i| i % 2 == 0, "odd!");
    // ///
    // /// assert_eq!(iter.next(), Some(Ok(0)));
    // /// assert_eq!(iter.next(), Some(Err(ValidErr::WithElement(1, Rc::from("odd!")))));
    // /// assert_eq!(iter.next(), Some(Ok(2)));
    // /// assert_eq!(iter.next(), Some(Err(ValidErr::WithElement(3, Rc::from("odd!")))));
    // /// ```
    // ///
    // /// You might want to chain `ensure` validations to create
    // /// a more complex test:
    // /// ```
    // /// # use std::rc::Rc;
    // /// # use crate::validiter::{Unvalidatable, ValidIter, ValidErr};
    // /// #
    // /// let mut iter = (0..=3)
    // ///             .validate()
    // ///             .ensure(|i| i % 2 == 0, "odd!")
    // ///             .ensure(|i| *i > 0, "non-positive!");
    // ///
    // /// assert_eq!(iter.next(), Some(Err(ValidErr::WithElement(0, Rc::from("non-positive!")))));
    // /// assert_eq!(iter.next(), Some(Err(ValidErr::WithElement(1, Rc::from("odd!")))));
    // /// assert_eq!(iter.next(), Some(Ok(2)));
    // /// assert_eq!(iter.next(), Some(Err(ValidErr::WithElement(3, Rc::from("odd!")))));
    // /// ```
    // ///
    // /// `ensure` ignores error elements:
    // /// ```
    // /// # use std::rc::Rc;
    // /// # use crate::validiter::{Unvalidatable, ValidIter, ValidErr};
    // /// #
    // /// let mut iter = (0..=3)
    // ///                     .validate()
    // ///                     .between(2, 3, "out of bounds!")
    // ///                     .ensure(|i| i % 2 == 0, "odd!");
    // ///
    // /// assert_eq!(iter.next(), Some(Err(ValidErr::WithElement(0, Rc::from("out of bounds!")))));
    // /// assert_eq!(iter.next(), Some(Err(ValidErr::WithElement(1, Rc::from("out of bounds!"))))); // invalid, but not tested
    // /// assert_eq!(iter.next(), Some(Ok(2)));
    // /// assert_eq!(iter.next(), Some(Err(ValidErr::WithElement(3, Rc::from("odd!")))));
    // /// ```
    // ///
    // /// [`Err(ValidErr::Invalid(element))`](crate::valid_result::ValidErr)
    // fn ensure<F>(self, validation: F, description: &str) -> Ensure<Self, F>
    // where
    //     F: Fn(&Self::BaseType) -> bool,
    // {
    //     Ensure::<Self, F>::new(self, validation, description)
    // }

    // /// Tests each element in the iteration based on the previous element.
    // ///
    // /// `look_back(extractor, validation, description)` is sugar for calling
    // /// [`look_back_n<1, _, _, _>::(extractor, validation)`](ValidIter::look_back_n). It takes
    // /// 2 closure arguments:
    // /// 1. `extractor` - a mapping of iterator elements to some extracted
    // /// value.
    // /// 2. `validation` - a test which accepts the value extracted from
    // /// the previous element, and tests the current element based on
    // /// this value.
    // ///
    // /// Elements which fail the `validation` test will be wrapped in
    // /// `Err(ValidErr::WithElement(element, Rc(description)))`.
    // ///
    // /// Elements already wrapped in a `Err(ValidErr::<some valid err variant>)`
    // /// are ignored by both the `extractor` and the `validation` closures.
    // ///
    // /// Examples:
    // ///
    // /// Basic usage:
    // /// ```
    // /// # use std::rc::Rc;
    // /// # use crate::validiter::{Unvalidatable, ValidIter, ValidErr};
    // /// #
    // /// // is the iteration ordered?
    // /// let mut iter = (0..=2)
    // ///                     .chain(1..=1)
    // ///                     .validate()
    // ///                     .look_back(|i| *i, |prev, i| prev <= i, "unordered element!");
    // ///
    // /// assert_eq!(iter.next(), Some(Ok(0))); // first value is never tested
    // /// assert_eq!(iter.next(), Some(Ok(1)));
    // /// assert_eq!(iter.next(), Some(Ok(2)));
    // /// assert_eq!(iter.next(), Some(Err(ValidErr::WithElement(1, Rc::from("unordered element!")))));
    // /// ```
    // ///
    // /// Or maybe a slightly more exotic test:
    // /// ```
    // /// # use std::rc::Rc;
    // /// # use crate::validiter::{Unvalidatable, ValidIter, ValidErr};
    // /// #
    // /// // Does the iteration converge?
    // /// const EPSILON: f64 = 0.0001;
    // /// let mut iter = (0..).map(|i| (-1_f64).powi(i) / 2_f64.powi(i))
    // ///                     .validate()
    // ///                     .look_back(|i| i.abs(), |prev, i| prev * (1.0 - EPSILON) >= *i, "convergence failed!")
    // ///                     .take(4);
    // ///
    // /// assert_eq!(iter.next(), Some(Ok(1.0)));
    // /// assert_eq!(iter.next(), Some(Ok(-1.0 / 2.0)));
    // /// assert_eq!(iter.next(), Some(Ok(1.0 / 4.0)));
    // /// assert_eq!(iter.next(), Some(Ok(-1.0 / 8.0)));
    // /// ```
    // ///
    // /// [`look_back_n<1, _, _, _>::(extractor, validation)`](ValidIter::look_back_n)
    // /// [`Err(ValidErr::LookBackFailed(element))`](crate::valid_result::ValidErr)
    // fn look_back<A, M, F>(
    //     self,
    //     extractor: M,
    //     validation: F,
    //     description: &str,
    // ) -> LookBack<Self, A, M, F, 1>
    // where
    //     A: Default,
    //     M: Fn(&Self::BaseType) -> A,
    //     F: Fn(&A, &Self::BaseType) -> bool,
    // {
    //     LookBack::new(self, extractor, validation, description)
    // }

    // /// Fails an iteration if it does not conform to some cycling
    // /// of properties.
    // ///
    // /// `look_back_n::<N, _, _, _>(extractor, validation, description)` takes 4
    // /// arguments:
    // /// 1. `N` - a constant `usize` describing a cycle length
    // /// 2. `extractor` - a mapping of iterator elements to some extracted
    // /// value.
    // /// 3. `validation` - a test which accepts the value extracted from
    // /// the Nth preceding element, and tests the current element based
    // /// on this value.
    // /// 4. A validation descriptor/identifier.
    // ///
    // /// Each iterator element wrapped in `Ok(element)` gets processed in
    // /// these 2 ways:
    // /// 1. Assuming there was a previous Nth element (we'll call it `p_nth`),
    // /// the current element is tested for `validation(extractor(p_nth), element)`.
    // /// 2. If the element passed the test, it is wrapped in `Ok(element)`.
    // /// otherwise it wrapped in `Err(ValidErr::WithElement(element, Rc(description)))`, and
    // /// will not be used to test the next nth element.
    // ///
    // /// Because of the underlying implementation, you must specify the generic
    // /// constant `N` when calling the method, and so you also must allow for
    // /// the other 3 generic arguments to be inferred. Therefore calling this
    // /// method is a bit cumbersome:
    // /// `look_back_n<N, _, _, _>(args...)`
    // ///
    // /// Important notes about the implementation:
    // ///  - The adapter uses stack memory to store the values extracted
    // /// from the previous n valid elements - so, ummm... maybe don't do
    // /// `look_back_n<1_000_000_000, _, _, _>`
    // ///  - The values actually stored inside the iterator memory are precomputed
    // /// results of `extractor`. For example - if the iteration is over elements of
    // /// type `Vec<i32>` and the extractor closure is `|v| v.iter().sum()`, the
    // /// type of the stored value is `i32`, rather than `Vec<i32>`.
    // ///
    // /// # Examples
    // ///
    // /// Basic usage:
    // /// ```
    // /// # use std::rc::Rc;
    // /// # use crate::validiter::{Unvalidatable, ValidIter, ValidErr};
    // /// #
    // /// let mut iter = (0..=2).chain(2..=4)
    // ///                     .validate()
    // ///                     .look_back_n::<2, _, _, _>(
    // ///                         |i| *i,
    // ///                         |prev, i| prev % 2 == i % 2,
    // ///                         "look back failed!"
    // ///                     );
    // ///
    // /// assert_eq!(iter.next(), Some(Ok(0)));
    // /// assert_eq!(iter.next(), Some(Ok(1)));
    // /// assert_eq!(iter.next(), Some(Ok(2))); // evaluated with respect to 0
    // /// assert_eq!(iter.next(), Some(Err(ValidErr::WithElement(2, Rc::from("look back failed!"))))); // evaluated with respect to 1
    // /// assert_eq!(iter.next(), Some(Ok(3))); // also evaluated with respect to 1
    // /// assert_eq!(iter.next(), Some(Ok(4))); // evaluted with respect to 2
    // /// ```
    // ///
    // /// `look_back_n` could be used to force an iteration to cycle through
    // /// a sequence of predetermined properties:
    // /// ```
    // /// # use std::rc::Rc;
    // /// # use crate::validiter::{Unvalidatable, ValidIter, ValidErr};
    // /// #
    // /// let sequence = "abc";
    // /// let s = "abfbc";
    // ///
    // /// let mut iter = sequence.chars().chain(s.chars())
    // ///                 .validate()
    // ///                 .look_back_n::<3, _, _, _>(|c| *c, |p_nth, c| p_nth == c, "cycle failed!");
    // ///
    // /// assert_eq!(iter.next(), Some(Ok('a')));
    // /// assert_eq!(iter.next(), Some(Ok('b')));
    // /// assert_eq!(iter.next(), Some(Ok('c')));
    // /// assert_eq!(iter.next(), Some(Ok('a')));
    // /// assert_eq!(iter.next(), Some(Ok('b')));
    // /// assert_eq!(iter.next(), Some(Err(ValidErr::WithElement('f', Rc::from("cycle failed!")))));
    // /// assert_eq!(iter.next(), Some(Err(ValidErr::WithElement('b', Rc::from("cycle failed!")))));
    // /// assert_eq!(iter.next(), Some(Ok('c')));
    // /// ```
    // ///
    // /// [`Err(ValidErr::LookBackFailed(element))`](crate::valid_result::ValidErr)
    // fn look_back_n<const N: usize, A, M, F>(
    //     self,
    //     extractor: M,
    //     validation: F,
    //     description: &str,
    // ) -> LookBack<Self, A, M, F, N>
    // where
    //     A: Default,
    //     M: Fn(&Self::BaseType) -> A,
    //     F: Fn(&A, &Self::BaseType) -> bool,
    // {
    //     LookBack::new(self, extractor, validation, description)
    // }

    // /// Fails an iteration if `extractor` does not give the same result
    // /// for all elements.
    // ///
    // /// `const_over(extractor, description)` takes a closure argument that computes
    // /// some value for each element in iteration. If for some element
    // /// this results in a value which is not equal to value computed
    // /// from the first element, this element is wrapped in
    // /// `Err(ValidErr::WithElement(element, Rc::from(description)))`. Otherwise, the element
    // /// is wrapped in `Ok(element)`. The first valid element is always wrapped
    // /// in `Ok`.
    // ///
    // /// # Examples
    // ///
    // /// Basic usage:
    // /// ```
    // /// # use std::rc::Rc;
    // /// # use crate::validiter::{Unvalidatable, ValidIter, ValidErr};
    // /// #
    // /// let uppercase = "ABc";
    // /// let mut iter = uppercase.chars().validate().const_over(|c| c.is_uppercase(), "case changed!");
    // ///
    // /// assert_eq!(iter.next(), Some(Ok('A')));
    // /// assert_eq!(iter.next(), Some(Ok('B')));
    // /// assert_eq!(iter.next(), Some(Err(ValidErr::WithElement('c', Rc::from("case changed!")))));
    // /// ```
    // ///
    // /// `const_over` ignores validation errors:
    // /// ```
    // /// # use std::rc::Rc;
    // /// # use crate::validiter::{Unvalidatable, ValidIter, ValidErr};
    // /// #
    // /// let uppercase = "1AB2c";
    // /// let mut iter = uppercase
    // ///                     .chars()
    // ///                     .validate()
    // ///                     .ensure(|c| c.is_alphabetic(), "not alphabetic!")
    // ///                     .const_over(|c| c.is_uppercase(), "case changed!");
    // ///
    // /// assert_eq!(iter.next(), Some(Err(ValidErr::WithElement('1', Rc::from("not alphabetic!")))));
    // /// assert_eq!(iter.next(), Some(Ok('A')));
    // /// assert_eq!(iter.next(), Some(Ok('B')));
    // /// assert_eq!(iter.next(), Some(Err(ValidErr::WithElement('2', Rc::from("not alphabetic!")))));
    // /// assert_eq!(iter.next(), Some(Err(ValidErr::WithElement('c', Rc::from("case changed!")))));
    // /// ```
    // ///
    // /// [`Err(ValidErr::BrokenConstant(element))`](crate::valid_result::ValidErr)
    // fn const_over<A, M>(self, extractor: M, description: &str) -> ConstOver<Self, A, M>
    // where
    //     A: PartialEq,
    //     M: Fn(&Self::BaseType) -> A,
    // {
    //     ConstOver::new(self, extractor, description)
    // }
}
