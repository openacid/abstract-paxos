use phase1::Phase1;
use phase2::Phase2;

use crate::apaxos::proposal::Proposal;
use crate::APaxos;
use crate::Types;

mod phase1;
mod phase2;

pub struct Proposer<'a, T: Types> {
    apaxos: &'a mut APaxos<T>,
    time: T::Time,
    proposal: Proposal<T, T::Value>,
}

impl<'a, T: Types> Proposer<'a, T> {
    pub fn new(apaxos: &'a mut APaxos<T>, time: T::Time, value: T::Value) -> Self {
        Self {
            apaxos,
            time,
            proposal: Proposal::new(time, value),
        }
    }

    pub fn run(&mut self) -> Proposal<T, T::Value> {
        let maybe_committed = self.new_phase1().run();
        let committed = self.new_phase2(maybe_committed).run();

        committed
    }

    fn new_phase1(&mut self) -> Phase1<T> {
        Phase1 {
            apaxos: &mut self.apaxos,
            time: self.time,
            granted: Default::default(),
            previously_accepted: Default::default(),
        }
    }

    fn new_phase2(&mut self, maybe_committed: Option<Proposal<T, T::Value>>) -> Phase2<T> {
        Phase2 {
            apaxos: &mut self.apaxos,
            time: self.time,
            decided: maybe_committed.unwrap_or_else(|| self.proposal.clone()),
            granted: Default::default(),
        }
    }
}
