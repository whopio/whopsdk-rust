pub use crate::prelude::*;

/// How the account pays for Whop Ads spend. `primary` is charged first; `backup` covers the charge when the primary fails. `null` until ads billing has been configured.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RetrievePreferencesResponseAdsPaymentMethods {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup: Option<RetrievePreferencesResponseAdsPaymentMethodsBackup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary: Option<RetrievePreferencesResponseAdsPaymentMethodsPrimary>,
}

impl RetrievePreferencesResponseAdsPaymentMethods {
    pub fn builder() -> RetrievePreferencesResponseAdsPaymentMethodsBuilder {
        <RetrievePreferencesResponseAdsPaymentMethodsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RetrievePreferencesResponseAdsPaymentMethodsBuilder {
    backup: Option<RetrievePreferencesResponseAdsPaymentMethodsBackup>,
    primary: Option<RetrievePreferencesResponseAdsPaymentMethodsPrimary>,
}

impl RetrievePreferencesResponseAdsPaymentMethodsBuilder {
    pub fn backup(mut self, value: RetrievePreferencesResponseAdsPaymentMethodsBackup) -> Self {
        self.backup = Some(value);
        self
    }

    pub fn primary(mut self, value: RetrievePreferencesResponseAdsPaymentMethodsPrimary) -> Self {
        self.primary = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RetrievePreferencesResponseAdsPaymentMethods`].
    pub fn build(self) -> Result<RetrievePreferencesResponseAdsPaymentMethods, BuildError> {
        Ok(RetrievePreferencesResponseAdsPaymentMethods {
            backup: self.backup,
            primary: self.primary,
        })
    }
}
