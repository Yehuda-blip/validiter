Validiter is meant to provide a nice rusty api for performing validations on iterators. Here is a very simple example for an adapter it provides:
```
(0..10).at_most(7) // wraps the first 7 elements in Ok(i32) and the rest in a Err(ValidErr::TooMany(i32))
```

Ideally, the adapters in the crate should allow for validiter result type propogation, so that something like:
```
(0..10).between(2, 7).at_most(3)
```
would return an iterator where the first 2 elements are an `ValidErr::OutOfBound(i32)` type, the next 3 are `Ok(i32)`, then 2 `ValidErr::TooMany(i32)` and finally 3 more `ValidErr::OutOfBounds(i32)`,
rather than wrapping the inner values twice in a double result.
