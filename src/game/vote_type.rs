use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, EnumIter, Display)]
pub enum VoteType {
    UpDown,
    FistOfFive,
    Fibonacci,
}

impl Default for VoteType {
    fn default() -> Self {
        VoteType::Fibonacci
    }
}
