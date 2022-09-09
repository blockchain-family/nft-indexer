use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, sqlx::Type)]
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
