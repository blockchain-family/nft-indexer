use serde::{Serialize, Deserialize};
use std::str::FromStr;


#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "t_address")]
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

impl FromStr for Address {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Address(s.to_string()))
    }
}
