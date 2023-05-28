# Abstract-Paxos

Abstract-Paxos unifies well-known distributed consensus algorithms (Paxos, Raft, 2PC, etc.) into a single approach.

-   V1(中文): [abstract-paxos-v1-cn](doc/built/zhihu/v1-cn.md).

    - Provides proofs and explanations.
    - Describes classic Paxos and Raft using Abstract-Paxos.

# Project Status

This repository includes the core of Abstract-Paxos and demo examples that
showcase how to implement various distributed consensus algorithms using
Abstract-Paxos.

Abstract-Paxos implements:

- [x] Classic-Paxos: [paxos](./src/implementations/paxos.rs).
- [x] Two-Phase-Commit: [two_pc](./src/implementations/two_pc.rs)
- [ ] Fast-Paxos
- [ ] CAS-Paxos
- [ ] EC-Paxos
- [ ] Raft

Refer to: [src/implementations](./src/implementations).
