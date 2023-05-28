use std::collections::BTreeSet;

use crate::QuorumSet;
use crate::Types;

/// Majority is the most simple **quorum** definition.
///
/// Any set of acceptors that has more than half of the whole set is a quorum.
/// So that every majority quorum intersect with each other.
/// And there is no differences between a read-quorum and a write-quorum.
pub struct Majority<T: Types> {
    acceptor_ids: BTreeSet<T::AcceptorId>,
}

impl<T: Types> Majority<T> {
    pub fn new(acceptor_ids: impl IntoIterator<Item = T::AcceptorId>) -> Self {
        Self {
            acceptor_ids: acceptor_ids.into_iter().collect(),
        }
    }
}

impl<T: Types> QuorumSet<T> for Majority<T> {
    fn is_read_quorum(&self, acceptor_ids: impl IntoIterator<Item = T::AcceptorId>) -> bool {
        let c = acceptor_ids.into_iter().filter(|x| self.acceptor_ids.contains(x)).count();
        c > self.acceptor_ids.len() / 2
    }

    fn is_write_quorum(&self, acceptor_ids: impl IntoIterator<Item = T::AcceptorId>) -> bool {
        self.is_read_quorum(acceptor_ids)
    }
}
