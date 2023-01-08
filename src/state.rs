use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter};

pub const KEY: &str = "state.point.poker";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct State {
    pub theme: Theme,
}

impl Default for State {
    fn default() -> Self {
        State { theme: Theme::Dark }
    }
}

impl State {
    #[allow(clippy::unused_self)]
    pub fn with_theme(self, t: Theme) -> Self {
        State { theme: t }
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
