use rust_decimal::Decimal;
use serde::Deserialize;
use serde_with::{serde_as, DisplayFromStr};

/// A fixed-precision decimal number representation of a price value.
#[serde_as]
#[derive(Debug, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Price {
    #[serde_as(as = "DisplayFromStr")]
    value: Decimal,
    currency: Currency,
}

/// A currency type.
#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Currency {
    #[serde(rename = "EUR")]
    Euro,
    #[serde(rename = "USD")]
    UsDollar,
    #[serde(rename = "HUF")]
    Forint,
}
