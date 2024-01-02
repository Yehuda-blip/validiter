#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, PartialEq)]
pub enum ValidErr<E> {
    TooMany(E),
    TooFew,
    OutOfBounds(E),
    Invalid(E),
    Lifted,
    LookBackFailed(E),
    BrokenConstant(E),
}

pub type VResult<E> = Result<E, ValidErr<E>>;
