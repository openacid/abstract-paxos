#![feature(associated_type_defaults)]

pub mod apaxos;
pub mod commonly_used;
pub mod implementations;

use std::collections::BTreeMap;
use std::fmt::Debug;

use apaxos::proposal::Proposal;
use apaxos::ptime::Time;

use crate::apaxos::acceptor::Acceptor;

pub trait AcceptorId: Debug + Clone + Copy + Ord + 'static {}

pub trait Value: Debug + Clone + 'static {}

/// Defines types that are used in the Abstract-Paxos algorithm.
pub trait Types: Debug + Clone + Sized + 'static {
    /// Acceptor ID
    type AcceptorId: AcceptorId = u64;

    /// Pseudo time used in a distributed consensus.
    ///
    /// Every distributed consensus algorithm has its own definition of time.
    /// - In Paxos, it is ballot number, which is `(round, proposer_id)`.
    /// - In Raft, it is `(term, Option<voted_for>)`.
    /// - In 2PC, it is mainly a vector of related data entry name.
    // TODO: explain 2pc time.
    type Time: Time;

    /// The value to propose and to commit
    type Value: Value;

    /// A part of the [`Proposal`] data that is stored on an [`Acceptor`].
    type Part: Value;

    /// Quorum set defines quorums for read and write.
    ///
    /// Read-quorum is used by phase-1, write-quorum is used by phase-2.
    /// In most cases, read-quorum and write-quorum are the same.
    ///
    /// A quorum set defines the cluster structure.
    // TODO: explain cluster structure
    type QuorumSet: QuorumSet<Self>;

    /// The network transport for sending and receiving messages.
    type Transport: Transport<Self>;

    /// The distribution algorithm for distributing a value to several acceptors
    /// and for rebuilding the value from accepted value parts.
    type Distribute: Distribute<Self>;
}

pub trait Transport<T: Types> {
    fn send_phase1_request(&mut self, target: T::AcceptorId, t: T::Time);
    fn recv_phase1_reply(&mut self) -> (T::AcceptorId, Acceptor<T>);

    fn send_phase2_request(
        &mut self,
        target: T::AcceptorId,
        t: T::Time,
        proposal: Proposal<T, T::Part>,
    );
    fn recv_phase2_reply(&mut self) -> (T::AcceptorId, bool);
}

/// Defines the distribution policy for storing portions of a value on several
/// Acceptor-s.
///
/// This trait is responsible to split the [`Proposal`] into several `Part`s,
/// each part for every Acceptor, and to rebuild a [`Proposal`] from `Part`s
pub trait Distribute<T: Types> {
    /// Distribute a value to several [`Acceptor`];
    fn distribute<'a>(
        &mut self,
        value: T::Value,
        acceptor_ids: impl IntoIterator<Item = &'a T::AcceptorId>,
    ) -> Vec<T::Part>;

    fn rebuild<'a>(
        &mut self,
        x: impl IntoIterator<Item = (&'a T::AcceptorId, &'a T::Part)>,
    ) -> Option<T::Value>;
}

pub trait QuorumSet<T: Types> {
    fn is_read_quorum(&self, acceptor_ids: impl IntoIterator<Item = T::AcceptorId>) -> bool;
    fn is_write_quorum(&self, acceptor_ids: impl IntoIterator<Item = T::AcceptorId>) -> bool;
}

pub struct APaxos<T: Types> {
    acceptors: BTreeMap<T::AcceptorId, ()>,

    quorum_set: T::QuorumSet,
    rebuild: T::Distribute,
    transport: T::Transport,
}

impl<T: Types> APaxos<T> {
    pub fn new(
        acceptors: impl IntoIterator<Item = T::AcceptorId>,
        quorum_set: T::QuorumSet,
        rebuild: T::Distribute,
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
