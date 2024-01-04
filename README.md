Validiter is meant to provide a nice rusty api for performing validations on iterators. Here is a very simple example for an adapter it provides:
```
(0..10).at_most(7) // wraps the first 7 elements in Ok(i32) and the rest in a Err(ValidErr::TooMany(i32))
```

Ideally, the adapters in the crate should allow for validiter result type propogation, so that something like:
```
(0..10).between(2, 7).at_most(3)
```
would return an iterator where the first 2 elements are an `ValidErr::OutOfBound(i32)` type, the next 3 are `Ok(i32)`, then 2 `ValidErr::TooMany(i32)` and finally 3 more `ValidErr::OutOfBounds(i32)`,
rather than wrapping the inner values twice in a double result. Unfortunately, this straightforward behaviour is probably impossible without the "trait specialization" feature of unstable rust, 
which would allow us to wrap only non `Result<_, ValidErr<_>>` types and provide specialized behaviour for already wrapped types. Whether this drawback is beneficial overall or not (it probably is), 
it forces us to create our own `ValidationSpace` type system, to which we send generic iterators, and in which every element in the iterator is already wrapped in the specialized `ValidResult` type.
And so together with the validation adapters, we need to provide senders to the "Any Type Space".



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