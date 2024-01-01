mod at_least;
mod at_most;
mod between;
mod ensure;
mod look_back;
pub mod err_lift;
pub mod valid_iter;
pub mod valid_result;
mod validatable;

#[cfg(test)]
mod tests {
    use crate::{
        valid_iter::{Unvalidatable, ValidIter},
        valid_result::{VResult, ValidErr},
    };

    #[test]
    fn test_multi_validation_on_iterator() {
        let validation_results = (0..10)
            .validate()
            .at_most(7)
            .between(2, 8)
            .ensure(|i| i % 2 == 0)
            .at_least(4)
            .collect::<Vec<VResult<_>>>();
        assert_eq!(
            validation_results,
            [
                Err(ValidErr::OutOfBounds(0)),
                Err(ValidErr::OutOfBounds(1)),
                Ok(2),
                Err(ValidErr::Invalid(3)),
                Ok(4),
                Err(ValidErr::Invalid(5)),
                Ok(6),
                Err(ValidErr::TooMany(7)),
                Err(ValidErr::TooMany(8)),
                Err(ValidErr::TooMany(9)),
                Err(ValidErr::TooFew),
            ]
        )
    }
}
