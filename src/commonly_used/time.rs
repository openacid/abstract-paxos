pub struct BallotNumber {
    pub round: u64,
    pub leader: u64,
}

impl BallotNumber {
    pub fn new(round: u64, leader: u64) -> Self {
        Self { round, leader }
    }
}

// impl Time for u64 {}
