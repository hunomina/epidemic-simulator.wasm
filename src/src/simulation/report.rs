use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Report {
    pub healthy: usize,
    pub sick: usize,
    pub recovered: usize,
}

impl Report {
    pub fn empty() -> Self {
        Report {
            healthy: 0,
            recovered: 0,
            sick: 0,
        }
    }
}
