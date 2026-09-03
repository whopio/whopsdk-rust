pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PaymentFee {
    /// The fee in the currency it was collected in.
    #[serde(default)]
    pub amount: Money,
    /// When the fee was collected, as an ISO 8601 timestamp, or null when it has not been.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collected_at: Option<String>,
    /// A longer explanation of the fee, when there is one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The name the dashboard shows for this fee.
    #[serde(default)]
    pub label: String,
    /// The specific fee this line is, such as `payment_processing_percentage_fee` or `revshare_percentage_fee`.
    pub origin: PaymentFeeOrigin,
    /// The fee converted to the payment's settlement currency, so lines can be totalled against the payment.
    #[serde(default)]
    pub settlement_amount: Money,
    /// The family the fee belongs to: `whop_fee`, `processing_fee`, `affiliate_program_fee`, or `other_fee`.
    pub r#type: PaymentFeeType,
}

impl PaymentFee {
    pub fn builder() -> PaymentFeeBuilder {
        <PaymentFeeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentFeeBuilder {
    amount: Option<Money>,
    collected_at: Option<String>,
    description: Option<String>,
    label: Option<String>,
    origin: Option<PaymentFeeOrigin>,
    settlement_amount: Option<Money>,
    r#type: Option<PaymentFeeType>,
}

impl PaymentFeeBuilder {
    pub fn amount(mut self, value: Money) -> Self {
        self.amount = Some(value);
        self
    }

    pub fn collected_at(mut self, value: impl Into<String>) -> Self {
        self.collected_at = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn origin(mut self, value: PaymentFeeOrigin) -> Self {
        self.origin = Some(value);
        self
    }

    pub fn settlement_amount(mut self, value: Money) -> Self {
        self.settlement_amount = Some(value);
        self
    }

    pub fn r#type(mut self, value: PaymentFeeType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaymentFee`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount`](PaymentFeeBuilder::amount)
    /// - [`label`](PaymentFeeBuilder::label)
    /// - [`origin`](PaymentFeeBuilder::origin)
    /// - [`settlement_amount`](PaymentFeeBuilder::settlement_amount)
    /// - [`r#type`](PaymentFeeBuilder::r#type)
    pub fn build(self) -> Result<PaymentFee, BuildError> {
        Ok(PaymentFee {
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            collected_at: self.collected_at,
            description: self.description,
            label: self
                .label
                .ok_or_else(|| BuildError::missing_field("label"))?,
            origin: self
                .origin
                .ok_or_else(|| BuildError::missing_field("origin"))?,
            settlement_amount: self
                .settlement_amount
                .ok_or_else(|| BuildError::missing_field("settlement_amount"))?,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
