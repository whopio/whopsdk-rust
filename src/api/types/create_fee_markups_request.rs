pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateFeeMarkupsRequest {
    /// The unique identifier of the company to create or update the fee markup for.
    #[serde(default)]
    pub account_id: String,
    /// The type of fee this markup applies to, such as processing or platform fees.
    pub fee_type: FeeMarkupTypes,
    /// The fixed fee amount in USD to charge per transaction. Must be between 0 and 50.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_fee_usd: Option<f64>,
    /// Custom key-value metadata to attach to this fee markup.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    /// Internal notes about this fee markup for record-keeping purposes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    /// The percentage fee to charge per transaction. Must be between 0 and 25.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentage_fee: Option<f64>,
}

impl CreateFeeMarkupsRequest {
    pub fn builder() -> CreateFeeMarkupsRequestBuilder {
        <CreateFeeMarkupsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateFeeMarkupsRequestBuilder {
    account_id: Option<String>,
    fee_type: Option<FeeMarkupTypes>,
    fixed_fee_usd: Option<f64>,
    metadata: Option<HashMap<String, serde_json::Value>>,
    notes: Option<String>,
    percentage_fee: Option<f64>,
}

impl CreateFeeMarkupsRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn fee_type(mut self, value: FeeMarkupTypes) -> Self {
        self.fee_type = Some(value);
        self
    }

    pub fn fixed_fee_usd(mut self, value: f64) -> Self {
        self.fixed_fee_usd = Some(value);
        self
    }

    pub fn metadata(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    pub fn percentage_fee(mut self, value: f64) -> Self {
        self.percentage_fee = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateFeeMarkupsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_id`](CreateFeeMarkupsRequestBuilder::account_id)
    /// - [`fee_type`](CreateFeeMarkupsRequestBuilder::fee_type)
    pub fn build(self) -> Result<CreateFeeMarkupsRequest, BuildError> {
        Ok(CreateFeeMarkupsRequest {
            account_id: self
                .account_id
                .ok_or_else(|| BuildError::missing_field("account_id"))?,
            fee_type: self
                .fee_type
                .ok_or_else(|| BuildError::missing_field("fee_type"))?,
            fixed_fee_usd: self.fixed_fee_usd,
            metadata: self.metadata,
            notes: self.notes,
            percentage_fee: self.percentage_fee,
        })
    }
}
