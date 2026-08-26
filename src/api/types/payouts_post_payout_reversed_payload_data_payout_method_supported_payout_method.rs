pub use crate::prelude::*;

/// Supported payout method display details.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostPayoutReversedPayloadDataPayoutMethodSupportedPayoutMethod {
    /// How the funds are delivered to the recipient.
    pub delivery_type: PostPayoutReversedPayloadDataPayoutMethodSupportedPayoutMethodDeliveryType,
    /// Supported payout method icon URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    /// Supported payout method display name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payer_name: Option<String>,
}

impl PostPayoutReversedPayloadDataPayoutMethodSupportedPayoutMethod {
    pub fn builder() -> PostPayoutReversedPayloadDataPayoutMethodSupportedPayoutMethodBuilder {
        <PostPayoutReversedPayloadDataPayoutMethodSupportedPayoutMethodBuilder as Default>::default(
        )
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostPayoutReversedPayloadDataPayoutMethodSupportedPayoutMethodBuilder {
    delivery_type:
        Option<PostPayoutReversedPayloadDataPayoutMethodSupportedPayoutMethodDeliveryType>,
    icon_url: Option<String>,
    payer_name: Option<String>,
}

impl PostPayoutReversedPayloadDataPayoutMethodSupportedPayoutMethodBuilder {
    pub fn delivery_type(
        mut self,
        value: PostPayoutReversedPayloadDataPayoutMethodSupportedPayoutMethodDeliveryType,
    ) -> Self {
        self.delivery_type = Some(value);
        self
    }

    pub fn icon_url(mut self, value: impl Into<String>) -> Self {
        self.icon_url = Some(value.into());
        self
    }

    pub fn payer_name(mut self, value: impl Into<String>) -> Self {
        self.payer_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostPayoutReversedPayloadDataPayoutMethodSupportedPayoutMethod`].
    /// This method will fail if any of the following fields are not set:
    /// - [`delivery_type`](PostPayoutReversedPayloadDataPayoutMethodSupportedPayoutMethodBuilder::delivery_type)
    pub fn build(
        self,
    ) -> Result<PostPayoutReversedPayloadDataPayoutMethodSupportedPayoutMethod, BuildError> {
        Ok(
            PostPayoutReversedPayloadDataPayoutMethodSupportedPayoutMethod {
                delivery_type: self
                    .delivery_type
                    .ok_or_else(|| BuildError::missing_field("delivery_type"))?,
                icon_url: self.icon_url,
                payer_name: self.payer_name,
            },
        )
    }
}
