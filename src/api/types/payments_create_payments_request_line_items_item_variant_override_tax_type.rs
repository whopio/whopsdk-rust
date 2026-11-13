pub use crate::prelude::*;

/// Whether the variant's price includes tax: `inclusive` (tax is included in the price), `exclusive` (tax is added on top), or `unspecified`. Omit it or pass `null` to use the account's tax setting. On an account enrolled in the Whop tax service, `inclusive` or `exclusive` replaces the account's setting and `unspecified` keeps it. Otherwise it applies only where the account is the merchant of record: `inclusive` or `exclusive` replaces the account's setting when the account collects tax, and `unspecified` means the variant collects no tax. Where Whop is the merchant of record, the account's setting applies. Variants that differ only in this value are different variants.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreatePaymentsRequestLineItemsItemVariantOverrideTaxType {
    Exclusive,
    Inclusive,
    Unspecified,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreatePaymentsRequestLineItemsItemVariantOverrideTaxType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Exclusive => serializer.serialize_str("exclusive"),
            Self::Inclusive => serializer.serialize_str("inclusive"),
            Self::Unspecified => serializer.serialize_str("unspecified"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreatePaymentsRequestLineItemsItemVariantOverrideTaxType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "exclusive" => Ok(Self::Exclusive),
            "inclusive" => Ok(Self::Inclusive),
            "unspecified" => Ok(Self::Unspecified),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreatePaymentsRequestLineItemsItemVariantOverrideTaxType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Exclusive => write!(f, "exclusive"),
            Self::Inclusive => write!(f, "inclusive"),
            Self::Unspecified => write!(f, "unspecified"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
