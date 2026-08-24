pub use crate::prelude::*;

/// The supported payout method this saved method was created from.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostPayoutMethodCreatedPayloadDataSupportedPayoutMethod {
    /// ISO 3166-1 alpha-3 country the destination pays out to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    /// How funds are delivered.
    pub delivery_type: PostPayoutMethodCreatedPayloadDataSupportedPayoutMethodDeliveryType,
    /// Supported payout method icon URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    /// Supported payout method display name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    pub supports_instant_delivery: bool,
    /// Whether the payer can link this method by signing in to their bank instead of typing account details.
    #[serde(default)]
    pub supports_plaid: bool,
    #[serde(default)]
    pub supports_standard_delivery: bool,
}

impl PostPayoutMethodCreatedPayloadDataSupportedPayoutMethod {
    pub fn builder() -> PostPayoutMethodCreatedPayloadDataSupportedPayoutMethodBuilder {
        <PostPayoutMethodCreatedPayloadDataSupportedPayoutMethodBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostPayoutMethodCreatedPayloadDataSupportedPayoutMethodBuilder {
    country_code: Option<String>,
    delivery_type: Option<PostPayoutMethodCreatedPayloadDataSupportedPayoutMethodDeliveryType>,
    icon_url: Option<String>,
    name: Option<String>,
    supports_instant_delivery: Option<bool>,
    supports_plaid: Option<bool>,
    supports_standard_delivery: Option<bool>,
}

impl PostPayoutMethodCreatedPayloadDataSupportedPayoutMethodBuilder {
    pub fn country_code(mut self, value: impl Into<String>) -> Self {
        self.country_code = Some(value.into());
        self
    }

    pub fn delivery_type(
        mut self,
        value: PostPayoutMethodCreatedPayloadDataSupportedPayoutMethodDeliveryType,
    ) -> Self {
        self.delivery_type = Some(value);
        self
    }

    pub fn icon_url(mut self, value: impl Into<String>) -> Self {
        self.icon_url = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn supports_instant_delivery(mut self, value: bool) -> Self {
        self.supports_instant_delivery = Some(value);
        self
    }

    pub fn supports_plaid(mut self, value: bool) -> Self {
        self.supports_plaid = Some(value);
        self
    }

    pub fn supports_standard_delivery(mut self, value: bool) -> Self {
        self.supports_standard_delivery = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostPayoutMethodCreatedPayloadDataSupportedPayoutMethod`].
    /// This method will fail if any of the following fields are not set:
    /// - [`delivery_type`](PostPayoutMethodCreatedPayloadDataSupportedPayoutMethodBuilder::delivery_type)
    /// - [`supports_instant_delivery`](PostPayoutMethodCreatedPayloadDataSupportedPayoutMethodBuilder::supports_instant_delivery)
    /// - [`supports_plaid`](PostPayoutMethodCreatedPayloadDataSupportedPayoutMethodBuilder::supports_plaid)
    /// - [`supports_standard_delivery`](PostPayoutMethodCreatedPayloadDataSupportedPayoutMethodBuilder::supports_standard_delivery)
    pub fn build(
        self,
    ) -> Result<PostPayoutMethodCreatedPayloadDataSupportedPayoutMethod, BuildError> {
        Ok(PostPayoutMethodCreatedPayloadDataSupportedPayoutMethod {
            country_code: self.country_code,
            delivery_type: self
                .delivery_type
                .ok_or_else(|| BuildError::missing_field("delivery_type"))?,
            icon_url: self.icon_url,
            name: self.name,
            supports_instant_delivery: self
                .supports_instant_delivery
                .ok_or_else(|| BuildError::missing_field("supports_instant_delivery"))?,
            supports_plaid: self
                .supports_plaid
                .ok_or_else(|| BuildError::missing_field("supports_plaid"))?,
            supports_standard_delivery: self
                .supports_standard_delivery
                .ok_or_else(|| BuildError::missing_field("supports_standard_delivery"))?,
        })
    }
}
