pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "typename")]
#[non_exhaustive]
pub enum DescribeStatsResponse {
    #[non_exhaustive]
    DescribeRoot {
        #[serde(default)]
        nodes: Vec<String>,
        #[serde(default)]
        views: Vec<String>,
        #[serde(default)]
        metrics: Vec<DescribeStatsResponseDescribeRootMetricsItem>,
        #[serde(skip_serializing_if = "Option::is_none")]
        debug: Option<DescribeStatsResponseDescribeRootDebug>,
    },

    #[non_exhaustive]
    DescribeNode {
        #[serde(default)]
        node: String,
        #[serde(default)]
        engine: String,
        #[serde(default)]
        columns: Vec<String>,
        #[serde(default)]
        sortable_columns: Vec<String>,
        #[serde(default)]
        associations: Vec<DescribeStatsResponseDescribeNodeAssociationsItem>,
        #[serde(default)]
        metrics: Vec<DescribeStatsResponseDescribeNodeMetricsItem>,
        #[serde(skip_serializing_if = "Option::is_none")]
        sample: Option<Vec<HashMap<String, serde_json::Value>>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        debug: Option<DescribeStatsResponseDescribeNodeDebug>,
    },

    #[non_exhaustive]
    DescribeMetric {
        #[serde(default)]
        metric: String,
        #[serde(default)]
        node: String,
        #[serde(default)]
        engine: String,
        #[serde(default)]
        timestamp_column: String,
        #[serde(default)]
        supported_engines: Vec<String>,
        #[serde(default)]
        filterable_columns: Vec<String>,
        #[serde(default)]
        breakdownable_columns: Vec<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        sql: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        debug: Option<DescribeStatsResponseDescribeMetricDebug>,
    },

    #[non_exhaustive]
    DescribeView {
        #[serde(default)]
        view: String,
        #[serde(default)]
        resource: String,
        #[serde(default)]
        model: String,
        #[serde(default)]
        engine: String,
        #[serde(default)]
        columns: Vec<String>,
        #[serde(default)]
        sortable_columns: Vec<String>,
        #[serde(default)]
        associations: Vec<DescribeStatsResponseDescribeViewAssociationsItem>,
        #[serde(default)]
        metrics: Vec<DescribeStatsResponseDescribeViewMetricsItem>,
        #[serde(skip_serializing_if = "Option::is_none")]
        sample: Option<Vec<HashMap<String, serde_json::Value>>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        debug: Option<DescribeStatsResponseDescribeViewDebug>,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl DescribeStatsResponse {
    pub fn describe_root(
        nodes: Vec<String>,
        views: Vec<String>,
        metrics: Vec<DescribeStatsResponseDescribeRootMetricsItem>,
    ) -> Self {
        Self::DescribeRoot {
            nodes,
            views,
            metrics,
            debug: None,
        }
    }

    pub fn describe_node(
        node: String,
        engine: String,
        columns: Vec<String>,
        sortable_columns: Vec<String>,
        associations: Vec<DescribeStatsResponseDescribeNodeAssociationsItem>,
        metrics: Vec<DescribeStatsResponseDescribeNodeMetricsItem>,
    ) -> Self {
        Self::DescribeNode {
            node,
            engine,
            columns,
            sortable_columns,
            associations,
            metrics,
            sample: None,
            debug: None,
        }
    }

    pub fn describe_metric(
        metric: String,
        node: String,
        engine: String,
        timestamp_column: String,
        supported_engines: Vec<String>,
        filterable_columns: Vec<String>,
        breakdownable_columns: Vec<String>,
    ) -> Self {
        Self::DescribeMetric {
            metric,
            node,
            engine,
            timestamp_column,
            supported_engines,
            filterable_columns,
            breakdownable_columns,
            sql: None,
            debug: None,
        }
    }

    pub fn describe_view(
        view: String,
        resource: String,
        model: String,
        engine: String,
        columns: Vec<String>,
        sortable_columns: Vec<String>,
        associations: Vec<DescribeStatsResponseDescribeViewAssociationsItem>,
        metrics: Vec<DescribeStatsResponseDescribeViewMetricsItem>,
    ) -> Self {
        Self::DescribeView {
            view,
            resource,
            model,
            engine,
            columns,
            sortable_columns,
            associations,
            metrics,
            sample: None,
            debug: None,
        }
    }

    pub fn describe_root_with_debug(
        nodes: Vec<String>,
        views: Vec<String>,
        metrics: Vec<DescribeStatsResponseDescribeRootMetricsItem>,
        debug: DescribeStatsResponseDescribeRootDebug,
    ) -> Self {
        Self::DescribeRoot {
            nodes,
            views,
            metrics,
            debug: Some(debug),
        }
    }

    pub fn describe_node_with_sample(
        node: String,
        engine: String,
        columns: Vec<String>,
        sortable_columns: Vec<String>,
        associations: Vec<DescribeStatsResponseDescribeNodeAssociationsItem>,
        metrics: Vec<DescribeStatsResponseDescribeNodeMetricsItem>,
        sample: Vec<HashMap<String, serde_json::Value>>,
        debug: Option<DescribeStatsResponseDescribeNodeDebug>,
    ) -> Self {
        Self::DescribeNode {
            node,
            engine,
            columns,
            sortable_columns,
            associations,
            metrics,
            sample: Some(sample),
            debug,
        }
    }

    pub fn describe_node_with_debug(
        node: String,
        engine: String,
        columns: Vec<String>,
        sortable_columns: Vec<String>,
        associations: Vec<DescribeStatsResponseDescribeNodeAssociationsItem>,
        metrics: Vec<DescribeStatsResponseDescribeNodeMetricsItem>,
        sample: Option<Vec<HashMap<String, serde_json::Value>>>,
        debug: DescribeStatsResponseDescribeNodeDebug,
    ) -> Self {
        Self::DescribeNode {
            node,
            engine,
            columns,
            sortable_columns,
            associations,
            metrics,
            sample,
            debug: Some(debug),
        }
    }

    pub fn describe_metric_with_sql(
        metric: String,
        node: String,
        engine: String,
        timestamp_column: String,
        supported_engines: Vec<String>,
        filterable_columns: Vec<String>,
        breakdownable_columns: Vec<String>,
        sql: String,
        debug: Option<DescribeStatsResponseDescribeMetricDebug>,
    ) -> Self {
        Self::DescribeMetric {
            metric,
            node,
            engine,
            timestamp_column,
            supported_engines,
            filterable_columns,
            breakdownable_columns,
            sql: Some(sql),
            debug,
        }
    }

    pub fn describe_metric_with_debug(
        metric: String,
        node: String,
        engine: String,
        timestamp_column: String,
        supported_engines: Vec<String>,
        filterable_columns: Vec<String>,
        breakdownable_columns: Vec<String>,
        sql: Option<String>,
        debug: DescribeStatsResponseDescribeMetricDebug,
    ) -> Self {
        Self::DescribeMetric {
            metric,
            node,
            engine,
            timestamp_column,
            supported_engines,
            filterable_columns,
            breakdownable_columns,
            sql,
            debug: Some(debug),
        }
    }

    pub fn describe_view_with_sample(
        view: String,
        resource: String,
        model: String,
        engine: String,
        columns: Vec<String>,
        sortable_columns: Vec<String>,
        associations: Vec<DescribeStatsResponseDescribeViewAssociationsItem>,
        metrics: Vec<DescribeStatsResponseDescribeViewMetricsItem>,
        sample: Vec<HashMap<String, serde_json::Value>>,
        debug: Option<DescribeStatsResponseDescribeViewDebug>,
    ) -> Self {
        Self::DescribeView {
            view,
            resource,
            model,
            engine,
            columns,
            sortable_columns,
            associations,
            metrics,
            sample: Some(sample),
            debug,
        }
    }

    pub fn describe_view_with_debug(
        view: String,
        resource: String,
        model: String,
        engine: String,
        columns: Vec<String>,
        sortable_columns: Vec<String>,
        associations: Vec<DescribeStatsResponseDescribeViewAssociationsItem>,
        metrics: Vec<DescribeStatsResponseDescribeViewMetricsItem>,
        sample: Option<Vec<HashMap<String, serde_json::Value>>>,
        debug: DescribeStatsResponseDescribeViewDebug,
    ) -> Self {
        Self::DescribeView {
            view,
            resource,
            model,
            engine,
            columns,
            sortable_columns,
            associations,
            metrics,
            sample,
            debug: Some(debug),
        }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
