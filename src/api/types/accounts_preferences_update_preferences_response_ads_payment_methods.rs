pub use crate::prelude::*;

/// How the account pays for Whop Ads spend. `primary` is charged first; `backup` covers the charge when the primary fails. `null` until ads billing has been configured.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdatePreferencesResponseAdsPaymentMethods {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup: Option<UpdatePreferencesResponseAdsPaymentMethodsBackup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary: Option<UpdatePreferencesResponseAdsPaymentMethodsPrimary>,
}

impl UpdatePreferencesResponseAdsPaymentMethods {
    pub fn builder() -> UpdatePreferencesResponseAdsPaymentMethodsBuilder {
        <UpdatePreferencesResponseAdsPaymentMethodsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdatePreferencesResponseAdsPaymentMethodsBuilder {
    backup: Option<UpdatePreferencesResponseAdsPaymentMethodsBackup>,
    primary: Option<UpdatePreferencesResponseAdsPaymentMethodsPrimary>,
}

impl UpdatePreferencesResponseAdsPaymentMethodsBuilder {
    pub fn backup(mut self, value: UpdatePreferencesResponseAdsPaymentMethodsBackup) -> Self {
        self.backup = Some(value);
        self
    }

    pub fn primary(mut self, value: UpdatePreferencesResponseAdsPaymentMethodsPrimary) -> Self {
        self.primary = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdatePreferencesResponseAdsPaymentMethods`].
    pub fn build(self) -> Result<UpdatePreferencesResponseAdsPaymentMethods, BuildError> {
        Ok(UpdatePreferencesResponseAdsPaymentMethods {
            backup: self.backup,
            primary: self.primary,
        })
    }
}
