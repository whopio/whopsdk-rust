pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct UpdatePreferencesRequestAdsPaymentMethodsPrimary {
    /// The funding source ID: a Whop balance (`ldgr_`) for `platform_balance`, or a payment method (`payt_`) for `card`. Optional for `platform_balance` — defaults to the account's default Whop balance. Required for `card`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The funding source kind.
    pub r#type: UpdatePreferencesRequestAdsPaymentMethodsPrimaryType,
}

impl UpdatePreferencesRequestAdsPaymentMethodsPrimary {
    pub fn builder() -> UpdatePreferencesRequestAdsPaymentMethodsPrimaryBuilder {
        <UpdatePreferencesRequestAdsPaymentMethodsPrimaryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdatePreferencesRequestAdsPaymentMethodsPrimaryBuilder {
    id: Option<String>,
    r#type: Option<UpdatePreferencesRequestAdsPaymentMethodsPrimaryType>,
}

impl UpdatePreferencesRequestAdsPaymentMethodsPrimaryBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: UpdatePreferencesRequestAdsPaymentMethodsPrimaryType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdatePreferencesRequestAdsPaymentMethodsPrimary`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](UpdatePreferencesRequestAdsPaymentMethodsPrimaryBuilder::r#type)
    pub fn build(self) -> Result<UpdatePreferencesRequestAdsPaymentMethodsPrimary, BuildError> {
        Ok(UpdatePreferencesRequestAdsPaymentMethodsPrimary {
            id: self.id,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
