use crate::ReadOnlyString;
use serde::Deserialize;

/// A location.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
pub struct Location {
    /// Postal code.
    #[serde(rename = "postalCode")]
    pub post_code: ReadOnlyString,

    /// Country code.
    pub country: ReadOnlyString,

    /// State or province.
    /// Some API calls dont provide this.
    #[serde(rename = "stateOrProvince")]
    pub state: Option<ReadOnlyString>,

    /// City.
    /// Some API calls dont provide this.
    #[serde(rename = "city")]
    pub city: Option<ReadOnlyString>,
}
