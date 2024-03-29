use std::{
    error::Error,
    fmt::{Debug, Display},
};


const ERR_MSG_SPACING: &'static str = " - ";


#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ValidErr<E> {
    /// Corresponds to the [`ValidIter`](crate::ValidIter) [`at_most`](crate::ValidIter::at_most) adapter
    TooMany { element: E, msg: Option<String> },
    /// Corresponds to the [`ValidIter`](crate::ValidIter) [`at_least`](crate::ValidIter::at_least) adapter
    TooFew { msg: Option<String> },
    /// Corresponds to the [`ValidIter`](crate::ValidIter) [`between`](crate::ValidIter::between) adapter
    OutOfBounds { element: E, msg: Option<String> },
    /// Corresponds to the [`ValidIter`](crate::ValidIter) [`ensure`](crate::ValidIter::ensure) adapter
    Invalid { element: E, msg: Option<String> },
    /// A general error recieved after using the [`lift_errs`](crate::ErrLiftable::lift_errs) adapter
    Lifted { msg: Option<String> },
    /// Corresponds to the [`ValidIter`](crate::ValidIter) [`look_back`](crate::ValidIter::look_back) and [`look_back_n`](crate::ValidIter::look_back_n) adapters
    LookBackFailed { element: E, msg: Option<String> },
    /// Corresponds to the [`ValidIter`](crate::ValidIter) [`const_over`](crate::ValidIter::const_over) adapter
    BrokenConstant { element: E, msg: Option<String> },
    /// A general error, that can be used to translate non `ValidErr` error types to `ValidErr::Mapped`
    Mapped { msg: String },
}

impl<E> ValidErr<E> {
    fn variant_str(&self) -> &'static str {
        match self {
            ValidErr::TooMany { .. } => "ValidErr::TooMany",
            ValidErr::TooFew { .. } => "ValidErr::TooFew",
            ValidErr::OutOfBounds { .. }=> "ValidErr::OutOfBounds",
            ValidErr::Invalid { .. }=> "ValidErr::Invalid",
            ValidErr::Lifted { .. }=> "ValidErr::Lifted",
            ValidErr::LookBackFailed{ .. } => "ValidErr::LookBackFailed",
            ValidErr::BrokenConstant { .. }=> "ValidErr::BrokenConstant",
            ValidErr::Mapped{ .. } => "ValidErr::Mapped"
        }
    }

    fn add_msg_or_empty(&self) -> String {
        match self {
            
            ValidErr::TooMany { msg, .. } => {
                match msg {
                    Some(msg) => ERR_MSG_SPACING.to_string() + msg,
                    None => "".into()
                }
            },
            ValidErr::TooFew  { msg, .. } => {
                match msg {
                    Some(msg) => ERR_MSG_SPACING.to_string() + msg,
                    None => "".into()
                }
            },
            ValidErr::OutOfBounds  { msg, .. } => {
                match msg {
                    Some(msg) => ERR_MSG_SPACING.to_string() + msg,
                    None => "".into()
                }
            },
            ValidErr::Invalid  { msg, .. } => {
                match msg {
                    Some(msg) => ERR_MSG_SPACING.to_string() + msg,
                    None => "".into()
                }
            },
            ValidErr::Lifted  { msg, .. } => {
                match msg {
                    Some(msg) => ERR_MSG_SPACING.to_string() + msg,
                    None => "".into()
                }
            },
            ValidErr::LookBackFailed { msg, .. } => {
                match msg {
                    Some(msg) => ERR_MSG_SPACING.to_string() + msg,
                    None => "".into()
                }
            },
            ValidErr::BrokenConstant  { msg, .. } => {
                match msg {
                    Some(msg) => ERR_MSG_SPACING.to_string() + msg,
                    None => "".into()
                }
            },
            ValidErr::Mapped { msg, .. } => {
                ERR_MSG_SPACING.to_string() + msg
            },
        }
    }
}

impl<E> Display for ValidErr<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.variant_str().to_string() + &self.add_msg_or_empty())
    }
}

impl<E> Error for ValidErr<E> where E: Debug {}

pub type VResult<E> = Result<E, ValidErr<E>>;
