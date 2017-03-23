//! Stopgap module to support url::Url serialization with serde 0.9.

use std::error::Error;

use serde::de::Error as DeserializeError;
use serde::{Serializer, Deserializer, Deserialize};
use url::Url;

pub fn serialize<S>(url: &Url, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer
{
    serializer.serialize_str(&url.to_string())
}

pub fn deserialize<D>(deserializer: D) -> Result<Url, D::Error>
    where D: Deserializer
{
    let url = String::deserialize(deserializer)?;
    Url::parse(&url).map_err(|err| DeserializeError::custom(err.description()))
}
