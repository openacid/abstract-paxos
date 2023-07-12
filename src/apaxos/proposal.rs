use crate::Types;

/// A proposal is a value that is proposed by a Proposer.
///
/// `Proposal` is either an entire proposal or a part of a proposal that is
/// stored on an [`Acceptor`].
#[derive(Debug, Clone)]
pub struct Proposal<T: Types, D> {
    /// When the proposal is proposed, i.e. the time of the Proposer that
    /// **created** this proposal. This time won't change once is decided.
    pub propose_time: T::Time,

    /// The data of the proposal.
    ///
    /// It could be T::Value or T::Part.
    pub data: D,
}

impl<T: Types, D> Proposal<T, D> {
    pub fn new(propose_time: T::Time, data: D) -> Self {
        Self { propose_time, data }
    }
}
