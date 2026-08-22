pub use crate::prelude::*;

/// Debug information.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DescribeStatsResponseDescribeNodeDebug {
    /// Unique request identifier for debugging.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

impl DescribeStatsResponseDescribeNodeDebug {
    pub fn builder() -> DescribeStatsResponseDescribeNodeDebugBuilder {
        <DescribeStatsResponseDescribeNodeDebugBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DescribeStatsResponseDescribeNodeDebugBuilder {
    request_id: Option<String>,
}

impl DescribeStatsResponseDescribeNodeDebugBuilder {
    pub fn request_id(mut self, value: impl Into<String>) -> Self {
        self.request_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DescribeStatsResponseDescribeNodeDebug`].
    pub fn build(self) -> Result<DescribeStatsResponseDescribeNodeDebug, BuildError> {
        Ok(DescribeStatsResponseDescribeNodeDebug {
            request_id: self.request_id,
        })
    }
}
