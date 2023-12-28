use crate::{at_least::AtLeast, between::Between};

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
}
