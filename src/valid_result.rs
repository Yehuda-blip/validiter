use std::{
    error::Error,
    fmt::{Debug, Display}, rc::Rc,
};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ValidErr<E> {
    Description(Rc<str>),
    WithElement(E, Rc<str>)
}

impl<E> Display for ValidErr<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let err_type_str = match self {
            Self::Description(desc) => desc,
            Self::WithElement(_, desc) => desc
        };
        write!(f, "{}", err_type_str)
    }
}

impl<E> Error for ValidErr<E> where E: Debug {}

pub type VResult<E> = Result<E, ValidErr<E>>;
