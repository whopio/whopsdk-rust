pub use crate::prelude::*;

/// Estimated arrival times before an amount-specific quote is requested. Null when the method is not currently eligible.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostPayoutMethodCreatedPayloadDataEstimatedArrival {
    /// Estimated instant-delivery arrival, or null when unavailable.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub instant: Option<DateTime<FixedOffset>>,
    /// Estimated standard-delivery arrival, or null when unavailable.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub standard: Option<DateTime<FixedOffset>>,
}

impl PostPayoutMethodCreatedPayloadDataEstimatedArrival {
    pub fn builder() -> PostPayoutMethodCreatedPayloadDataEstimatedArrivalBuilder {
        <PostPayoutMethodCreatedPayloadDataEstimatedArrivalBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostPayoutMethodCreatedPayloadDataEstimatedArrivalBuilder {
    instant: Option<DateTime<FixedOffset>>,
    standard: Option<DateTime<FixedOffset>>,
}

impl PostPayoutMethodCreatedPayloadDataEstimatedArrivalBuilder {
    pub fn instant(mut self, value: DateTime<FixedOffset>) -> Self {
        self.instant = Some(value);
        self
    }

    pub fn standard(mut self, value: DateTime<FixedOffset>) -> Self {
        self.standard = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostPayoutMethodCreatedPayloadDataEstimatedArrival`].
    pub fn build(self) -> Result<PostPayoutMethodCreatedPayloadDataEstimatedArrival, BuildError> {
        Ok(PostPayoutMethodCreatedPayloadDataEstimatedArrival {
            instant: self.instant,
            standard: self.standard,
        })
    }
}
