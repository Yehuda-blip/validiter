use std::fmt::{Debug, Display};

use crate::{
    msg::MsgPusher,
    valid_iter::ValidIter,
    valid_result::{VResult, ValidErr},
};

#[derive(Debug, Clone)]
pub struct ConstOver<I, A, M>
where
    I: ValidIter + Iterator<Item = VResult<I::BaseType>>,
    A: PartialEq,
    M: Fn(&I::BaseType) -> A,
{
    iter: I,
    stored_value: Option<A>,
    extractor: M,
}

impl<I, A, M> ConstOver<I, A, M>
where
    I: ValidIter + Iterator<Item = VResult<I::BaseType>>,
    A: PartialEq,
    M: Fn(&I::BaseType) -> A,
{
    pub(crate) fn new(iter: I, extractor: M) -> ConstOver<I, A, M> {
        Self {
            iter,
            stored_value: None,
            extractor,
        }
    }

    pub fn msg(
        self,
        msg: &str,
    ) -> MsgPusher<
        Self,
        impl Fn(
            &Self,
            ValidErr<<Self as ValidIter>::BaseType>,
        ) -> ValidErr<<Self as ValidIter>::BaseType>,
    > {
        let msg = String::from(msg);
        return MsgPusher::new(self, move |_, verr| match verr {
            ValidErr::BrokenConstant { element, msg: None } => ValidErr::BrokenConstant {
                element,
                msg: Some(msg.to_owned()),
            },
            other => other,
        });
    }

    pub fn auto_msg(
        self,
    ) -> MsgPusher<
        Self,
        impl Fn(
            &Self,
            ValidErr<<Self as ValidIter>::BaseType>,
        ) -> ValidErr<<Self as ValidIter>::BaseType>,
    >
    where
        A: Display,
    {
        MsgPusher::new(self, move |self_ref, verr| match verr {
            ValidErr::BrokenConstant { element, msg: None } => {
                let element_eval = (&self_ref.extractor)(&element);
                // string cache in MsgPusher?
                let stored_value_str = match &self_ref.stored_value {
                    Some(sv) => sv.to_string(),
                    None => "(no stored value)".to_owned()
                };
                ValidErr::BrokenConstant {
                    element,
                    msg: Some(format!(
                        "element evaluates to {}, should be {}",
                        element_eval, stored_value_str
                    )),
                }
            }
            other => other,
        })
    }

    pub fn auto_msg_plus(
        self,
        msg: &str,
    ) -> MsgPusher<
        Self,
        impl Fn(
            &Self,
            ValidErr<<Self as ValidIter>::BaseType>,
        ) -> ValidErr<<Self as ValidIter>::BaseType>,
    >
    where
        A: Display,
    {
        let stored_val_string = match &self.stored_value {
            Some(val) => format!("{}", val),
            None => "(no stored value)".to_string(),
        };
        let msg = String::from(msg);
        MsgPusher::new(self, move |self_ref, verr| match verr {
            ValidErr::BrokenConstant { element, msg: None } => {
                let element_eval = (self_ref.extractor)(&element);
                ValidErr::BrokenConstant {
                    element,
                    msg: Some(format!(
                        "element evaluates to {}, should be {} - {}",
                        element_eval,
                        stored_val_string,
                        msg.to_string()
                    )),
                }
            }
            other => other,
        })
    }

    pub fn auto_msg_debug(
        self,
    ) -> MsgPusher<
        Self,
        impl Fn(
            &Self,
            ValidErr<<Self as ValidIter>::BaseType>,
        ) -> ValidErr<<Self as ValidIter>::BaseType>,
    >
    where
        A: Debug,
    {
        let stored_val_string = match &self.stored_value {
            Some(val) => format!("{:?}", val),
            None => "(no stored value)".to_string(),
        };
        MsgPusher::new(self, move |self_ref, verr| match verr {
            ValidErr::BrokenConstant { element, msg: None } => {
                let element_eval = (self_ref.extractor)(&element);
                ValidErr::BrokenConstant {
                    element,
                    msg: Some(format!(
                        "element evaluates to {:?}, should be {}",
                        element_eval, stored_val_string
                    )),
                }
            }
            other => other,
        })
    }

    pub fn auto_msg_debug_plus(
        self,
        msg: &str,
    ) -> MsgPusher<
        Self,
        impl Fn(
            &Self,
            ValidErr<<Self as ValidIter>::BaseType>,
        ) -> ValidErr<<Self as ValidIter>::BaseType>,
    >
    where
        A: Debug,
    {
        let stored_val_string = match &self.stored_value {
            Some(val) => format!("{:?}", val),
            None => "(no stored value)".to_string(),
        };
        let msg = String::from(msg);
        MsgPusher::new(self, move |self_ref, verr| match verr {
            ValidErr::BrokenConstant { element, msg: None } => {
                let element_eval = (self_ref.extractor)(&element);
                ValidErr::BrokenConstant {
                    element,
                    msg: Some(format!(
                        "element evaluates to {:?}, should be {} - {}",
                        element_eval, stored_val_string, msg
                    )),
                }
            }
            other => other,
        })
    }
}

impl<I, A, M> Iterator for ConstOver<I, A, M>
where
    I: ValidIter + Iterator<Item = VResult<I::BaseType>>,
    A: PartialEq,
    M: Fn(&I::BaseType) -> A,
{
    type Item = VResult<I::BaseType>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(Ok(val)) => match &self.stored_value {
                Some(expected_const) => match (self.extractor)(&val) == *expected_const {
                    true => Some(Ok(val)),
                    false => Some(Err(ValidErr::BrokenConstant {
                        element: val,
                        msg: None,
                    })),
                },
                None => {
                    self.stored_value = Some((self.extractor)(&val));
                    Some(Ok(val))
                }
            },
            other => other,
        }
    }
}

impl<I, A, M> ValidIter for ConstOver<I, A, M>
where
    I: ValidIter + Iterator<Item = VResult<I::BaseType>>,
    A: PartialEq,
    M: Fn(&I::BaseType) -> A,
{
    type BaseType = I::BaseType;
}

#[cfg(test)]
mod tests {
    use std::iter::repeat;

    use crate::{
        valid_iter::{Unvalidatable, ValidIter},
        valid_result::ValidErr,
    };

    #[test]
    fn test_const_over_ok() {
        if repeat(1)
            .take(5)
            .validate()
            .const_over(|i| *i)
            .any(|res| res.is_err())
        {
            panic!("const over failed on constant iteration")
        }
    }

    #[test]
    fn test_const_over_err() {
        let results: Vec<_> = [0, 0, 0, 1]
            .into_iter()
            .validate()
            .const_over(|i| *i)
            .collect();
        assert_eq!(
            results,
            [
                Ok(0),
                Ok(0),
                Ok(0),
                Err(ValidErr::BrokenConstant {
                    element: 1,
                    msg: None
                })
            ]
        )
    }

    #[test]
    fn test_const_over_bounds() {
        if (0..0).validate().const_over(|i| *i).any(|res| res.is_err()) {
            panic!("const over failed on empty iter")
        }

        if (0..1).validate().const_over(|i| *i).any(|res| res.is_err()) {
            panic!("const over failed on count == 1 iter")
        }
    }

    #[test]
    fn test_const_over_all_elements_are_present_and_in_order() {
        let results: Vec<_> = [[0], [0], [0], [1], [0], [2]]
            .into_iter()
            .validate()
            .const_over(|slice| slice[0])
            .collect();
        assert_eq!(
            results,
            [
                Ok([0]),
                Ok([0]),
                Ok([0]),
                Err(ValidErr::BrokenConstant {
                    element: [1],
                    msg: None
                }),
                Ok([0]),
                Err(ValidErr::BrokenConstant {
                    element: [2],
                    msg: None
                })
            ]
        )
    }
}
