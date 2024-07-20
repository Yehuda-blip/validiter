use std::{
    error::Error,
    fmt::{Debug, Display},
    rc::Rc,
};

pub trait ValidErr<T> {}

pub trait WithElement<T, E>: where E: ValidErr<T> {
    fn from_element(element: T) -> E;
}

pub trait ErrorOnly<T, E>: where E: ValidErr<T> {
    fn new() -> E;
}

// /// An enum for the possible shapes of a [`ValidIter`](crate::ValidIter) error.
// #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
// pub enum ValidErr<E> {
//     /// Contains an [`Rc::(str)`] where the [`str`] is the token passed to the erroring [`ValidIter`](crate::ValidIter).
//     /// Created by validiter adapters when returning the error-causing element is impossible or meaningless.
//     Description(Rc<str>),
//     /// Contains the element that caused the error, and an [`Rc::(str)`] where the [`str`] is the token passed
//     /// to the erroring [`ValidIter`](crate::ValidIter).
//     WithElement(E, Rc<str>),
// }

// impl<E> Display for ValidErr<E> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let err_type_str = match self {
//             Self::Description(desc) => desc,
//             Self::WithElement(_, desc) => desc,
//         };
//         write!(f, "{}", err_type_str)
//     }
// }

// impl<E> Error for ValidErr<E> where E: Debug {}

// /// An alias for [`Result<E, ValidErr<E>>`]. See [`ValidErr<E>`](crate::ValidErr)
// pub type VResult<T> = Result<T, E>
// where E: ValidErr<T>;
