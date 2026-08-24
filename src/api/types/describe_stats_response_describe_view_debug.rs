pub use crate::prelude::*;

/// Debug information.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DescribeStatsResponseDescribeViewDebug {
    /// Unique request identifier for debugging.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

impl DescribeStatsResponseDescribeViewDebug {
    pub fn builder() -> DescribeStatsResponseDescribeViewDebugBuilder {
        <DescribeStatsResponseDescribeViewDebugBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DescribeStatsResponseDescribeViewDebugBuilder {
    request_id: Option<String>,
}

impl DescribeStatsResponseDescribeViewDebugBuilder {
    pub fn request_id(mut self, value: impl Into<String>) -> Self {
        self.request_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DescribeStatsResponseDescribeViewDebug`].
    pub fn build(self) -> Result<DescribeStatsResponseDescribeViewDebug, BuildError> {
        Ok(DescribeStatsResponseDescribeViewDebug {
            request_id: self.request_id,
        })
    }
}
