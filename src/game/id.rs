use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Id(u32);

impl Display for Id {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Id {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let inner = s.parse::<u32>()?;
        Ok(inner.into())
    }
}

impl From<i32> for Id {
    #[allow(clippy::cast_sign_loss)]
    fn from(value: i32) -> Self {
        Self(value as u32)
    }
}
impl From<u32> for Id {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
