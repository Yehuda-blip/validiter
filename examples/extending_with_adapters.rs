use std::rc::Rc;

use validiter::{Unvalidatable, VResult, ValidErr, ValidIter};

const DESCRIPTION: &str = "always failing";

struct FailAlways<I>
where
    I: ValidIter + Iterator<Item = VResult<I::BaseType>>,
{
    iter: I,
}

impl<I> Iterator for FailAlways<I>
where
    I: ValidIter + Iterator<Item = VResult<I::BaseType>>,
{
    type Item = VResult<I::BaseType>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(Ok(val)) => Some(Err(ValidErr::WithElement(val, Rc::from(DESCRIPTION)))),
            other => other,
        }
    }
}

impl<I> ValidIter for FailAlways<I>
where
    I: ValidIter + Iterator<Item = VResult<I::BaseType>>,
{
    type BaseType = I::BaseType;
}

trait MyValidIter: ValidIter {
    fn fail_always(self) -> FailAlways<Self> {
        FailAlways { iter: self }
    }
}

impl<T: ValidIter> MyValidIter for T {}

fn main() {
    (0..10)
        .validate()
        .ensure(|i| i % 2 == 0, "ensure even")
        .fail_always()
        .for_each(|i| println!("{i:?}"))
}
