use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter};

#[derive(Clone, Copy, Debug, EnumIter, Display, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum VoteType {
    UpDown,
    FistOfFive,
    #[default]
    Fibonacci,
}

impl VoteType {
    #[must_use]
    pub fn options(&self) -> Vec<i32> {
        match self {
            VoteType::UpDown => vec![-1, 0, 1],
            VoteType::FistOfFive => vec![0, 1, 2, 3, 4, 5],
            VoteType::Fibonacci => vec![0, 1, 2, 3, 5, 8, 13, 21],
        }
    }
}
