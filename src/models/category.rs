use std::{convert::Infallible, fmt::Display, str::FromStr};

use crate::{ReadOnlyString, ReadOnlyVec};
use serde::Deserialize;
use serde_with::{serde_as, DisplayFromStr};

/// A category.
#[serde_as]
#[derive(Debug, Clone, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Category {
    /// Numeric category ID.
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "categoryId")]
    pub id: u32,

    /// Name of the category.
    #[serde(rename = "categoryName")]
    pub name: ReadOnlyString,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CategoryPath(ReadOnlyVec<ReadOnlyString>);

impl FromStr for CategoryPath {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.split('|').map(|value| value.into()).collect()))
    }
}

impl Display for CategoryPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, item) in self.0.iter().enumerate() {
            write!(f, "{}", item)?;

            if i < (self.0.len() - 1) {
                write!(f, "|")?;
            }
        }

        Ok(())
    }
}
