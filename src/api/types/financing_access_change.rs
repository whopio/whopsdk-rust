pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct FinancingAccessChange {
    /// The merchant account whose financing access changed, prefixed biz_.
    #[serde(default)]
    pub account_id: String,
    /// When the payment methods were disabled, as an ISO 8601 timestamp.
    #[serde(default)]
    pub disabled_at: String,
    #[serde(default)]
    pub disabled_payment_methods: Vec<FinancingAccessChangeDisabledPaymentMethodsItem>,
    /// The type of this object.
    pub object: FinancingAccessChangeObject,
    /// The BNPL provider whose dispute rate triggered enforcement. Other payment methods may also lose access.
    pub provider: FinancingAccessChangeProvider,
    /// Why financing access was removed.
    pub reason: FinancingAccessChangeReason,
}

impl FinancingAccessChange {
    pub fn builder() -> FinancingAccessChangeBuilder {
        <FinancingAccessChangeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FinancingAccessChangeBuilder {
    account_id: Option<String>,
    disabled_at: Option<String>,
    disabled_payment_methods: Option<Vec<FinancingAccessChangeDisabledPaymentMethodsItem>>,
    object: Option<FinancingAccessChangeObject>,
    provider: Option<FinancingAccessChangeProvider>,
    reason: Option<FinancingAccessChangeReason>,
}

impl FinancingAccessChangeBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn disabled_at(mut self, value: impl Into<String>) -> Self {
        self.disabled_at = Some(value.into());
        self
    }

    pub fn disabled_payment_methods(
        mut self,
        value: Vec<FinancingAccessChangeDisabledPaymentMethodsItem>,
    ) -> Self {
        self.disabled_payment_methods = Some(value);
        self
    }

    pub fn object(mut self, value: FinancingAccessChangeObject) -> Self {
        self.object = Some(value);
        self
    }

    pub fn provider(mut self, value: FinancingAccessChangeProvider) -> Self {
        self.provider = Some(value);
        self
    }

    pub fn reason(mut self, value: FinancingAccessChangeReason) -> Self {
        self.reason = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FinancingAccessChange`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_id`](FinancingAccessChangeBuilder::account_id)
    /// - [`disabled_at`](FinancingAccessChangeBuilder::disabled_at)
    /// - [`disabled_payment_methods`](FinancingAccessChangeBuilder::disabled_payment_methods)
    /// - [`object`](FinancingAccessChangeBuilder::object)
    /// - [`provider`](FinancingAccessChangeBuilder::provider)
    /// - [`reason`](FinancingAccessChangeBuilder::reason)
    pub fn build(self) -> Result<FinancingAccessChange, BuildError> {
        Ok(FinancingAccessChange {
            account_id: self
                .account_id
                .ok_or_else(|| BuildError::missing_field("account_id"))?,
            disabled_at: self
                .disabled_at
                .ok_or_else(|| BuildError::missing_field("disabled_at"))?,
            disabled_payment_methods: self
                .disabled_payment_methods
                .ok_or_else(|| BuildError::missing_field("disabled_payment_methods"))?,
            object: self
                .object
                .ok_or_else(|| BuildError::missing_field("object"))?,
            provider: self
                .provider
                .ok_or_else(|| BuildError::missing_field("provider"))?,
            reason: self
                .reason
                .ok_or_else(|| BuildError::missing_field("reason"))?,
        })
    }
}
