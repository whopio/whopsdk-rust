pub use crate::prelude::*;

/// What this document proves, in the processor's own evidence vocabulary. `return_policy`, `cancellation_policy`, and `terms_of_service` are the seller's policy documents — uploading one overrides the account's copy for this dispute (`return_policy`, `cancellation_policy`, and `customer_communication` also override the matching fixed evidence slot). `shipping_policy` is the seller's shipping terms. `customer_communication` is correspondence with the buyer — a support thread or chat log. `product_image` is a photo of the product or service the buyer received. `physical_fulfillment` is proof a physical order shipped and arrived; `digital_fulfillment` is proof the buyer accessed a digital product. `customer_order_history` is the buyer's past orders with this seller; `prior_transactions` is their broader payment history across the platform, for a fraud defense. `customer_session` is checkout forensics — IP, device fingerprint, AVS/CVV, 3D Secure result. `subscription` is membership lifecycle evidence — renewals, cancellation, reminders sent.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DisputeEvidenceDocumentDocumentType {
    ReturnPolicy,
    ShippingPolicy,
    CancellationPolicy,
    TermsOfService,
    PhysicalFulfillment,
    CustomerOrderHistory,
    ProductImage,
    PriorTransactions,
    CustomerSession,
    DigitalFulfillment,
    Subscription,
    CustomerCommunication,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for DisputeEvidenceDocumentDocumentType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::ReturnPolicy => serializer.serialize_str("return_policy"),
            Self::ShippingPolicy => serializer.serialize_str("shipping_policy"),
            Self::CancellationPolicy => serializer.serialize_str("cancellation_policy"),
            Self::TermsOfService => serializer.serialize_str("terms_of_service"),
            Self::PhysicalFulfillment => serializer.serialize_str("physical_fulfillment"),
            Self::CustomerOrderHistory => serializer.serialize_str("customer_order_history"),
            Self::ProductImage => serializer.serialize_str("product_image"),
            Self::PriorTransactions => serializer.serialize_str("prior_transactions"),
            Self::CustomerSession => serializer.serialize_str("customer_session"),
            Self::DigitalFulfillment => serializer.serialize_str("digital_fulfillment"),
            Self::Subscription => serializer.serialize_str("subscription"),
            Self::CustomerCommunication => serializer.serialize_str("customer_communication"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for DisputeEvidenceDocumentDocumentType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "return_policy" => Ok(Self::ReturnPolicy),
            "shipping_policy" => Ok(Self::ShippingPolicy),
            "cancellation_policy" => Ok(Self::CancellationPolicy),
            "terms_of_service" => Ok(Self::TermsOfService),
            "physical_fulfillment" => Ok(Self::PhysicalFulfillment),
            "customer_order_history" => Ok(Self::CustomerOrderHistory),
            "product_image" => Ok(Self::ProductImage),
            "prior_transactions" => Ok(Self::PriorTransactions),
            "customer_session" => Ok(Self::CustomerSession),
            "digital_fulfillment" => Ok(Self::DigitalFulfillment),
            "subscription" => Ok(Self::Subscription),
            "customer_communication" => Ok(Self::CustomerCommunication),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for DisputeEvidenceDocumentDocumentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ReturnPolicy => write!(f, "return_policy"),
            Self::ShippingPolicy => write!(f, "shipping_policy"),
            Self::CancellationPolicy => write!(f, "cancellation_policy"),
            Self::TermsOfService => write!(f, "terms_of_service"),
            Self::PhysicalFulfillment => write!(f, "physical_fulfillment"),
            Self::CustomerOrderHistory => write!(f, "customer_order_history"),
            Self::ProductImage => write!(f, "product_image"),
            Self::PriorTransactions => write!(f, "prior_transactions"),
            Self::CustomerSession => write!(f, "customer_session"),
            Self::DigitalFulfillment => write!(f, "digital_fulfillment"),
            Self::Subscription => write!(f, "subscription"),
            Self::CustomerCommunication => write!(f, "customer_communication"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
