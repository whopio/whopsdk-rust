pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListTradesRequestType {
    Close,
    Order,
    Deposit,
    Withdraw,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ListTradesRequestType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Close => serializer.serialize_str("close"),
            Self::Order => serializer.serialize_str("order"),
            Self::Deposit => serializer.serialize_str("deposit"),
            Self::Withdraw => serializer.serialize_str("withdraw"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ListTradesRequestType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "close" => Ok(Self::Close),
            "order" => Ok(Self::Order),
            "deposit" => Ok(Self::Deposit),
            "withdraw" => Ok(Self::Withdraw),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ListTradesRequestType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Close => write!(f, "close"),
            Self::Order => write!(f, "order"),
            Self::Deposit => write!(f, "deposit"),
            Self::Withdraw => write!(f, "withdraw"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
