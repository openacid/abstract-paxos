use std::collections::BTreeMap;
use std::collections::VecDeque;

use crate::apaxos::acceptor::Acceptor;
use crate::apaxos::proposal::Proposal;
use crate::Transport;
use crate::Types;

/// Simulate network transport by delegate RPC to local function calls.
pub struct DirectCall<T: Types> {
    acceptors: BTreeMap<T::AcceptorId, Acceptor<T>>,

    p1_replies: VecDeque<(T::AcceptorId, Acceptor<T>)>,
    p2_replies: VecDeque<(T::AcceptorId, bool)>,
}

impl<T: Types> DirectCall<T> {
    pub fn new(acceptors: BTreeMap<T::AcceptorId, Acceptor<T>>) -> Self {
        Self {
            acceptors,

            p1_replies: VecDeque::new(),
            p2_replies: VecDeque::new(),
        }
    }
}

impl<T: Types> Transport<T> for DirectCall<T> {
    fn send_phase1_request(&mut self, target: T::AcceptorId, t: T::Time) {
        dbg!("send_phase_request", target, t);
        let reply = self.acceptors.get_mut(&target).unwrap().handle_phase1_request(t);
        self.p1_replies.push_back((target, reply));
    }

    fn recv_phase1_reply(&mut self) -> (T::AcceptorId, Acceptor<T>) {
        self.p1_replies.pop_front().unwrap()
    }

    fn send_phase2_request(
        &mut self,
        target: T::AcceptorId,
        t: T::Time,
        proposal_part: Proposal<T, T::Part>,
    ) {
        let reply =
            self.acceptors.get_mut(&target).unwrap().handle_phase2_request(t, proposal_part);
        self.p2_replies.push_back((target, reply));
    }

    fn recv_phase2_reply(&mut self) -> (T::AcceptorId, bool) {
        self.p2_replies.pop_front().unwrap()
    }
}
