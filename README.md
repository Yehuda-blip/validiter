Are you a rusty little thing?

Do you want to write 10000-lines functions and just chain iterators forever? 

Do you like to watch the compiler cry while it's trying to figure out WTF is a `Fold<Map<Filter<TakeWhile<Map<StepBy<Zip<SkipWhile...`

Do you get insecure about your ugly, disgusting, _imperative_ code when your'e next to Haskellers?

Introducing...
# validiter

`validiter` is meant to provide a nice rusty api for performing validations on iterators. Here is a very simple example for an adapter it provides:
```
(0..10).validate().at_most(7) // wraps the first 7 elements in Ok(i32) and the rest in a Err(ValidErr::TooMany(i32))
```

Unfortunately, our beautiful and pure functions are often fouled by unsanitized data. Until validiter came along, this meant that sanitizing data
could not be part of a large, LAZY iterator chain (unless you add side effects to something that really shouldn't have them). This is where validiter
comes to help.
All in all, this crate is pretty simple - take an iterator, and declare that you want to validate it. There are 2 ways to do so:
1. Import `validiter::Unvalidatable` to your scope, and call `validate()` on said iterator - you can now call all of the `ValidIter` type adapters.
2. A little more scuffed, but also valid - if your iterator is already yielding results, you can map them to `ValidIter::ValidErr<the-type-you-want>::Mapped`,
   then import `validiter::ErrLiftable` into your scope and call `lift_errs` on the iterator - this too will allow you to call `ValidIter` methods.

There are some adapters already in the crate, that should handle most use cases. The things that validiter does not (yet) support and you might want, but won't
get, are:
- Error messages inside the ValidErr type.
- Tight compatability with the `anyhow` crate (https://docs.rs/anyhow/latest/anyhow/).

I'd love to hear suggestions about anything - ESPECIALLY if you think I'm doing somrthing wrong in the definition or implementation of this crate.


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
