use derive_more::Display;
use rust_decimal::Decimal;
use serde::Deserialize;
use serde_with::{serde_as, DisplayFromStr};
use std::{fmt::Display, ops::Deref};

/// A fixed-precision decimal number representation of a price value.
#[serde_as]
#[derive(Debug, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Price {
    #[serde_as(as = "DisplayFromStr")]
    value: Decimal,
    currency: Currency,
}

/// A currency type.
#[derive(Debug, Deserialize, Display, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Currency {
    #[display(fmt = "€")]
    #[serde(rename = "EUR")]
    Euro,
    #[display(fmt = "$")]
    #[serde(rename = "USD")]
    UsDollar,
    #[display(fmt = "HUF")]
    #[serde(rename = "HUF")]
    Forint,
}

impl Display for Price {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.value, self.currency)
    }
}

impl Deref for Price {
    type Target = Decimal;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
