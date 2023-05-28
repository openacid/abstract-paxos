use std::collections::BTreeMap;

use crate::apaxos::greater_equal::GreaterEqual;
use crate::commonly_used::quorum_set::all::All;
use crate::commonly_used::rebuild::Partitioned;
use crate::commonly_used::transport::DirectCall;
use crate::Types;

/// Implement TwoPC with abstract-paxos
#[derive(Debug, Clone)]
struct TwoPC {}

/// Unlike Paxos rebuilding proposal from a single [`Acceptor`]'s data, TwoPC
/// can rebuild the proposal data from [`Acceptor`]s' data with different
/// `accept_time`(vballot in paxos), but with the same propose-time.
///
/// ```text
/// p3: rebuild from v2,v1,v1,v1,v1; because they have the same propose-time
///
/// p3 |
///    |  +------------+       a1..a5
/// p2 |        v2
///    |        +------------+ a3..a7
/// p1 |        v1 v1 v1 v1 v1
///    |        +------------+ a3..a7
/// ---|----------------------
///    |  a1 a2 a3 a4 a5 a6 a7
/// ```
impl Types for TwoPC {
    type Time = TwoPCTime;
    type Value = BTreeMap<u64, String>;
    type Part = String;
    type QuorumSet = All<TwoPC>;
    type Transport = DirectCall<TwoPC>;
    type Distribute = Partitioned<TwoPC>;
}

/// The pseudo time for TwoPC represents the mutual exclusion between
/// transactions.
///
/// There is a GE relation if either:
/// 1. they belong to the same txn, or
/// 2. the other txn has not yet locked the resource.
///
/// TwoPCTime is not transitive and can form a cycle.
#[derive(Clone, Copy, Debug, Default)]
#[derive(PartialEq, Eq)]
struct TwoPCTime {
    txn_id: u64,
    locked: bool,
}

/// There is a GE relation if either:
/// 1. they belong to the same txn, or
/// 2. the other txn has not yet locked the resource.
impl GreaterEqual for TwoPCTime {
    fn greater_equal(&self, other: &Self) -> bool {
        self.txn_id == other.txn_id || !other.locked
    }
}

impl TwoPCTime {
    pub fn new(txn_id: u64) -> Self {
        Self {
            txn_id,
            locked: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use maplit::btreemap;

    use crate::apaxos::acceptor::Acceptor;
    use crate::apaxos::proposer::Proposer;
    use crate::commonly_used::quorum_set::all::All;
    use crate::commonly_used::rebuild::Partitioned;
    use crate::commonly_used::transport::DirectCall;
    use crate::implementations::two_pc::TwoPC;
    use crate::implementations::two_pc::TwoPCTime;
    use crate::APaxos;

    #[test]
    fn test_two_pc() {
        let acceptor_ids = [1, 2, 3];

        let mut acceptors = BTreeMap::new();
        for id in acceptor_ids {
            acceptors.insert(id, Acceptor::default());
        }

        let quorum_set = All::new(acceptor_ids);
        let transport = DirectCall::new(acceptors.clone());
        let rebuild = Partitioned::<TwoPC>::new(acceptor_ids);

        let mut apaxos = APaxos::<TwoPC>::new(acceptor_ids, quorum_set, rebuild, transport);

        let t1 = TwoPCTime::new(1);
        let tx1_data = btreemap! {1u64=>s("hello"), 2u64=>s("world"), 3u64=>s("")};
        let mut proposer = Proposer::new(&mut apaxos, t1, tx1_data.clone());
        let committed = proposer.run();

        assert_eq!(committed.propose_time, t1);
        assert_eq!(committed.data, tx1_data);

        // Tx2 is expected to fail:
        {
            let t2 = TwoPCTime::new(2);
            let tx2_data = btreemap! {1u64=>s("bye"), 2u64=>s("planet"), 3u64=>s("earth")};
            let mut proposer = Proposer::new(&mut apaxos, t2, tx2_data.clone());
            let committed = proposer.run();

            assert_eq!(committed.propose_time, t2);
            assert_eq!(committed.data, tx2_data);
        }
    }

    fn s(x: impl ToString) -> String {
        x.to_string()
    }
}
