pub use crate::prelude::*;

/// Supported payout method display details.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CancelPayoutsResponsePayoutMethodSupportedPayoutMethod {
    /// How the funds are delivered to the recipient.
    pub delivery_type: CancelPayoutsResponsePayoutMethodSupportedPayoutMethodDeliveryType,
    /// Supported payout method icon URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    /// Supported payout method display name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payer_name: Option<String>,
}

impl CancelPayoutsResponsePayoutMethodSupportedPayoutMethod {
    pub fn builder() -> CancelPayoutsResponsePayoutMethodSupportedPayoutMethodBuilder {
        <CancelPayoutsResponsePayoutMethodSupportedPayoutMethodBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CancelPayoutsResponsePayoutMethodSupportedPayoutMethodBuilder {
    delivery_type: Option<CancelPayoutsResponsePayoutMethodSupportedPayoutMethodDeliveryType>,
    icon_url: Option<String>,
    payer_name: Option<String>,
}

impl CancelPayoutsResponsePayoutMethodSupportedPayoutMethodBuilder {
    pub fn delivery_type(
        mut self,
        value: CancelPayoutsResponsePayoutMethodSupportedPayoutMethodDeliveryType,
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

    /// Consumes the builder and constructs a [`CancelPayoutsResponsePayoutMethodSupportedPayoutMethod`].
    /// This method will fail if any of the following fields are not set:
    /// - [`delivery_type`](CancelPayoutsResponsePayoutMethodSupportedPayoutMethodBuilder::delivery_type)
    pub fn build(
        self,
    ) -> Result<CancelPayoutsResponsePayoutMethodSupportedPayoutMethod, BuildError> {
        Ok(CancelPayoutsResponsePayoutMethodSupportedPayoutMethod {
            delivery_type: self
                .delivery_type
                .ok_or_else(|| BuildError::missing_field("delivery_type"))?,
            icon_url: self.icon_url,
            payer_name: self.payer_name,
        })
    }
}
