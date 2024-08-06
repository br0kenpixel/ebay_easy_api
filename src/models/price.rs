use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Price {
    value: Box<str>,
    currency: Box<str>,
}
