use crate::error::Result;
use reqwest::blocking::Response;
use serde::de::DeserializeOwned;

pub trait Jsonify {
    fn jsonify<T: DeserializeOwned>(self) -> Result<T>;
}

impl Jsonify for Response {
    fn jsonify<T: DeserializeOwned>(self) -> Result<T> {
        Ok(serde_json::from_reader(self)?)
    }
}
