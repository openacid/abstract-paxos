//! Implement classic-paxos with abstract-paxos
//!
//! - [`Time`] in paxos is ballot-number, a monotonic incremental integer.
//! - [`QuorumSet`] is a simple [`Majority`].
//! - Network [`Transport`] is implemented with direct function call to an
//!   [`Acceptor`].
//! - To rebuild a **maybe committed** value with [`Distribute`], it just use
//!   the one with max `v_ballot`.

use crate::commonly_used::quorum_set::majority::Majority;
use crate::commonly_used::rebuild::Mirrored;
use crate::commonly_used::transport::DirectCall;
use crate::Types;

/// Implement classic-paxos with abstract-paxos
#[derive(Debug, Clone)]
struct Paxos {}

impl Types for Paxos {
    type Time = u64;
    type Value = String;
    type Part = String;
    type QuorumSet = Majority<Paxos>;
    type Transport = DirectCall<Paxos>;
    type Distribute = Mirrored<Paxos>;
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use crate::apaxos::acceptor::Acceptor;
    use crate::apaxos::proposer::Proposer;
    use crate::commonly_used::quorum_set::majority::Majority;
    use crate::commonly_used::rebuild::Mirrored;
    use crate::commonly_used::transport::DirectCall;
    use crate::implementations::paxos::Paxos;
    use crate::APaxos;

    #[test]
    fn test_paxos() {
        //

        let acceptor_ids = [1, 2, 3];

        let mut acceptors = BTreeMap::new();
        for id in acceptor_ids {
            acceptors.insert(id, Acceptor::default());
        }

        let quorum_set = Majority::new(acceptor_ids);
        let transport = DirectCall::new(acceptors.clone());
        let rebuild = Mirrored::<Paxos>::new();

        let mut apaxos = APaxos::<Paxos>::new(acceptor_ids, quorum_set, rebuild, transport);

        let mut proposer = Proposer::new(&mut apaxos, 5, "hello".to_string());
        let committed = proposer.run();

        assert_eq!(committed.propose_time, 5);
        assert_eq!(committed.data, "hello".to_string());

        let mut proposer = Proposer::new(&mut apaxos, 6, "world".to_string());
        let committed = proposer.run();

        assert_eq!(committed.propose_time, 5);
        assert_eq!(committed.data, "hello".to_string());

        // TODO: rebuild from previous value
    }
}
