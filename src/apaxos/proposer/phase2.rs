use std::collections::BTreeMap;

use crate::apaxos::proposal::Proposal;
use crate::APaxos;
use crate::Distribute;
use crate::QuorumSet;
use crate::Transport;
use crate::Types;

pub struct Phase2<'a, T: Types> {
    pub apaxos: &'a mut APaxos<T>,

    /// The time of the Proposer that running phase1.
    pub time: T::Time,

    pub decided: Proposal<T, T::Value>,

    pub granted: BTreeMap<T::AcceptorId, ()>,
}

impl<'a, T: Types> Phase2<'a, T> {
    pub fn run(mut self) -> Proposal<T, T::Value> {
        let apaxos = &mut self.apaxos;

        let mut sent = 0;

        let acceptor_ids = apaxos.acceptors.keys();
        let parts = apaxos.rebuild.distribute(self.decided.data.clone(), acceptor_ids.clone());

        let id_parts = acceptor_ids.zip(parts);

        for (id, part) in id_parts {
            let p = Proposal::new(self.decided.propose_time, part);
            apaxos.transport.send_phase2_request(*id, self.time, p);
            sent += 1;
        }

        for _ in 0..sent {
            let (target, is_accepted) = apaxos.transport.recv_phase2_reply();
            if is_accepted {
                self.granted.insert(target, ());
            }

            if apaxos.quorum_set.is_write_quorum(self.granted.keys().cloned()) {
                return self.decided;
            }
        }

        unreachable!("not enough acceptors")
    }
}
