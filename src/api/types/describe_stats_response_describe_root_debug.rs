pub use crate::prelude::*;

/// Debug information.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DescribeStatsResponseDescribeRootDebug {
    /// Unique request identifier for debugging.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

impl DescribeStatsResponseDescribeRootDebug {
    pub fn builder() -> DescribeStatsResponseDescribeRootDebugBuilder {
        <DescribeStatsResponseDescribeRootDebugBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DescribeStatsResponseDescribeRootDebugBuilder {
    request_id: Option<String>,
}

impl DescribeStatsResponseDescribeRootDebugBuilder {
    pub fn request_id(mut self, value: impl Into<String>) -> Self {
        self.request_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DescribeStatsResponseDescribeRootDebug`].
    pub fn build(self) -> Result<DescribeStatsResponseDescribeRootDebug, BuildError> {
        Ok(DescribeStatsResponseDescribeRootDebug {
            request_id: self.request_id,
        })
    }
}
