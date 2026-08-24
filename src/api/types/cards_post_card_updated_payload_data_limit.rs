pub use crate::prelude::*;

/// The spending limit configuration.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostCardUpdatedPayloadDataLimit {
    /// The limit amount in dollars.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub amount: f64,
    /// The window the limit amount applies to. `per_transaction` caps each individual authorization and is what a limit set with `transaction_limit` reports.
    pub frequency: PostCardUpdatedPayloadDataLimitFrequency,
}

impl PostCardUpdatedPayloadDataLimit {
    pub fn builder() -> PostCardUpdatedPayloadDataLimitBuilder {
        <PostCardUpdatedPayloadDataLimitBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostCardUpdatedPayloadDataLimitBuilder {
    amount: Option<f64>,
    frequency: Option<PostCardUpdatedPayloadDataLimitFrequency>,
}

impl PostCardUpdatedPayloadDataLimitBuilder {
    pub fn amount(mut self, value: f64) -> Self {
        self.amount = Some(value);
        self
    }

    pub fn frequency(mut self, value: PostCardUpdatedPayloadDataLimitFrequency) -> Self {
        self.frequency = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostCardUpdatedPayloadDataLimit`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount`](PostCardUpdatedPayloadDataLimitBuilder::amount)
    /// - [`frequency`](PostCardUpdatedPayloadDataLimitBuilder::frequency)
    pub fn build(self) -> Result<PostCardUpdatedPayloadDataLimit, BuildError> {
        Ok(PostCardUpdatedPayloadDataLimit {
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            frequency: self
                .frequency
                .ok_or_else(|| BuildError::missing_field("frequency"))?,
        })
    }
}
