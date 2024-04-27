use std::{
    error::Error,
    fmt::{Debug, Display},
};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ValidErr<T> {
    /// Corresponds to the [`ValidIter`](crate::ValidIter) [`at_most`](crate::ValidIter::at_most) adapter
    TooMany(T, String),
    /// Corresponds to the [`ValidIter`](crate::ValidIter) [`at_least`](crate::ValidIter::at_least) adapter
    TooFew(String),
    /// Corresponds to the [`ValidIter`](crate::ValidIter) [`between`](crate::ValidIter::between) adapter
    OutOfBounds(T, String),
    /// Corresponds to the [`ValidIter`](crate::ValidIter) [`ensure`](crate::ValidIter::ensure) adapter
    Invalid(T),
    /// A general error recieved after using the [`lift_errs`](crate::ErrLiftable::lift_errs) adapter
    Lifted,
    /// Corresponds to the [`ValidIter`](crate::ValidIter) [`look_back`](crate::ValidIter::look_back) and [`look_back_n`](crate::ValidIter::look_back_n) adapters
    LookBackFailed(T),
    /// Corresponds to the [`ValidIter`](crate::ValidIter) [`const_over`](crate::ValidIter::const_over) adapter
    BrokenConstant(T, String),
    /// A general error, that can be used to translate non `ValidErr` error types to `ValidErr::Mapped`
    Mapped,
}

impl<T> Display for ValidErr<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let err_type_str = match self {
            ValidErr::TooMany(_, msg) => msg,
            ValidErr::TooFew(msg) => msg,
            ValidErr::OutOfBounds(_, msg) => msg,
            ValidErr::Invalid(_) => "ValidErr::Invalid",
            ValidErr::Lifted => "ValidErr::Lifted",
            ValidErr::LookBackFailed(_) => "ValidErr::LookBackFailed",
            ValidErr::BrokenConstant(_, msg) => msg,
            ValidErr::Mapped => "ValidErr::Mapped",
        };
        write!(f, "{}", err_type_str)
    }
}

impl<T> Error for ValidErr<T> where T: Debug {}

pub type VResult<T> = Result<T, ValidErr<T>>;
