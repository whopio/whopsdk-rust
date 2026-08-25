pub use crate::prelude::*;

/// What to collect. `custom_password` — the plan is password-protected: send the buyer's answer through update, and the entry disappears once it is right; confirm refuses while it stands. `email` — the buyer's email address; it identifies who the purchase is for, and confirm has no other way to resolve them. `terms` — explicit acceptance, sent as the `tos_accepted` attestation on confirm; show the seller's documents from `account.terms`. `custom_fields` — the seller's own questions, published in `fields`; answer them through `custom_field_responses`. `shipping_address` — a postal address for physical goods, set through `shipping_address`. `phone_number` — the seller collects buyer phone numbers: set one through update and it is recorded against the order; a missing number never refuses the confirm. The list is closed; new entries are added deliberately, so an unrecognized type is safe to skip.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CheckoutSessionRequirementType {
    CustomPassword,
    Email,
    Terms,
    CustomFields,
    ShippingAddress,
    PhoneNumber,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CheckoutSessionRequirementType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::CustomPassword => serializer.serialize_str("custom_password"),
            Self::Email => serializer.serialize_str("email"),
            Self::Terms => serializer.serialize_str("terms"),
            Self::CustomFields => serializer.serialize_str("custom_fields"),
            Self::ShippingAddress => serializer.serialize_str("shipping_address"),
            Self::PhoneNumber => serializer.serialize_str("phone_number"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CheckoutSessionRequirementType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "custom_password" => Ok(Self::CustomPassword),
            "email" => Ok(Self::Email),
            "terms" => Ok(Self::Terms),
            "custom_fields" => Ok(Self::CustomFields),
            "shipping_address" => Ok(Self::ShippingAddress),
            "phone_number" => Ok(Self::PhoneNumber),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CheckoutSessionRequirementType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CustomPassword => write!(f, "custom_password"),
            Self::Email => write!(f, "email"),
            Self::Terms => write!(f, "terms"),
            Self::CustomFields => write!(f, "custom_fields"),
            Self::ShippingAddress => write!(f, "shipping_address"),
            Self::PhoneNumber => write!(f, "phone_number"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
