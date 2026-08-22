pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RetrievePreferencesResponseAdsPaymentMethodsBackup {
    /// Card brand, present for `card` entries.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_brand: Option<String>,
    /// Expiration month, present for `card` entries.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp_month: Option<i64>,
    /// Expiration year, present for `card` entries.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp_year: Option<i64>,
    /// Balance owner icon URL, present for `platform_balance` entries.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    /// The funding source ID: a Whop balance (`ldgr_`) for `platform_balance`, or a payment method (`payt_`) for `card`.
    #[serde(default)]
    pub id: String,
    /// Last four digits, present for `card` entries.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last4: Option<String>,
    /// Balance name, present for account `platform_balance` entries (null for a personal balance).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The funding source kind: a Whop balance or a saved card.
    pub r#type: RetrievePreferencesResponseAdsPaymentMethodsBackupType,
}

impl RetrievePreferencesResponseAdsPaymentMethodsBackup {
    pub fn builder() -> RetrievePreferencesResponseAdsPaymentMethodsBackupBuilder {
        <RetrievePreferencesResponseAdsPaymentMethodsBackupBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RetrievePreferencesResponseAdsPaymentMethodsBackupBuilder {
    card_brand: Option<String>,
    exp_month: Option<i64>,
    exp_year: Option<i64>,
    icon_url: Option<String>,
    id: Option<String>,
    last4: Option<String>,
    title: Option<String>,
    r#type: Option<RetrievePreferencesResponseAdsPaymentMethodsBackupType>,
}

impl RetrievePreferencesResponseAdsPaymentMethodsBackupBuilder {
    pub fn card_brand(mut self, value: impl Into<String>) -> Self {
        self.card_brand = Some(value.into());
        self
    }

    pub fn exp_month(mut self, value: i64) -> Self {
        self.exp_month = Some(value);
        self
    }

    pub fn exp_year(mut self, value: i64) -> Self {
        self.exp_year = Some(value);
        self
    }

    pub fn icon_url(mut self, value: impl Into<String>) -> Self {
        self.icon_url = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn last4(mut self, value: impl Into<String>) -> Self {
        self.last4 = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: RetrievePreferencesResponseAdsPaymentMethodsBackupType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RetrievePreferencesResponseAdsPaymentMethodsBackup`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](RetrievePreferencesResponseAdsPaymentMethodsBackupBuilder::id)
    /// - [`r#type`](RetrievePreferencesResponseAdsPaymentMethodsBackupBuilder::r#type)
    pub fn build(self) -> Result<RetrievePreferencesResponseAdsPaymentMethodsBackup, BuildError> {
        Ok(RetrievePreferencesResponseAdsPaymentMethodsBackup {
            card_brand: self.card_brand,
            exp_month: self.exp_month,
            exp_year: self.exp_year,
            icon_url: self.icon_url,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            last4: self.last4,
            title: self.title,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
