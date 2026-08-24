pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ExtendMembershipsRequest {
    /// Number of free days to add (1-1095).
    #[serde(default)]
    pub days: i64,
}

impl ExtendMembershipsRequest {
    pub fn builder() -> ExtendMembershipsRequestBuilder {
        <ExtendMembershipsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExtendMembershipsRequestBuilder {
    days: Option<i64>,
}

impl ExtendMembershipsRequestBuilder {
    pub fn days(mut self, value: i64) -> Self {
        self.days = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ExtendMembershipsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`days`](ExtendMembershipsRequestBuilder::days)
    pub fn build(self) -> Result<ExtendMembershipsRequest, BuildError> {
        Ok(ExtendMembershipsRequest {
            days: self.days.ok_or_else(|| BuildError::missing_field("days"))?,
        })
    }
}
