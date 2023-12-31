#[derive(Debug, PartialEq)]
pub enum ValidErr<E> {
    TooMany(E),
    TooFew,
    OutOfBounds(E),
    Invalid(E),
    LiftedErr,
}

pub type VResult<E> = Result<E, ValidErr<E>>;
