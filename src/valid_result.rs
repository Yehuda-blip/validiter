#[derive(Debug, PartialEq)]
pub enum ValidErr<E> {
    TooMany(E),
    TooFew,
    OutOfBounds(E),
    Invalid(E),
}

pub type VResult<E> = Result<E, ValidErr<E>>;

pub trait ValidationResult {
    type BaseType;
}

impl<E> ValidationResult for VResult<E> {
    type BaseType = E;
}
