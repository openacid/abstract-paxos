use std::marker::PhantomData;

use crate::AcceptedValue;
use crate::Rebuild;
use crate::Types;

#[derive(Default)]
pub struct Mirror<T: Types> {
    _p: PhantomData<T>,
}

impl<T: Types> Mirror<T> {
    pub fn new() -> Self {
        Self { _p: Default::default() }
    }
}

impl<T: Types> Rebuild<T> for Mirror<T> {
    fn rebuild<'a>(
        &mut self,
        vs: impl IntoIterator<Item = (&'a T::AcceptorId, &'a AcceptedValue<T>)>,
    ) -> Option<T::Value> {
        let mut max: Option<AcceptedValue<T>> = None;

        for (_, accepted_value) in vs.into_iter() {
            if Some(accepted_value.v_time) > max.as_ref().map(|x| x.v_time) {
                max = Some(accepted_value.clone());
            }
        }

        max.map(|x| x.value)
    }
}
