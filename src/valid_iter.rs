use crate::{
    at_least::AtLeast,
    between::Between,
    ensure::Ensure,
    valid_result::ValidationResult,
};

use super::{at_most::AtMost, validatable::Validatable};

pub trait Unvalidatable: Iterator + Sized {
    fn validate(self) -> Validatable<Self> {
        Validatable { iter: self }
    }
}

impl<T> Unvalidatable for T where T: Iterator + Sized {}

pub trait ValidationSpaceAdapter {
    type BaseType;
}

pub trait ValidIter {
    type BaseType;

    fn at_most(self, max_count: usize) -> AtMost<Self>
    where
        Self: Sized + ValidationSpaceAdapter,
    {
        AtMost::<Self>::new(self, max_count)
    }

    fn at_least(self, min_count: usize) -> AtLeast<Self>
    where
        Self: Sized + ValidationSpaceAdapter,
    {
        AtLeast::<Self>::new(self, min_count)
    }

    fn between(
        self,
        lower_bound: <Self as ValidationSpaceAdapter>::BaseType,
        upper_bound: <Self as ValidationSpaceAdapter>::BaseType,
    ) -> Between<Self>
    where
        Self: Sized + ValidationSpaceAdapter,
        <Self as ValidationSpaceAdapter>::BaseType: PartialOrd,
    {
        Between::<Self>::new(self, lower_bound, upper_bound)
    }

    fn ensure<F>(self, validation: F) -> Ensure<Self, F>
    where
        Self: Sized + ValidationSpaceAdapter,
        F: FnMut(&<Self as ValidationSpaceAdapter>::BaseType) -> bool,
    {
        Ensure::<Self, F>::new(self, validation)
    }
}

impl<I> ValidationSpaceAdapter for I
where
    I: Iterator + Sized,
    I::Item: ValidationResult
{
    type BaseType = <<I as Iterator>::Item as ValidationResult>::BaseType;
}
