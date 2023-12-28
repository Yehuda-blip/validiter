use super::{at_most::AtMost, valid_result::VResult, validatable::Validatable};

pub trait Unvalidatable: Iterator + Sized {
    fn to_validation_space(self) -> Validatable<Self> {
        Validatable {iter: self}
    }
}

impl<T> Unvalidatable for T where T: Iterator + Sized {}

pub trait ValidationSpaceAdapter {
    type BaseType;
    // type Transformed: ValidationSpaceAdapter;
}

pub trait ValidIter {
    type BaseType;
    // type InnerIter: ValidationSpaceAdapter;

    fn at_most(self, max_count: usize) -> AtMost<Self>
    where
        Self: Sized + ValidationSpaceAdapter,
    {
        AtMost::<Self>::new(self, max_count)
    }
}

// impl<T> ValidIter for T where T: ValidIter {
//     type BaseType = T::BaseType;

//     type InnerIter = T::InnerIter;
// }
