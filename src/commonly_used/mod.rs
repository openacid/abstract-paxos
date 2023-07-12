//! Provides most used implementations of component traits.
//!
//! Such as `majority` as the quorum used in raft, or `u64` as the ballot number used in paxos.

pub mod quorum_set;
pub mod rebuild;
pub mod time;
pub mod transport;
pub mod value;

use crate::AcceptorId;

impl AcceptorId for u64 {}
