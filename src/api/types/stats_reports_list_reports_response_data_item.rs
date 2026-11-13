pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ListReportsResponseDataItem {
    /// The values `breakdown_by` accepts.
    #[serde(default)]
    pub breakdowns: Vec<String>,
    /// Every property and metric the report can return.
    #[serde(default)]
    pub columns: Vec<ListReportsResponseDataItemColumnsItem>,
    /// What the report measures.
    #[serde(default)]
    pub description: String,
    /// The report's key, which is also the last segment of its path.
    #[serde(default)]
    pub key: String,
    /// The path that retrieves the report.
    #[serde(default)]
    pub path: String,
    /// The timestamp a range window filters on, such as `paid_at`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_field: Option<String>,
    /// `range` reports measure activity in a period; `as_of` reports measure state at a moment.
    pub window_kind: ListReportsResponseDataItemWindowKind,
}

impl ListReportsResponseDataItem {
    pub fn builder() -> ListReportsResponseDataItemBuilder {
        <ListReportsResponseDataItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListReportsResponseDataItemBuilder {
    breakdowns: Option<Vec<String>>,
    columns: Option<Vec<ListReportsResponseDataItemColumnsItem>>,
    description: Option<String>,
    key: Option<String>,
    path: Option<String>,
    window_field: Option<String>,
    window_kind: Option<ListReportsResponseDataItemWindowKind>,
}

impl ListReportsResponseDataItemBuilder {
    pub fn breakdowns(mut self, value: Vec<String>) -> Self {
        self.breakdowns = Some(value);
        self
    }

    pub fn columns(mut self, value: Vec<ListReportsResponseDataItemColumnsItem>) -> Self {
        self.columns = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }

    pub fn path(mut self, value: impl Into<String>) -> Self {
        self.path = Some(value.into());
        self
    }

    pub fn window_field(mut self, value: impl Into<String>) -> Self {
        self.window_field = Some(value.into());
        self
    }

    pub fn window_kind(mut self, value: ListReportsResponseDataItemWindowKind) -> Self {
        self.window_kind = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListReportsResponseDataItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`breakdowns`](ListReportsResponseDataItemBuilder::breakdowns)
    /// - [`columns`](ListReportsResponseDataItemBuilder::columns)
    /// - [`description`](ListReportsResponseDataItemBuilder::description)
    /// - [`key`](ListReportsResponseDataItemBuilder::key)
    /// - [`path`](ListReportsResponseDataItemBuilder::path)
    /// - [`window_kind`](ListReportsResponseDataItemBuilder::window_kind)
    pub fn build(self) -> Result<ListReportsResponseDataItem, BuildError> {
        Ok(ListReportsResponseDataItem {
            breakdowns: self
                .breakdowns
                .ok_or_else(|| BuildError::missing_field("breakdowns"))?,
            columns: self
                .columns
                .ok_or_else(|| BuildError::missing_field("columns"))?,
            description: self
                .description
                .ok_or_else(|| BuildError::missing_field("description"))?,
            key: self.key.ok_or_else(|| BuildError::missing_field("key"))?,
            path: self.path.ok_or_else(|| BuildError::missing_field("path"))?,
            window_field: self.window_field,
            window_kind: self
                .window_kind
                .ok_or_else(|| BuildError::missing_field("window_kind"))?,
        })
    }
}
