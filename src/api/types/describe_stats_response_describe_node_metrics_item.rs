pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DescribeStatsResponseDescribeNodeMetricsItem {
    /// The metric name.
    #[serde(default)]
    pub name: String,
    /// The node path this metric operates on.
    #[serde(default)]
    pub node_path: String,
    /// Query engines that support this metric.
    #[serde(default)]
    pub supported_engines: Vec<String>,
}

impl DescribeStatsResponseDescribeNodeMetricsItem {
    pub fn builder() -> DescribeStatsResponseDescribeNodeMetricsItemBuilder {
        <DescribeStatsResponseDescribeNodeMetricsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DescribeStatsResponseDescribeNodeMetricsItemBuilder {
    name: Option<String>,
    node_path: Option<String>,
    supported_engines: Option<Vec<String>>,
}

impl DescribeStatsResponseDescribeNodeMetricsItemBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn node_path(mut self, value: impl Into<String>) -> Self {
        self.node_path = Some(value.into());
        self
    }

    pub fn supported_engines(mut self, value: Vec<String>) -> Self {
        self.supported_engines = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DescribeStatsResponseDescribeNodeMetricsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](DescribeStatsResponseDescribeNodeMetricsItemBuilder::name)
    /// - [`node_path`](DescribeStatsResponseDescribeNodeMetricsItemBuilder::node_path)
    /// - [`supported_engines`](DescribeStatsResponseDescribeNodeMetricsItemBuilder::supported_engines)
    pub fn build(self) -> Result<DescribeStatsResponseDescribeNodeMetricsItem, BuildError> {
        Ok(DescribeStatsResponseDescribeNodeMetricsItem {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            node_path: self
                .node_path
                .ok_or_else(|| BuildError::missing_field("node_path"))?,
            supported_engines: self
                .supported_engines
                .ok_or_else(|| BuildError::missing_field("supported_engines"))?,
        })
    }
}
