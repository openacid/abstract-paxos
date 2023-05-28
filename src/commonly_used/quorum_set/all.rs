use std::collections::BTreeSet;

use crate::QuorumSet;
use crate::Types;

/// All requires all acceptors to form a **quorum**.
pub struct All<T: Types> {
    acceptor_ids: BTreeSet<T::AcceptorId>,
}

impl<T: Types> All<T> {
    pub fn new(acceptor_ids: impl IntoIterator<Item = T::AcceptorId>) -> Self {
        Self {
            acceptor_ids: acceptor_ids.into_iter().collect(),
        }
    }
}

impl<T: Types> QuorumSet<T> for All<T> {
    fn is_read_quorum(&self, acceptor_ids: impl IntoIterator<Item = T::AcceptorId>) -> bool {
        let c = acceptor_ids.into_iter().filter(|x| self.acceptor_ids.contains(x)).count();
        c == self.acceptor_ids.len()
    }

    fn is_write_quorum(&self, acceptor_ids: impl IntoIterator<Item = T::AcceptorId>) -> bool {
        self.is_read_quorum(acceptor_ids)
    }
}
