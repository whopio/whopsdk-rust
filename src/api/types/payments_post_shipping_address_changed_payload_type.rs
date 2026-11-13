pub use crate::prelude::*;

/// The webhook event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostShippingAddressChangedPayloadType {
    #[serde(rename = "shipping_address.changed")]
    ShippingAddressChanged,
}
impl fmt::Display for PostShippingAddressChangedPayloadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::ShippingAddressChanged => "shipping_address.changed",
        };
        write!(f, "{}", s)
    }
}
