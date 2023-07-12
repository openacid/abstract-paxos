use std::collections::BTreeMap;
use std::collections::BTreeSet;

use crate::apaxos::accepted::Accepted;
use crate::apaxos::greater_equal::GreaterEqual;
use crate::apaxos::proposal::Proposal;
use crate::APaxos;
use crate::Distribute;
use crate::QuorumSet;
use crate::Transport;
use crate::Types;

pub struct Phase1<'a, T: Types> {
    pub apaxos: &'a mut APaxos<T>,

    /// The time of the [`Proposer`] that running [`Phase1`].
    pub time: T::Time,

    /// The set of acceptors that granted the [`Proposer`]'s [`Phase1`] request.
    pub granted: BTreeSet<T::AcceptorId>,

    /// The value part that the acceptor has accepted.
    ///
    /// These value parts are proposed by smaller [`Proposer`]s.
    pub previously_accepted: BTreeMap<T::AcceptorId, Accepted<T>>,
}

impl<'a, T: Types> Phase1<'a, T> {
    pub fn run(&mut self) -> Option<Proposal<T, T::Value>> {
        let apaxos = &mut self.apaxos;

        let mut sent = 0;

        for id in apaxos.acceptors.keys() {
            apaxos.transport.send_phase1_request(*id, self.time);
            sent += 1;
        }

        let mut max_accept_time = T::Time::default();
        let mut is_quorum = false;

        for _ in 0..sent {
            let (target, a) = self.apaxos.transport.recv_phase1_reply();
            dbg!("received phase-1 reply", &target, &a);
            if a.time != self.time {
                // Phase-2 request is rejected.
                continue;
            }

            self.granted.insert(target);
            is_quorum =
                is_quorum || self.apaxos.quorum_set.is_read_quorum(self.granted.iter().cloned());

            if let Some(accepted) = a.accepted {
                if accepted.accept_time.greater_equal(&max_accept_time) {
                    max_accept_time = accepted.accept_time;
                }

                self.previously_accepted.insert(target, accepted);
            }

            if is_quorum {
                if let Some(x) = self.rebuild(max_accept_time) {
                    return Some(x);
                }
            }
        }

        if is_quorum {
            None
        } else {
            unreachable!("TODO: no read quorum constituted")
        }
    }

    /// Rebuild the proposal using the parts from the replies.
    ///
    /// First, identify a reply with the max accept time, as this is the only
    /// proposal that can be committed.
    ///
    /// Second, find out the propose-time of that specific reply. Regardless of
    /// the accept-time, replies with the same propose-time originate from
    /// the same [`Proposer`] and are always compatible for reconstructing
    /// the original proposal.
    fn rebuild(&mut self, max_accept_time: T::Time) -> Option<Proposal<T, T::Value>> {
        let apaxos = &mut self.apaxos;

        let (_aid, accepted) = self
            .previously_accepted
            .iter()
            .filter(|(_, a)| a.accept_time == max_accept_time)
            .next()?;
        let propose_time = accepted.proposal.propose_time;

        let it = self
            .previously_accepted
            .iter()
            .filter(|(_, a)| a.proposal.propose_time == propose_time)
            .map(|(id, a)| (id, &a.proposal.data));
        let rebuilt_value = apaxos.rebuild.rebuild(it);
        rebuilt_value.map(|x| Proposal::new(propose_time, x))
    }
}
