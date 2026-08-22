pub use crate::prelude::*;

/// Estimated arrival times before an amount-specific quote is requested. Null when the method is not currently eligible.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListMethodsResponseDataItemEstimatedArrival {
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

impl ListMethodsResponseDataItemEstimatedArrival {
    pub fn builder() -> ListMethodsResponseDataItemEstimatedArrivalBuilder {
        <ListMethodsResponseDataItemEstimatedArrivalBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListMethodsResponseDataItemEstimatedArrivalBuilder {
    instant: Option<DateTime<FixedOffset>>,
    standard: Option<DateTime<FixedOffset>>,
}

impl ListMethodsResponseDataItemEstimatedArrivalBuilder {
    pub fn instant(mut self, value: DateTime<FixedOffset>) -> Self {
        self.instant = Some(value);
        self
    }

    pub fn standard(mut self, value: DateTime<FixedOffset>) -> Self {
        self.standard = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListMethodsResponseDataItemEstimatedArrival`].
    pub fn build(self) -> Result<ListMethodsResponseDataItemEstimatedArrival, BuildError> {
        Ok(ListMethodsResponseDataItemEstimatedArrival {
            instant: self.instant,
            standard: self.standard,
        })
    }
}
