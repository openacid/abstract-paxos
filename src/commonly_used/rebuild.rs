use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::marker::PhantomData;

use crate::AcceptorId;
use crate::Distribute;
use crate::Types;
use crate::Value;

/// Distribute a proposal by mirror copying and rebuild a **maybe committed**
/// proposal from accepted, mirror copied proposals.
///
/// In such a scenario, every acceptor stores a copy of the proposal data.
/// E.g.:
/// ```text
/// | p | v  v  v  v
/// ----|------------
/// | a | a1 a2 a3 a4
/// ```
#[derive(Default)]
pub struct Mirrored<T: Types> {
    _p: PhantomData<T>,
}

impl<T: Types> Mirrored<T> {
    pub fn new() -> Self {
        Self {
            _p: Default::default(),
        }
    }
}

impl<T: Types> Distribute<T> for Mirrored<T>
where
    T::Value: From<T::Part>,
    T::Part: From<T::Value>,
{
    fn distribute<'a>(
        &mut self,
        v: T::Value,
        acceptor_ids: impl IntoIterator<Item = &'a T::AcceptorId>,
    ) -> Vec<T::Part> {
        acceptor_ids.into_iter().map(|_| T::Part::from(v.clone())).collect()
    }

    fn rebuild<'a>(
        &mut self,
        vs: impl IntoIterator<Item = (&'a T::AcceptorId, &'a T::Part)>,
    ) -> Option<T::Value> {
        vs.into_iter().map(|(_, v)| T::Value::from(v.clone())).next()
    }
}

/// Distribute a proposal value by splitting into parts, and rebuild a **maybe
/// committed** proposal using partitioned acceptor states.
///
/// In this situation, every acceptor holds a unique portion of the proposal
/// data, requiring a Reader to access all acceptors to retrieve the proposal
/// data. This is commonly seen in the 2PC (Two-Phase Commit) protocol.
#[derive(Default)]
pub struct Partitioned<T: Types> {
    acceptor_ids: BTreeSet<T::AcceptorId>,
}

impl<T: Types> Partitioned<T> {
    pub fn new(acceptor_ids: impl IntoIterator<Item = impl Into<T::AcceptorId>>) -> Self {
        Self {
            acceptor_ids: acceptor_ids.into_iter().map(|x| x.into()).collect(),
        }
    }
}

impl<AID, P, T: Types<Value = BTreeMap<AID, P>, AcceptorId = AID, Part = P>> Distribute<T>
    for Partitioned<T>
where
    AID: AcceptorId,
    P: Value,
{
    fn distribute<'a>(
        &mut self,
        v: T::Value,
        acceptor_ids: impl IntoIterator<Item = &'a T::AcceptorId>,
    ) -> Vec<T::Part> {
        acceptor_ids.into_iter().map(|id| v[id].clone()).collect()
    }

    fn rebuild<'a>(
        &mut self,
        vs: impl IntoIterator<Item = (&'a T::AcceptorId, &'a T::Part)>,
    ) -> Option<T::Value> {
        let vs: BTreeMap<T::AcceptorId, T::Part> =
            vs.into_iter().map(|(id, v)| (*id, v.clone())).collect::<BTreeMap<_, _>>();

        let keys = vs.keys().copied().collect::<BTreeSet<_>>();
        if keys != self.acceptor_ids {
            assert_eq!(
                0,
                keys.difference(&self.acceptor_ids).count(),
                "acceptor ids mismatch"
            );
            return None;
        }

        Some(vs)
    }
}
