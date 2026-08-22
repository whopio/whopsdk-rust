pub use crate::prelude::*;

/// Debug information.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DescribeStatsResponseDescribeMetricDebug {
    /// Unique request identifier for debugging.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

impl DescribeStatsResponseDescribeMetricDebug {
    pub fn builder() -> DescribeStatsResponseDescribeMetricDebugBuilder {
        <DescribeStatsResponseDescribeMetricDebugBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DescribeStatsResponseDescribeMetricDebugBuilder {
    request_id: Option<String>,
}

impl DescribeStatsResponseDescribeMetricDebugBuilder {
    pub fn request_id(mut self, value: impl Into<String>) -> Self {
        self.request_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DescribeStatsResponseDescribeMetricDebug`].
    pub fn build(self) -> Result<DescribeStatsResponseDescribeMetricDebug, BuildError> {
        Ok(DescribeStatsResponseDescribeMetricDebug {
            request_id: self.request_id,
        })
    }
}
