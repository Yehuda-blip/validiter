use crate::{ErrorOnly, ValidErr, WithElement};

struct AtMost;

struct AtLeast;

enum MyValidErr<T>{
    AtMost(T),
    AtLeast
}

impl<T> ValidErr<T> for MyValidErr<T> {}

impl<T> WithElement<T, MyValidErr<T>> for AtMost {
    fn from_element(element: T) -> MyValidErr<T> {
        MyValidErr::AtMost(element)
    }
}

impl<T> ErrorOnly<T, MyValidErr<T>> for AtLeast {
    fn new() -> MyValidErr<T> {
        MyValidErr::AtLeast
    }
}

