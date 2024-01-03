use std::{error::Error, fmt::{Debug, Display}};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ValidErr<E> {
    TooMany(E),
    TooFew,
    OutOfBounds(E),
    Invalid(E),
    Lifted,
    LookBackFailed(E),
    BrokenConstant(E),
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
            ValidErr::BrokenConstant(_) => "ValidErr::BrokenConstant"
        };
        write!(f, "{}", err_type_str)
    }
}

impl<E> Error for ValidErr<E> where E: Debug {}

pub type VResult<E> = Result<E, ValidErr<E>>;
