use serde::{Serialize, Deserialize};
use std::str::FromStr;


#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[derive(sqlx::Type)]
pub struct Address(pub String);

impl From<String> for Address {
    fn from(str_address: String) -> Self {
        Address(str_address)
    }
}

impl From<&str> for Address {
    fn from(str_address: &str) -> Self {
        Address(str_address.to_string())
    }
}

impl<'a> Into<&'a str> for &'a Address {
    fn into(self) -> &'a str {
        self.0.as_str()
    }
}

impl Into<String> for Address {
    fn into(self) -> String {
        self.0
    }
}

impl<'a> Into<&'a String> for &'a Address {
    fn into(self) -> &'a String {
        &self.0
    }
}

impl FromStr for Address {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Address(s.to_string()))
    }
}
