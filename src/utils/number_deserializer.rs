#[allow(unused_imports)]
use std::str::FromStr;
use std::fmt::Display;
use serde::{Deserialize ,Deserializer};

pub fn deserialize_number_from_string<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr + serde::Deserialize<'de>,
    <T as FromStr>::Err: Display,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrInt<T> {
        String(String),
        Number(T),
    }

    match StringOrInt::<T>::deserialize(deserializer)? {
        StringOrInt::String(s) => {
            let res = s.parse::<T>().map_err(serde::de::Error::custom);
            match res {
                Ok(i) => Ok(Some(i)),
                Err(e) => e,
            }
        },
        StringOrInt::Number(i) => Ok(Some(i)),
    }
}

