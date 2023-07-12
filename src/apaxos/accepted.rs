use crate::apaxos::proposal::Proposal;
use crate::Types;

/// The state that is accepted by an [`Acceptor`].
#[derive(Debug, Clone)]
pub struct Accepted<T: Types> {
    /// When the proposal is accepted.
    ///
    /// I.e., the time of the [`Proposer`] that **replicated** the value.
    /// In other words, it increases every time a new proposer replicates the
    /// value.
    pub accept_time: T::Time,

    /// The proposal data that is accepted by the [`Acceptor`].
    pub proposal: Proposal<T, T::Part>,
}
