use derive_more::Display;
use rust_decimal::{prelude::ToPrimitive, Decimal};
use serde::Deserialize;
use serde_with::{serde_as, DisplayFromStr};
use std::{fmt::Display, ops::Deref};

/// A fixed-precision decimal number representation of a price value.
#[serde_as]
#[derive(Debug, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Price {
    /// Value.
    #[serde_as(as = "DisplayFromStr")]
    pub value: Decimal,

    /// Currency.
    pub currency: Currency,
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

impl Price {
    /// Convert the price value to a [`f32`].
    ///
    /// # Panics
    /// This may panic if the value cannot be represented as a [`f32`].
    #[must_use]
    pub fn as_f32(&self) -> f32 {
        self.value.to_f32().expect("cannot convert to f32")
    }

    /// Convert the price value to a [`f64`].
    ///
    /// # Panics
    /// This may panic if the value cannot be represented as a [`f64`].
    #[must_use]
    pub fn as_f64(&self) -> f64 {
        self.value.to_f64().expect("cannot convert to f64")
    }
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
