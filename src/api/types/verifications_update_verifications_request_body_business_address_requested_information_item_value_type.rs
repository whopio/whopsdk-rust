pub use crate::prelude::*;

/// Whether `value` is raw input or a vault token.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UpdateVerificationsRequestBodyBusinessAddressRequestedInformationItemValueType {
    Raw,
    VaultToken,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for UpdateVerificationsRequestBodyBusinessAddressRequestedInformationItemValueType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Raw => serializer.serialize_str("raw"),
            Self::VaultToken => serializer.serialize_str("vault_token"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de>
    for UpdateVerificationsRequestBodyBusinessAddressRequestedInformationItemValueType
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "raw" => Ok(Self::Raw),
            "vault_token" => Ok(Self::VaultToken),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display
    for UpdateVerificationsRequestBodyBusinessAddressRequestedInformationItemValueType
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Raw => write!(f, "raw"),
            Self::VaultToken => write!(f, "vault_token"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
