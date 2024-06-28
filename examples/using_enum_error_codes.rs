use validiter::{Unvalidatable, ValidErr, ValidIter};

enum ValidIterError {
    Odd,
    Even,
}

impl ValidIterError {
    const fn to_validiter_identity(&self) -> &'static str {
        match self {
            // using single-character non descriptive string,
            // because we will match against a shorter string,
            // and the description is the variant.
            ValidIterError::Odd => "A",
            ValidIterError::Even => "B",
        }
    }

    fn from_validerr<T>(err: ValidErr<T>) -> Self {
        let iden_to_variant = |iden| match iden {
            "A" => Self::Odd,
            "B" => Self::Even,
            _ => panic!("unrecognized error identifier"),
        };
        match err {
            ValidErr::Description(identity) => iden_to_variant(&identity.as_ref()),
            ValidErr::WithElement(_, identity) => iden_to_variant(&identity.as_ref()),
        }
    }
}

fn main() {
    let iteration = (0..10)
        .validate()
        .ensure(|i| i % 2 == 0, ValidIterError::Odd.to_validiter_identity())
        .ensure(|i| i % 2 == 1, ValidIterError::Even.to_validiter_identity());
    for elmt in iteration {
        match elmt {
            Ok(val) => print!("{val}"),
            Err(ValidErr::Description(_)) => panic!("no description variants were created"),
            Err(error) => match ValidIterError::from_validerr(error) {
                ValidIterError::Even => panic!("found even value"),
                ValidIterError::Odd => panic!("found odd value")
            }
        }
    }
}
