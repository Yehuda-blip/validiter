use crate::valid_err::ValidErr;

pub struct Between<I: Iterator>
where
    I::Item: PartialOrd,
{
    iter: I,
    lower_bound: I::Item,
    upper_bound: I::Item,
}

impl<I: Iterator<Item = impl PartialOrd>> Between<I> {
    pub(crate) fn new(iter: I, lower_bound: I::Item, upper_bound: I::Item) -> Self {
        Self {
            iter,
            lower_bound,
            upper_bound,
        }
    }
}

impl<I: Iterator<Item = impl PartialOrd>> Iterator for Between<I> {
    type Item = Result<I::Item, ValidErr<I>>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(val) => match val >= self.lower_bound && val < self.upper_bound {
                true => Some(Ok(val)),
                false => Some(Err(ValidErr::OutOfBounds(val))),
            },
            None => None,
        }
    }
}
