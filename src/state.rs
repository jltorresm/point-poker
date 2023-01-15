use crate::game::Poker;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter};
use uuid::Uuid;

pub const KEY: &str = "state.point.poker";

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct State {
    pub theme: Theme,
    pub uuid: String,
    pub name: Option<String>,
    pub game: Option<Poker>,
}

impl Default for State {
    fn default() -> Self {
        State {
            theme: Theme::Dark,
            uuid: Uuid::new_v4().to_string(),
            name: None,
            game: None,
        }
    }
}

impl State {
    pub fn with_theme(&self, t: Theme) -> Self {
        State {
            theme: t,
            ..self.clone()
        }
    }

    pub fn with_name(&self, n: String) -> Self {
        State {
            name: Some(n),
            ..self.clone()
        }
    }

    pub fn with_game(&self, g: Poker) -> Self {
        State {
            game: Some(g),
            ..self.clone()
        }
    }
}

#[derive(Clone, Copy, Debug, EnumIter, Display, PartialEq, Eq, Serialize, Deserialize)]
pub enum Theme {
    Dark,
    Light,
}

impl Theme {
    pub fn icon(&self) -> &str {
        match self {
            Theme::Dark => "🌙",
            Theme::Light => "💡",
        }
    }
}
