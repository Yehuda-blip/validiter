Are you a rusty little thing?

Do you want to write 10,000-line functions and just chain iterators forever? 

Do you like to watch the compiler cry while it's trying to figure out WTF is a `Fold<Map<Filter<TakeWhile<Map<StepBy<Zip<SkipWhile...`

Do you get insecure about your ugly, disgusting, _imperative_ code when you're next to Haskellers?

Introducing...
# validiter

`validiter` is meant to provide a nice rusty api for performing validations on iterators. Here is a very simple example for an adapter it provides:
```
(0..10).validate().at_most(7) // wraps the first 7 elements in Ok(i32) and the rest in a Err(ValidErr::TooMany(i32))
```

Unfortunately, our beautiful and pure functions are often fouled by unsanitized data. So how can we allow data preprocessing to be integrated into a large, LAZY iterator chain? This is where validiter comes to help.
All in all, this crate is pretty simple - take an iterator, and declare that you want to validate it. There are 2 ways to do so:
1. Import `validiter::Unvalidatable` to your scope, and call `validate()` on said iterator - you can now call all of the `ValidIter` type adapters.
2. A little more scuffed, but also valid - if your iterator is already yielding results, you can map them to `ValidIter::ValidErr<the-type-you-want>::Mapped`,
   then import `validiter::ErrLiftable` into your scope and call `lift_errs` on the iterator - this too will allow you to call `ValidIter` methods.

We have examples for both of these methods. We'll start with a simple one, using the `validate` method (this is `multi_validated_iterator` in the `examples` folder):
```
use validiter::{Unvalidatable, ValidIter};

fn main() {
    // This is the standard way to use validiter - call validate on
    // some 'Unvalidatable' iterator, and then place restrictions
    // on the iteration. Notice that 'ValidErr' type errors are always
    // ignored by validiter adapters, so the order of validation
    // placement matters, if the iteration fails - there might be 
    // ignored errors, on elements that already failed a different
    // validation.
    (0..10)
        .validate()
        .at_most(7)
        .between(2, 8)
        .ensure(|i| i % 2 == 0)
        .at_least(4)
        .for_each(|v| println!("{:?}", v));
}
```
The second example is a bit more involved, using the `lift_errs` method (`numeric_csv_parsing` in the `examples` folder):
```
use validiter::{ErrLiftable, ValidErr, ValidIter};

fn main() {
    // In this example we will use the 'lift_errs' method to
    // create a 'Vec<Vec<f64>>' collection, while ensuring 
    // the mathematical validity if this collection as a numerical
    // matrix. We will also force the matrix to be non-negative,
    // just for funsies.

    // this is a CSV format str, with 2 rows and 2 columns
    let csv = "1.2, 3.0
                4.2, 0.5";

    // we'll use iterator methods on the CSV to build an actual 
    // split the csv by rows/lines
    let mat = csv        .lines()
        // convert each row to a matrix row
        .map(
            |line| {
                // split by elements
                line.split(",")
                    // map the elements to f64 values
                    .map(|s| s.trim())
                    // if we get a parse error, we want to map it to our own error types - ValidErr<f64>
                    .map(|s| s.parse::<f64>().map_err(|_| ValidErr::<f64>::Mapped))
                    // because 'Map' is not a 'ValidIter', we need to convert the underlying data structure type
                    .lift_errs() // the iterator is over VResult<f64>, but map is not a ValidIter!
                    // force non-empty rows
                    .at_least(1)
                    // simple 'greater than 0' validation
                    .ensure(|f| *f >= 0.0)
                    // collecting each row to a vector, but now Ok Type is a vector, but Err Type is f64!
                    .collect::<Result<Vec<f64>, ValidErr<f64>>>()
            },
        )
        // we use lift_errs again to fix the typing issues
        .lift_errs()
        // force non-empty matrix
        .at_least(1)
        // force equal-sized rows
        .const_over(|vec| vec.len())
        // collect into a matrix
        .collect::<Result<Vec<_>, _>>();
    assert_eq!(mat, Ok(vec![vec![1.2, 3.0], vec![4.2, 0.5]]));
    print!("{:?}", mat)
}
```

Most of the documentation for this crate is just the docstrings for the various methods of the `ValidIter` trait, so that's (probably) the place to go if you're unsure about what some validation adapter does.

The things that validiter does not (yet) support and you might want, but won't
get, are:
- Error messages inside the ValidErr type.
- Tight compatability with the `anyhow` crate (https://docs.rs/anyhow/latest/anyhow/).

I'd love to hear suggestions about anything - ESPECIALLY if you think I'm doing something wrong in the definition or implementation of this crate.


## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
