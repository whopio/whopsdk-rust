pub use crate::prelude::*;

/// Optional second method charged if the primary fails. Any pairing is allowed (two cards, card+balance, balance+card); omit it to run on a single method. Must differ from the primary.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct UpdatePreferencesRequestAdsPaymentMethodsBackup {
    /// The funding source ID: a Whop balance (`ldgr_`) for `platform_balance`, or a payment method (`payt_`) for `card`. Optional for `platform_balance` — defaults to the account's default Whop balance. Required for `card`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The funding source kind.
    pub r#type: UpdatePreferencesRequestAdsPaymentMethodsBackupType,
}

impl UpdatePreferencesRequestAdsPaymentMethodsBackup {
    pub fn builder() -> UpdatePreferencesRequestAdsPaymentMethodsBackupBuilder {
        <UpdatePreferencesRequestAdsPaymentMethodsBackupBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdatePreferencesRequestAdsPaymentMethodsBackupBuilder {
    id: Option<String>,
    r#type: Option<UpdatePreferencesRequestAdsPaymentMethodsBackupType>,
}

impl UpdatePreferencesRequestAdsPaymentMethodsBackupBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: UpdatePreferencesRequestAdsPaymentMethodsBackupType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdatePreferencesRequestAdsPaymentMethodsBackup`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](UpdatePreferencesRequestAdsPaymentMethodsBackupBuilder::r#type)
    pub fn build(self) -> Result<UpdatePreferencesRequestAdsPaymentMethodsBackup, BuildError> {
        Ok(UpdatePreferencesRequestAdsPaymentMethodsBackup {
            id: self.id,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
