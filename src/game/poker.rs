use super::{id::Id, vote_type::VoteType};
use chrono::{DateTime, Utc};
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Poker {
    pub id: Id,
    pub vote_type: VoteType,
    pub started: DateTime<Utc>,
}

impl Poker {
    fn new_id() -> Id {
        rand::thread_rng().gen_range(100_000..1_000_000).into()
    }

    #[must_use]
    pub fn with_vote_type(&self, vote_type: VoteType) -> Self {
        Self {
            vote_type,
            ..self.clone()
        }
    }
}

impl Default for Poker {
    fn default() -> Self {
        Poker {
            id: Self::new_id(),
            vote_type: VoteType::default(),
            started: Utc::now(),
        }
    }
}
