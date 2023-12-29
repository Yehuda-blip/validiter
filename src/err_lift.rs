use crate::{valid_iter::ValidIter, valid_result::ValidErr};

pub struct ErrLift<OkType, ErrType, I>
where
    I: Iterator<Item = Result<OkType, ValidErr<ErrType>>> + Sized,
{
    iter: I,
}

impl<OkType, ErrType, I> ErrLift<OkType, ErrType, I>
where
    I: Iterator<Item = Result<OkType, ValidErr<ErrType>>> + Sized,
{
    pub(crate) fn new(iter: I) -> Self {
        Self { iter }
    }
}

impl<OkType, ErrType, I> Iterator for ErrLift<OkType, ErrType, I>
where
    I: Iterator<Item = Result<OkType, ValidErr<ErrType>>> + Sized,
{
    type Item = Result<OkType, ValidErr<OkType>>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(Err(_err_type)) => Some(Err(ValidErr::LiftedErr)),
            Some(Ok(ok_type)) => Some(Ok(ok_type)),
            None => None,
        }
    }
}

impl<OkType, ErrType, I> ValidIter for ErrLift<OkType, ErrType, I>
where
    I: Iterator<Item = Result<OkType, ValidErr<ErrType>>> + Sized,
{
    type BaseType = OkType;
}

pub trait ErrLiftable<OkType, ErrType>:
    Iterator<Item = Result<OkType, ValidErr<ErrType>>> + Sized
{
    fn lift(self) -> ErrLift<OkType, ErrType, Self> {
        ErrLift::new(self)
    }
}

impl<OkType, ErrType, I> ErrLiftable<OkType, ErrType> for I where
    I: Iterator<Item = Result<OkType, ValidErr<ErrType>>> + Sized
{
}
