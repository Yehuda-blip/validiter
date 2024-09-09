#[cfg(feature = "adapters")]
pub mod validation_adapters {
    pub(crate) mod at_least;
    pub(crate) mod at_most;
    pub(crate) mod const_over;
    pub(crate) mod look_back;
    pub use at_least::AtLeast;
    pub use at_most::AtMost;
    pub use const_over::ConstOver;
    pub use look_back::LookBack;
}

mod valid_iter;

pub use validation_adapters as adapters;
pub use valid_iter::ValidErr;
pub use valid_iter::ValidIter;

// #[cfg(test)]
// mod tests {
//     use std::rc::Rc;

//     use crate::{Unvalidatable, VResult, ValidErr, ValidIter};

//     #[test]
//     fn test_multi_validation_on_iterator() {
//         let validation_results = (0..10)
//             .chain(0..10)
//             .chain(-1..=-1)
//             .chain(1..=1)
//             .validate()
//             .const_over(|i| *i >= 0, "co")
//             .look_back_n::<10, _, _, _>(|i| *i, |prev, curr| prev == curr, "lb")
//             .at_most(7, "am")
//             .between(2, 8, "b")
//             .ensure(|i| i % 2 == 0, "e")
//             .at_least(4, "al")
//             .collect::<Vec<VResult<_>>>();
//         assert_eq!(
//             validation_results,
//             [
//                 Err(ValidErr::WithElement(0, Rc::from("b"))),
//                 Err(ValidErr::WithElement(1, Rc::from("b"))),
//                 Ok(2),
//                 Err(ValidErr::WithElement(3, Rc::from("e"))),
//                 Ok(4),
//                 Err(ValidErr::WithElement(5, Rc::from("e"))),
//                 Ok(6),
//                 Err(ValidErr::WithElement(7, Rc::from("am"))),
//                 Err(ValidErr::WithElement(8, Rc::from("am"))),
//                 Err(ValidErr::WithElement(9, Rc::from("am"))),
//                 Err(ValidErr::WithElement(0, Rc::from("am"))),
//                 Err(ValidErr::WithElement(1, Rc::from("am"))),
//                 Err(ValidErr::WithElement(2, Rc::from("am"))),
//                 Err(ValidErr::WithElement(3, Rc::from("am"))),
//                 Err(ValidErr::WithElement(4, Rc::from("am"))),
//                 Err(ValidErr::WithElement(5, Rc::from("am"))),
//                 Err(ValidErr::WithElement(6, Rc::from("am"))),
//                 Err(ValidErr::WithElement(7, Rc::from("am"))),
//                 Err(ValidErr::WithElement(8, Rc::from("am"))),
//                 Err(ValidErr::WithElement(9, Rc::from("am"))),
//                 Err(ValidErr::WithElement(-1, Rc::from("co"))),
//                 Err(ValidErr::WithElement(1, Rc::from("lb"))),
//                 Err(ValidErr::Description(Rc::from("al"))),
//             ]
//         )
//     }
// }
