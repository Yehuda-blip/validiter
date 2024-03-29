mod at_least;
mod at_most;
mod between;
mod const_over;
mod ensure;
mod lift_errs;
mod look_back;
mod valid_iter;
mod valid_result;
mod validatable;

pub use lift_errs::ErrLiftable;
pub use valid_iter::{Unvalidatable, ValidIter};
pub use valid_result::{VResult, ValidErr};

#[cfg(test)]
mod tests {
    use crate::{
        valid_iter::{Unvalidatable, ValidIter},
        valid_result::{VResult, ValidErr},
    };

    #[test]
    fn test_multi_validation_on_iterator() {
        let validation_results = (0..10)
            .chain(0..10)
            .chain(-1..=-1)
            .chain(1..=1)
            .validate()
            .const_over(|i| *i >= 0)
            .look_back_n::<10, _, _, _>(|i| *i, |prev, curr| prev == curr)
            .at_most(7)
            .between(2, 8)
            .ensure(|i| i % 2 == 0)
            .at_least(4)
            .collect::<Vec<VResult<_>>>();
        assert_eq!(
            validation_results,
            [
                Err(ValidErr::OutOfBounds {
                    element: 0,
                    msg: None
                }),
                Err(ValidErr::OutOfBounds {
                    element: 1,
                    msg: None
                }),
                Ok(2),
                Err(ValidErr::Invalid {
                    element: 3,
                    msg: None
                }),
                Ok(4),
                Err(ValidErr::Invalid {
                    element: 5,
                    msg: None
                }),
                Ok(6),
                Err(ValidErr::TooMany {
                    element: 7,
                    msg: None
                }),
                Err(ValidErr::TooMany {
                    element: 8,
                    msg: None
                }),
                Err(ValidErr::TooMany {
                    element: 9,
                    msg: None
                }),
                Err(ValidErr::TooMany {
                    element: 0,
                    msg: None
                }),
                Err(ValidErr::TooMany {
                    element: 1,
                    msg: None
                }),
                Err(ValidErr::TooMany {
                    element: 2,
                    msg: None
                }),
                Err(ValidErr::TooMany {
                    element: 3,
                    msg: None
                }),
                Err(ValidErr::TooMany {
                    element: 4,
                    msg: None
                }),
                Err(ValidErr::TooMany {
                    element: 5,
                    msg: None
                }),
                Err(ValidErr::TooMany {
                    element: 6,
                    msg: None
                }),
                Err(ValidErr::TooMany {
                    element: 7,
                    msg: None
                }),
                Err(ValidErr::TooMany {
                    element: 8,
                    msg: None
                }),
                Err(ValidErr::TooMany {
                    element: 9,
                    msg: None
                }),
                Err(ValidErr::BrokenConstant {
                    element: -1,
                    msg: None
                }),
                Err(ValidErr::LookBackFailed {
                    element: 1,
                    msg: None
                }),
                Err(ValidErr::TooFew { msg: None }),
            ]
        )
    }
}
