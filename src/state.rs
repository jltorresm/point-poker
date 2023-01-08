use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter};

pub const KEY: &str = "state.point.poker";

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct State {
    pub theme: Theme,
    pub name: Option<String>,
}

impl Default for State {
    fn default() -> Self {
        State {
            theme: Theme::Dark,
            name: None,
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
