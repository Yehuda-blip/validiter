use std::rc::Rc;

use crate::{valid_iter::ValidIter, valid_result::ValidErr};

use super::valid_result::VResult;

/// The [`Ensure`] ValidIter adapter, for more info see [`ensure`](crate::ValidIter::ensure).
#[derive(Debug, Clone)]
pub struct Ensure<I, F>
where
    I: ValidIter + Iterator<Item = VResult<I::BaseType>>,
    F: Fn(&I::BaseType) -> bool,
{
    iter: I,
    validation: F,
    desc: Rc<str>,
}

impl<I, F> Ensure<I, F>
where
    I: Sized + ValidIter + Iterator<Item = VResult<I::BaseType>>,
    F: Fn(&I::BaseType) -> bool,
{
    pub(crate) fn new(iter: I, validation: F, desc: &str) -> Ensure<I, F> {
        Ensure {
            iter,
            validation,
            desc: Rc::from(desc),
        }
    }
}

impl<I, F> Iterator for Ensure<I, F>
where
    I: ValidIter + Iterator<Item = VResult<I::BaseType>>,
    F: Fn(&I::BaseType) -> bool,
{
    type Item = VResult<I::BaseType>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(Ok(val)) => match (self.validation)(&val) {
                true => Some(Ok(val)),
                false => Some(Err(ValidErr::WithElement(val, Rc::clone(&self.desc)))),
            },
            other => other,
        }
    }
}

impl<I, F> ValidIter for Ensure<I, F>
where
    I: ValidIter + Iterator<Item = VResult<I::BaseType>>,
    F: Fn(&I::BaseType) -> bool,
{
    type BaseType = I::BaseType;
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::{
        valid_iter::{Unvalidatable, ValidIter},
        valid_result::ValidErr,
    };

    #[test]
    fn test_ensure() {
        (0..10)
            .validate()
            .ensure(|i| i % 2 == 0, "ensure")
            .enumerate()
            .for_each(|(i, res_i)| match res_i {
                Ok(int) if i % 2 == 0 && i as i32 == int => {}
                Err(ValidErr::WithElement(int, msg))
                    if i % 2 == 1 && i as i32 == int && msg.as_ref() == "ensure" => {}
                _ => panic!("unexpected value in ensure adapter"),
            })
    }

    #[test]
    fn test_ensure_ignores_errors() {
        let v = (0..=0)
            .validate()
            .ensure(|i| *i != 0, "A")
            .ensure(|i| *i != 0, "B")
            .next();
        assert_eq!(v, Some(Err(ValidErr::WithElement(0, Rc::from("A")))))
    }
}
