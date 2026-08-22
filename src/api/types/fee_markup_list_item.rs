pub use crate::prelude::*;

/// A fee markup configuration that defines additional charges applied to transactions for a platform's connected accounts.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FeeMarkupListItem {
    /// The datetime the fee markup was created.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// The category of fee this markup applies to.
    pub fee_type: FeeMarkupTypes,
    /// A flat fee charged per transaction, in USD. Ranges from 0 to 50. Null if no fixed fee is configured.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub fixed_fee_usd: Option<f64>,
    /// The unique identifier for the fee markup.
    #[serde(default)]
    pub id: String,
    /// Internal notes about this fee markup, visible only to administrators. Null if no notes have been added.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    /// A percentage-based fee charged per transaction. Ranges from 0 to 25. Null if no percentage fee is configured.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub percentage_fee: Option<f64>,
    /// The datetime the fee markup was last updated.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
}

impl FeeMarkupListItem {
    pub fn builder() -> FeeMarkupListItemBuilder {
        <FeeMarkupListItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FeeMarkupListItemBuilder {
    created_at: Option<DateTime<FixedOffset>>,
    fee_type: Option<FeeMarkupTypes>,
    fixed_fee_usd: Option<f64>,
    id: Option<String>,
    notes: Option<String>,
    percentage_fee: Option<f64>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl FeeMarkupListItemBuilder {
    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
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

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
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

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FeeMarkupListItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](FeeMarkupListItemBuilder::created_at)
    /// - [`fee_type`](FeeMarkupListItemBuilder::fee_type)
    /// - [`id`](FeeMarkupListItemBuilder::id)
    /// - [`updated_at`](FeeMarkupListItemBuilder::updated_at)
    pub fn build(self) -> Result<FeeMarkupListItem, BuildError> {
        Ok(FeeMarkupListItem {
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            fee_type: self
                .fee_type
                .ok_or_else(|| BuildError::missing_field("fee_type"))?,
            fixed_fee_usd: self.fixed_fee_usd,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            notes: self.notes,
            percentage_fee: self.percentage_fee,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
