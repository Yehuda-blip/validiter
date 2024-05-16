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
    Invalid(T, String),
    /// A general error recieved after using the [`lift_errs`](crate::ErrLiftable::lift_errs) adapter
    Lifted(String),
    /// Corresponds to the [`ValidIter`](crate::ValidIter) [`look_back`](crate::ValidIter::look_back) and [`look_back_n`](crate::ValidIter::look_back_n) adapters
    LookBackFailed(T, String),
    /// Corresponds to the [`ValidIter`](crate::ValidIter) [`const_over`](crate::ValidIter::const_over) adapter
    BrokenConstant(T, String),
    /// A general error, that can be used to translate non `ValidErr` error types to `ValidErr::Mapped`
    Mapped(String),
}

impl<T> ValidErr<T> {
    pub fn into_msg(self) -> String {
        match self {
            ValidErr::TooMany(_, msg) => msg,
            ValidErr::TooFew(msg) => msg,
            ValidErr::OutOfBounds(_, msg) => msg,
            ValidErr::Invalid(_, msg) => msg,
            ValidErr::Lifted(msg) => msg,
            ValidErr::LookBackFailed(_, msg) => msg,
            ValidErr::BrokenConstant(_, msg) => msg,
            ValidErr::Mapped(msg) => msg,
        }
    }

    pub fn as_msg(&self) -> &str {
        match self {
            ValidErr::TooMany(_, msg) => &msg,
            ValidErr::TooFew(msg) => &msg,
            ValidErr::OutOfBounds(_, msg) => &msg,
            ValidErr::Invalid(_, msg) => &msg,
            ValidErr::Lifted(msg) => &msg,
            ValidErr::LookBackFailed(_, msg) => &msg,
            ValidErr::BrokenConstant(_, msg) => &msg,
            ValidErr::Mapped(msg) => &msg,
        }
    }
}


impl<T> Display for ValidErr<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_msg())
    }
}

impl<T> Error for ValidErr<T> where T: Debug {}

pub type VResult<T> = Result<T, ValidErr<T>>;
