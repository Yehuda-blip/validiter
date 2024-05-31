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
            .const_over(|i| *i >= 0, |_, _, _| "1".to_string())
            .look_back_n::<10, _, _, _, _>(|i| *i, |prev, curr| prev == curr, |_, _| "2".to_string())
            .at_most(7, |_, _| "3".to_string())
            .between(2, 8, |_, _, _| "4".to_string())
            .ensure(|i| i % 2 == 0, |_| "5".to_string())
            .at_least(4, |_, _| "6".to_string())
            .collect::<Vec<VResult<_>>>();
        assert_eq!(
            validation_results,
            [
                Err(ValidErr::OutOfBounds(0, "4".to_string())),
                Err(ValidErr::OutOfBounds(1, "4".to_string())),
                Ok(2),
                Err(ValidErr::Invalid(3, "5".to_string())),
                Ok(4),
                Err(ValidErr::Invalid(5, "5".to_string())),
                Ok(6),
                Err(ValidErr::TooMany(7, "3".to_string())),
                Err(ValidErr::TooMany(8, "3".to_string())),
                Err(ValidErr::TooMany(9, "3".to_string())),
                Err(ValidErr::TooMany(0, "3".to_string())),
                Err(ValidErr::TooMany(1, "3".to_string())),
                Err(ValidErr::TooMany(2, "3".to_string())),
                Err(ValidErr::TooMany(3, "3".to_string())),
                Err(ValidErr::TooMany(4, "3".to_string())),
                Err(ValidErr::TooMany(5, "3".to_string())),
                Err(ValidErr::TooMany(6, "3".to_string())),
                Err(ValidErr::TooMany(7, "3".to_string())),
                Err(ValidErr::TooMany(8, "3".to_string())),
                Err(ValidErr::TooMany(9, "3".to_string())),
                Err(ValidErr::BrokenConstant(-1, "1".to_string())),
                Err(ValidErr::LookBackFailed(1, "2".to_string())),
                Err(ValidErr::TooFew("6".to_string())),
            ]
        )
    }
}
