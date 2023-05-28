use std::fmt::Debug;
use std::fmt::Formatter;

use crate::apaxos::accepted::Accepted;
use crate::apaxos::greater_equal::GreaterEqual;
use crate::apaxos::proposal::Proposal;
use crate::Types;

#[derive(Clone)]
pub struct Acceptor<T: Types> {
    /// The time it has seen so far, i.e., the current time.
    pub time: T::Time,

    /// The state that is accepted by this [`Acceptor`].
    pub accepted: Option<Accepted<T>>,
}

impl<T: Types> Debug for Acceptor<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Acceptor")
            .field("time", &self.time)
            .field("accepted", &self.accepted)
            .finish()
    }
}

impl<T: Types> Default for Acceptor<T> {
    fn default() -> Self {
        Self {
            time: T::Time::default(),
            accepted: None,
        }
    }
}

impl<T: Types> Acceptor<T> {
    pub(crate) fn handle_phase1_request(&mut self, now: T::Time) -> Self {
        dbg!("handle_phase1_request", now, self.time);
        dbg!(now.greater_equal(&self.time));
        if now.greater_equal(&self.time) {
            self.time = now;
        }

        self.clone()
    }

    pub(crate) fn handle_phase2_request(
        &mut self,
        t: T::Time,
        proposal: Proposal<T, T::Part>,
    ) -> bool {
        dbg!("handle_phase2_request", t);
        if t.greater_equal(&self.time) {
            self.time = t;
            self.accepted = Some(Accepted {
                accept_time: t,
                proposal,
            });

            true
        } else {
            false
        }
    }
}
