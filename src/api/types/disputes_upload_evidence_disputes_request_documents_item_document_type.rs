pub use crate::prelude::*;

/// What kind of evidence the document is.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UploadEvidenceDisputesRequestDocumentsItemDocumentType {
    ReturnPolicy,
    ShippingPolicy,
    PhysicalFulfillment,
    CustomerOrderHistory,
    ProductImage,
    PriorTransactions,
    CustomerSession,
    DigitalFulfillment,
    Subscription,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for UploadEvidenceDisputesRequestDocumentsItemDocumentType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::ReturnPolicy => serializer.serialize_str("return_policy"),
            Self::ShippingPolicy => serializer.serialize_str("shipping_policy"),
            Self::PhysicalFulfillment => serializer.serialize_str("physical_fulfillment"),
            Self::CustomerOrderHistory => serializer.serialize_str("customer_order_history"),
            Self::ProductImage => serializer.serialize_str("product_image"),
            Self::PriorTransactions => serializer.serialize_str("prior_transactions"),
            Self::CustomerSession => serializer.serialize_str("customer_session"),
            Self::DigitalFulfillment => serializer.serialize_str("digital_fulfillment"),
            Self::Subscription => serializer.serialize_str("subscription"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for UploadEvidenceDisputesRequestDocumentsItemDocumentType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "return_policy" => Ok(Self::ReturnPolicy),
            "shipping_policy" => Ok(Self::ShippingPolicy),
            "physical_fulfillment" => Ok(Self::PhysicalFulfillment),
            "customer_order_history" => Ok(Self::CustomerOrderHistory),
            "product_image" => Ok(Self::ProductImage),
            "prior_transactions" => Ok(Self::PriorTransactions),
            "customer_session" => Ok(Self::CustomerSession),
            "digital_fulfillment" => Ok(Self::DigitalFulfillment),
            "subscription" => Ok(Self::Subscription),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for UploadEvidenceDisputesRequestDocumentsItemDocumentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ReturnPolicy => write!(f, "return_policy"),
            Self::ShippingPolicy => write!(f, "shipping_policy"),
            Self::PhysicalFulfillment => write!(f, "physical_fulfillment"),
            Self::CustomerOrderHistory => write!(f, "customer_order_history"),
            Self::ProductImage => write!(f, "product_image"),
            Self::PriorTransactions => write!(f, "prior_transactions"),
            Self::CustomerSession => write!(f, "customer_session"),
            Self::DigitalFulfillment => write!(f, "digital_fulfillment"),
            Self::Subscription => write!(f, "subscription"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
