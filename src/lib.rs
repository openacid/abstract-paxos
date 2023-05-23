#![feature(associated_type_defaults)]

pub mod implements;
#[cfg(test)] mod tests;

pub mod default_impl;

use std::collections::BTreeMap;
use std::fmt::Debug;

pub trait AcceptorId: Clone + Copy + Ord + 'static {}
pub trait Time: Default + Clone + Copy + PartialOrd + 'static {}
pub trait Value: Debug + Clone + 'static {}

pub trait Types: Clone + Sized + 'static {
    type AcceptorId: AcceptorId = u64;

    type Time: Time;

    type Value: Value;

    type QuorumSet: QuorumSet<Self>;

    type Transport: Transport<Self>;

    type Rebuild: Rebuild<Self>;
}

pub trait Timer<T: Types> {
    fn now(&mut self) -> T::Time;
}

pub trait Transport<T: Types> {
    fn send_phase1_request(&mut self, target: T::AcceptorId, t: T::Time);
    fn recv_phase1_reply(&mut self) -> (T::AcceptorId, Acceptor<T>);

    fn send_phase2_request(&mut self, target: T::AcceptorId, t: T::Time, v: T::Value);
    fn recv_phase2_reply(&mut self) -> (T::AcceptorId, bool);
}

pub trait Rebuild<T: Types> {
    fn rebuild<'a>(
        &mut self,
        x: impl IntoIterator<Item = (&'a T::AcceptorId, &'a AcceptedValue<T>)>,
    ) -> Option<T::Value>;
}

pub trait QuorumSet<T: Types> {
    fn is_read_quorum(&self, acceptor_ids: impl IntoIterator<Item = T::AcceptorId>) -> bool;
    fn is_write_quorum(&self, acceptor_ids: impl IntoIterator<Item = T::AcceptorId>) -> bool;
}

pub struct APaxos<T: Types> {
    acceptors: BTreeMap<T::AcceptorId, ()>,

    quorum_set: T::QuorumSet,
    rebuild: T::Rebuild,

    transport: T::Transport,
}

impl<T: Types> APaxos<T> {
    pub fn new(
        acceptors: impl IntoIterator<Item = T::AcceptorId>,
        quorum_set: T::QuorumSet,
        rebuild: T::Rebuild,
        transport: T::Transport,
    ) -> Self {
        let acceptors = acceptors.into_iter().map(|id| (id, ())).collect();

        Self {
            acceptors,
            quorum_set,
            rebuild,
            transport,
        }
    }
}

pub struct Proposer<'a, T: Types> {
    apaxos: &'a mut APaxos<T>,
    time: T::Time,
    proposal: T::Value,
}

impl<'a, T: Types> Proposer<'a, T> {
    pub fn new(apaxos: &'a mut APaxos<T>, time: T::Time, proposal: T::Value) -> Self {
        Self { apaxos, time, proposal }
    }

    pub fn run(&mut self) -> T::Value {
        let maybe_committed = self.new_phase1().run();
        let committed = self.new_phase2(maybe_committed).run();

        committed
    }

    pub fn new_phase1(&mut self) -> Phase1<T> {
        Phase1 {
            apaxos: &mut self.apaxos,
            time: self.time,
            replies: Default::default(),
        }
    }

    pub fn new_phase2(&mut self, maybe_committed: Option<T::Value>) -> Phase2<T> {
        Phase2 {
            apaxos: &mut self.apaxos,
            time: self.time,
            decided: maybe_committed.unwrap_or_else(|| self.proposal.clone()),
            replies: Default::default(),
        }
    }
}

pub struct Phase1<'a, T: Types> {
    apaxos: &'a mut APaxos<T>,
    time: T::Time,

    replies: BTreeMap<T::AcceptorId, AcceptedValue<T>>,
}

impl<'a, T: Types> Phase1<'a, T> {
    pub fn run(&mut self) -> Option<T::Value> {
        let apaxos = &mut self.apaxos;

        let mut sent = 0;

        for id in apaxos.acceptors.keys() {
            apaxos.transport.send_phase1_request(*id, self.time);
            sent += 1;
        }

        for _ in 0..sent {
            let (target, a) = apaxos.transport.recv_phase1_reply();
            if a.time == self.time {
                if let Some(accepted) = a.accepted {
                    self.replies.insert(target, accepted);
                }
            }

            if apaxos.quorum_set.is_read_quorum(self.replies.keys().cloned()) {
                if let Some(x) = apaxos.rebuild.rebuild(self.replies.iter()) {
                    return Some(x);
                }
            }
        }

        None
    }
}

pub struct Phase2<'a, T: Types> {
    apaxos: &'a mut APaxos<T>,

    time: T::Time,
    decided: T::Value,

    replies: BTreeMap<T::AcceptorId, ()>,
}

impl<'a, T: Types> Phase2<'a, T> {
    pub fn run(mut self) -> T::Value {
        let apaxos = &mut self.apaxos;

        let mut sent = 0;

        for id in apaxos.acceptors.keys() {
            apaxos.transport.send_phase2_request(*id, self.time, self.decided.clone());
            sent += 1;
        }

        for _ in 0..sent {
            let (target, accepted) = apaxos.transport.recv_phase2_reply();
            if accepted {
                self.replies.insert(target, ());
            }

            if apaxos.quorum_set.is_write_quorum(self.replies.keys().cloned()) {
                return self.decided;
            }
        }

        unreachable!("not enough acceptors")
    }
}

#[derive(Clone)]
pub struct AcceptedValue<T: Types> {
    v_time: T::Time,
    value: T::Value,
}

#[derive(Clone)]
pub struct Acceptor<T: Types> {
    time: T::Time,
    accepted: Option<AcceptedValue<T>>,
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
    fn handle_phase1_request(&mut self, now: T::Time) -> Self {
        if now >= self.time {
            self.time = now;
        }

        self.clone()
    }

    fn handle_phase2_request(&mut self, t: T::Time, v: T::Value) -> bool {
        if t >= self.time {
            self.time = t;
            self.accepted = Some(AcceptedValue { v_time: t, value: v });

            true
        } else {
            false
        }
    }
}
