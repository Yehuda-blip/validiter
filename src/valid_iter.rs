use crate::{
    at_least::AtLeast, between::Between, ensure::Ensure, look_back::LookBack, valid_result::VResult,
};

use super::{at_most::AtMost, validatable::Validatable};

pub trait Unvalidatable: Iterator + Sized {
    fn validate(self) -> Validatable<Self> {
        Validatable { iter: self }
    }
}

impl<T> Unvalidatable for T where T: Iterator + Sized {}

pub trait ValidIter: Iterator {
    type BaseType;

    fn at_most(self, max_count: usize) -> AtMost<Self>
    where
        Self: Sized + ValidIter + Iterator<Item = VResult<Self::BaseType>>,
    {
        AtMost::<Self>::new(self, max_count)
    }

    fn at_least(self, min_count: usize) -> AtLeast<Self>
    where
        Self: Sized + ValidIter + Iterator<Item = VResult<Self::BaseType>>,
    {
        AtLeast::<Self>::new(self, min_count)
    }

    fn between(
        self,
        lower_bound: <Self as ValidIter>::BaseType,
        upper_bound: <Self as ValidIter>::BaseType,
    ) -> Between<Self>
    where
        Self: Sized + ValidIter + Iterator<Item = VResult<Self::BaseType>>,
        <Self as ValidIter>::BaseType: PartialOrd,
    {
        Between::<Self>::new(self, lower_bound, upper_bound)
    }

    fn ensure<F>(self, validation: F) -> Ensure<Self, F>
    where
        Self: Sized + ValidIter + Iterator<Item = VResult<Self::BaseType>>,
        F: FnMut(&<Self as ValidIter>::BaseType) -> bool,
    {
        Ensure::<Self, F>::new(self, validation)
    }

    fn look_back<A, M, F>(self, extractor: M, validation: F) -> LookBack<Self, A, M, F, 1>
    where
        Self: Sized + ValidIter + Iterator<Item = VResult<Self::BaseType>>,
        A: Default,
        M: FnMut(&Self::BaseType) -> A,
        F: FnMut(&A, &Self::BaseType) -> bool,
    {
        LookBack::new(self, extractor, validation)
    }

    fn look_back_n<const N: usize, A, M, F>(self, extractor: M, validation: F) -> LookBack<Self, A, M, F, N>
    where
        Self: Sized + ValidIter + Iterator<Item = VResult<Self::BaseType>>,
        A: Default,
        M: FnMut(&Self::BaseType) -> A,
        F: FnMut(&A, &Self::BaseType) -> bool,
    {
        LookBack::new(self, extractor, validation)
    }
}
