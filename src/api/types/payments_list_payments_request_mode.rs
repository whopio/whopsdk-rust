pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListPaymentsRequestMode {
    AccountSales,
    UserSales,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ListPaymentsRequestMode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::AccountSales => serializer.serialize_str("account_sales"),
            Self::UserSales => serializer.serialize_str("user_sales"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ListPaymentsRequestMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "account_sales" => Ok(Self::AccountSales),
            "user_sales" => Ok(Self::UserSales),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ListPaymentsRequestMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AccountSales => write!(f, "account_sales"),
            Self::UserSales => write!(f, "user_sales"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
