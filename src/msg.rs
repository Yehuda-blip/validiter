use crate::{VResult, ValidErr, ValidIter};

pub struct MsgPusher<I, F>
where
    I: ValidIter + Iterator<Item = VResult<I::BaseType>>,
    F: FnMut(ValidErr<I::BaseType>) -> ValidErr<I::BaseType>,
{
    iter: I,
    pusher: F
}

impl<I, F> MsgPusher<I, F>
where
    I: ValidIter + Iterator<Item = VResult<I::BaseType>>,
    F: FnMut(ValidErr<I::BaseType>) -> ValidErr<I::BaseType>,
{
    pub(crate) fn new(iter: I, pusher: F) -> Self {
        Self { iter, pusher }
    }
}

impl<I, F> Iterator for MsgPusher<I, F>
where
    I: ValidIter + Iterator<Item = VResult<I::BaseType>>,
    F: FnMut(ValidErr<I::BaseType>) -> ValidErr<I::BaseType>,
{
    type Item = VResult<I::BaseType>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(Err(val)) => Some(Err((self.pusher)(val))),
            other => other
        }
    }
}

impl<I, F> ValidIter for MsgPusher<I, F>
where
    I: ValidIter + Iterator<Item = VResult<I::BaseType>>,
    F: FnMut(ValidErr<I::BaseType>) -> ValidErr<I::BaseType>,
{
    type BaseType = I::BaseType;
}
