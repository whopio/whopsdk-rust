pub use crate::prelude::*;

/// How the account pays for Whop Ads spend. `primary` is charged first; `backup` covers the charge when the primary fails.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct UpdatePreferencesRequestAdsPaymentMethods {
    /// Optional second method charged if the primary fails. Any pairing is allowed (two cards, card+balance, balance+card); omit it to run on a single method. Must differ from the primary.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup: Option<UpdatePreferencesRequestAdsPaymentMethodsBackup>,
    pub primary: UpdatePreferencesRequestAdsPaymentMethodsPrimary,
}

impl UpdatePreferencesRequestAdsPaymentMethods {
    pub fn builder() -> UpdatePreferencesRequestAdsPaymentMethodsBuilder {
        <UpdatePreferencesRequestAdsPaymentMethodsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdatePreferencesRequestAdsPaymentMethodsBuilder {
    backup: Option<UpdatePreferencesRequestAdsPaymentMethodsBackup>,
    primary: Option<UpdatePreferencesRequestAdsPaymentMethodsPrimary>,
}

impl UpdatePreferencesRequestAdsPaymentMethodsBuilder {
    pub fn backup(mut self, value: UpdatePreferencesRequestAdsPaymentMethodsBackup) -> Self {
        self.backup = Some(value);
        self
    }

    pub fn primary(mut self, value: UpdatePreferencesRequestAdsPaymentMethodsPrimary) -> Self {
        self.primary = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdatePreferencesRequestAdsPaymentMethods`].
    /// This method will fail if any of the following fields are not set:
    /// - [`primary`](UpdatePreferencesRequestAdsPaymentMethodsBuilder::primary)
    pub fn build(self) -> Result<UpdatePreferencesRequestAdsPaymentMethods, BuildError> {
        Ok(UpdatePreferencesRequestAdsPaymentMethods {
            backup: self.backup,
            primary: self
                .primary
                .ok_or_else(|| BuildError::missing_field("primary"))?,
        })
    }
}
