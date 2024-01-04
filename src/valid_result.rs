use std::{
    error::Error,
    fmt::{Debug, Display},
};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ValidErr<E> {
    /// Corresponds to the [`ValidIter`](crate::ValidIter) [`at_most`](crate::ValidIter::at_most) adapter
    TooMany(E),
    /// Corresponds to the [`ValidIter`](crate::ValidIter) [`at_least`](crate::ValidIter::at_least) adapter
    TooFew,
    /// Corresponds to the [`ValidIter`](crate::ValidIter) [`between`](crate::ValidIter::between) adapter
    OutOfBounds(E),
    /// Corresponds to the [`ValidIter`](crate::ValidIter) [`ensure`](crate::ValidIter::ensure) adapter
    Invalid(E),
    /// A general error recieved after using the [`lift_errs`](crate::ErrLiftable::lift_errs) adapter
    Lifted,
    /// Corresponds to the [`ValidIter`](crate::ValidIter) [`look_back`](crate::ValidIter::look_back) and [`look_back_n`](crate::ValidIter::look_back_n) adapters
    LookBackFailed(E),
    /// Corresponds to the [`ValidIter`](crate::ValidIter) [`const_over`](crate::ValidIter::const_over) adapter
    BrokenConstant(E),
    /// A general error, that can be used to translate non `ValidErr` error types to `ValidErr::Mapped`
    Mapped,
}

impl<E> Display for ValidErr<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let err_type_str = match self {
            ValidErr::TooMany(_) => "ValidErr::TooMany",
            ValidErr::TooFew => "ValidErr::TooFew",
            ValidErr::OutOfBounds(_) => "ValidErr::OutOfBounds",
            ValidErr::Invalid(_) => "ValidErr::Invalid",
            ValidErr::Lifted => "ValidErr::Lifted",
            ValidErr::LookBackFailed(_) => "ValidErr::LookBackFailed",
            ValidErr::BrokenConstant(_) => "ValidErr::BrokenConstant",
            ValidErr::Mapped => "ValidErr::Mapped",
        };
        write!(f, "{}", err_type_str)
    }
}

impl<E> Error for ValidErr<E> where E: Debug {}

pub type VResult<E> = Result<E, ValidErr<E>>;
