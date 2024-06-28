use validiter::{Unvalidatable, ValidErr, ValidIter};

enum ValidIterError<T> {
    NoElement,
    Odd(T),
    Even(T),
}

impl<T> ValidIterError<T> {
    const fn to_validiter_identity(&self) -> &'static str {
        match self {
            // using single-character non descriptive string,
            // because we will match against a shorter string,
            // and the description is the variant.
            ValidIterError::NoElement => "A",
            ValidIterError::Odd(_) => "B",
            ValidIterError::Even(_) => "C",
        }
    }

    fn from_validerr(err: ValidErr<T>) -> Self {
        let iden_to_variant = |iden| match iden {
            "A" => Self::NoElement,
            _ => panic!("unrecognized error identifier without element"),
        };
        let iden_with_elmt_to_variant = |elmt, iden| match iden {
            "B" => Self::Odd(elmt),
            "C" => Self::Even(elmt),
            _ => panic!("unrecognized error identifier with element"),
        };
        match err {
            ValidErr::Description(identity) => iden_to_variant(&identity.as_ref()),
            ValidErr::WithElement(elmt, identity) => {
                iden_with_elmt_to_variant(elmt, &identity.as_ref())
            }
        }
    }
}

fn main() {
    let iteration = (0..10)
        .validate()
        .ensure(
            |i| i % 2 == 0,
            ValidIterError::Odd(0).to_validiter_identity(),
        )
        .ensure(
            |i| i % 2 == 1,
            ValidIterError::Even(0).to_validiter_identity(),
        );
    for elmt in iteration {
        match elmt {
            Ok(val) => print!("{val}"),
            Err(ValidErr::Description(_)) => panic!("no description variants were created"),
            Err(error) => match ValidIterError::from_validerr(error) {
                ValidIterError::NoElement => panic!("found a no-element error"),
                ValidIterError::Even(val) => panic!("found even value {val}"),
                ValidIterError::Odd(val) => panic!("found odd value {val}"),
            },
        }
    }
}
