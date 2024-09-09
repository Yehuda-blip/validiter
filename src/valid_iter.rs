pub trait ValidErr {}

/// The trait defining a validatable iterator. While it is not sealed,
/// you should probably not implement it unless you're feeling experimental.
pub trait ValidIter<E>: Iterator<Item = Result<Self::BaseType, E>>
where
    E: ValidErr,
{
    type BaseType;
}

impl<I, T, E> ValidIter<E> for I
where
    I: Iterator<Item = Result<T, E>>,
    E: ValidErr,
{
    type BaseType = T;
}
