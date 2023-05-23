pub mod quorum_set;
pub mod rebuild;
pub mod time;
pub mod transport;
pub mod value;

use crate::AcceptorId;

impl AcceptorId for u64 {}
