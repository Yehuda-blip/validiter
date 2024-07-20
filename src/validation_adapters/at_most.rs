use std::{marker::PhantomData, rc::Rc};

use crate::{ValidErr, ValidIter, WithElement};


/// The [`AtMost`] ValidIter adapter, for more info see [`at_most`](crate::ValidIter::at_most).
#[derive(Debug, Clone)]
pub struct AtMost<I, E, W>
where
    I: ValidIter<E> + Iterator<Item = Result<I::BaseType, E>>,
    E: ValidErr<I::BaseType>,
    W: WithElement<I::BaseType, E> 
{
    iter: I,
    max_count: usize,
    counter: usize,
    error_factory: PhantomData<W>
}

impl<I, E, W> AtMost<I, E, W>
where
    I: ValidIter<E> + Iterator<Item = Result<I::BaseType, E>>,
    E: ValidErr<I::BaseType>,
    W: WithElement<I::BaseType, E> 
{
    pub(crate) fn new(iter: I, max_count: usize) -> AtMost<I, E, W> {
        AtMost {
            iter,
            max_count,
            counter: 0,
            error_factory: PhantomData::<W>
        }
    }
}

impl<I, E, W> Iterator for AtMost<I, E, W>
where
    I: ValidIter<E> + Iterator<Item = Result<I::BaseType, E>>,
    E: ValidErr<I::BaseType>,
    W: WithElement<I::BaseType, E> 
{
    type Item = Result<I::BaseType, E>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(Ok(val)) => match self.counter >= self.max_count {
                true => Some(Err(W::from_element(val))),
                false => {
                    self.counter += 1;
                    Some(Ok(val))
                }
            },
            other => other,
        }
    }
}

impl<I, E, W> ValidIter<E> for AtMost<I, E, W>
where
    I: ValidIter<E> + Iterator<Item = Result<I::BaseType, E>>,
    E: ValidErr<I::BaseType>,
    W: WithElement<I::BaseType, E> 
{
    type BaseType = I::BaseType;
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::{Unvalidatable, ValidIter};

//     #[test]
//     fn test_at_most() {
//         (0..10)
//             .validate()
//             .at_most(5, "test")
//             .for_each(|res_i| match res_i {
//                 Ok(i) => assert!(i < 5),
//                 Err(err_i) => match err_i {
//                     ValidErr::WithElement(i, msg) => {
//                         assert!(i >= 5);
//                         assert_eq!(msg, Rc::from("test"))
//                     }
//                     _ => panic!("incorrect err for at most validator"),
//                 },
//             })
//     }

//     #[test]
//     fn test_at_most_has_correct_bounds() {
//         let failed_collection = (0..10)
//             .validate()
//             .at_most(9, "test")
//             .collect::<Result<Vec<_>, _>>();
//         assert!(matches!(
//             failed_collection,
//             Err(ValidErr::WithElement(_, _))
//         ));

//         let collection = (0..10)
//             .validate()
//             .at_most(10, "test")
//             .collect::<Result<Vec<_>, _>>();
//         assert!(matches!(collection, Ok(_)));

//         let empty_collection = (0..0)
//             .validate()
//             .at_most(0, "test")
//             .collect::<Result<Vec<_>, _>>();
//         assert!(matches!(empty_collection, Ok(_)));
//     }

//     #[test]
//     fn test_at_most_all_elements_are_present_and_in_order() {
//         (0..10)
//             .validate()
//             .at_most(5, "test")
//             .enumerate()
//             .for_each(|(i, res_i)| match i < 5 {
//                 true => match res_i {
//                     Ok(int) if int == i as i32 => {}
//                     _ => panic!("bad match for item {}: {:?}", i, res_i),
//                 },
//                 false => match res_i {
//                     Err(ValidErr::WithElement(int, msg)) if int == i as i32 => {
//                         assert_eq!(msg, Rc::from("test"))
//                     }
//                     _ => panic!("bad match for item {}: {:?}", i, res_i),
//                 },
//             })
//     }

//     #[test]
//     fn test_at_most_by_ref() {
//         [0, 1, 2, 3]
//             .iter()
//             .validate()
//             .at_most(2, "test")
//             .enumerate()
//             .for_each(|(i, res_i)| match i < 2 {
//                 true => assert!(matches!(res_i, Ok(_))),
//                 false => assert!(matches!(res_i, Err(ValidErr::WithElement(_, _)))),
//             })
//     }

//     #[test]
//     fn test_at_most_counting_validator_correctly_skips_errors() {
//         let results = (0..5)
//             .validate()
//             .ensure(|i| i % 2 == 0, "ensure")
//             .at_most(2, "at-most")
//             .collect::<Vec<_>>();
//         assert_eq!(
//             results,
//             vec![
//                 Ok(0),
//                 Err(ValidErr::WithElement(1, Rc::from("ensure"))),
//                 Ok(2),
//                 Err(ValidErr::WithElement(3, Rc::from("ensure"))),
//                 Err(ValidErr::WithElement(4, Rc::from("at-most")))
//             ]
//         )
//     }
// }
