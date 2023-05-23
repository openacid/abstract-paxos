use crate::default_impl::quorum_set::Majority;
use crate::default_impl::rebuild::Mirror;
use crate::default_impl::transport::MQTransport;
use crate::Types;

#[derive(Clone)]
struct Paxos {}

impl Types for Paxos {
    type Time = u64;
    type Value = String;
    type QuorumSet = Majority<Paxos>;
    type Transport = MQTransport<Paxos>;
    type Rebuild = Mirror<Paxos>;
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use crate::default_impl::quorum_set::Majority;
    use crate::default_impl::rebuild::Mirror;
    use crate::default_impl::transport::MQTransport;
    use crate::implements::paxos::Paxos;
    use crate::APaxos;
    use crate::Acceptor;
    use crate::Proposer;

    #[test]
    fn test_paxos() {
        //

        let acceptor_ids = [1, 2, 3];

        let mut acceptors = BTreeMap::new();
        for id in acceptor_ids {
            acceptors.insert(id, Acceptor::default());
        }

        let quorum_set = Majority::new(acceptor_ids);
        let transport = MQTransport::new(acceptors.clone());
        let rebuild = Mirror::<Paxos>::new();

        let mut apaxos = APaxos::<Paxos>::new(acceptor_ids, quorum_set, rebuild, transport);

        let mut proposer = Proposer::new(&mut apaxos, 5, "hello".to_string());
        let committed = proposer.run();

        assert_eq!(committed, "hello".to_string());

        let mut proposer = Proposer::new(&mut apaxos, 6, "world".to_string());
        let committed = proposer.run();

        assert_eq!(committed, "hello".to_string());
    }
}
