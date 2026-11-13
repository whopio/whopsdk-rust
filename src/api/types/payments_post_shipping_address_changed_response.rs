pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum PostShippingAddressChangedResponse {
    ShippingAddressChangedWebhookReply(ShippingAddressChangedWebhookReply),

    PostShippingAddressChangedResponseDecline(PostShippingAddressChangedResponseDecline),
}

impl PostShippingAddressChangedResponse {
    pub fn is_shipping_address_changed_webhook_reply(&self) -> bool {
        matches!(self, Self::ShippingAddressChangedWebhookReply(_))
    }

    pub fn is_post_shipping_address_changed_response_decline(&self) -> bool {
        matches!(self, Self::PostShippingAddressChangedResponseDecline(_))
    }

    pub fn as_shipping_address_changed_webhook_reply(
        &self,
    ) -> Option<&ShippingAddressChangedWebhookReply> {
        match self {
            Self::ShippingAddressChangedWebhookReply(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_shipping_address_changed_webhook_reply(
        self,
    ) -> Option<ShippingAddressChangedWebhookReply> {
        match self {
            Self::ShippingAddressChangedWebhookReply(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_post_shipping_address_changed_response_decline(
        &self,
    ) -> Option<&PostShippingAddressChangedResponseDecline> {
        match self {
            Self::PostShippingAddressChangedResponseDecline(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_post_shipping_address_changed_response_decline(
        self,
    ) -> Option<PostShippingAddressChangedResponseDecline> {
        match self {
            Self::PostShippingAddressChangedResponseDecline(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for PostShippingAddressChangedResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ShippingAddressChangedWebhookReply(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::PostShippingAddressChangedResponseDecline(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
