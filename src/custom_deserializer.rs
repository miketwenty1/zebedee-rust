use serde::{Deserialize, Deserializer};
use std::{fmt::Display, str::FromStr};

pub fn deserialize_from_string<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr + serde::Deserialize<'de>,
    <T as FromStr>::Err: Display,
{
    String::deserialize(deserializer)?
        .parse::<T>()
        .map_err(serde::de::Error::custom)
}
