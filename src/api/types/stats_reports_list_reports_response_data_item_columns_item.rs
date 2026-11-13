pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ListReportsResponseDataItemColumnsItem {
    /// Metrics only. How to combine the metric across rows. `none` cannot be rebuilt from rows.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregate: Option<ListReportsResponseDataItemColumnsItemAggregate>,
    /// Properties only. The breakdowns this property can ride along with.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_on: Option<Vec<String>>,
    /// Metrics only. The breakdowns the metric supports, or `null` for every one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub breakdowns: Option<Vec<String>>,
    /// Metrics only. Whether the metric is returned when `columns` is omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<bool>,
    /// The column's key, as `columns` accepts it and as rows carry it.
    #[serde(default)]
    pub key: String,
    /// The column's display name.
    #[serde(default)]
    pub name: String,
    /// Properties only. `fixed` breakdowns list every value, zero-filled; `open` ones list values with activity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set: Option<ListReportsResponseDataItemColumnsItemSet>,
    /// Whether the column is an attribute of the row or a number measured over it.
    pub r#type: ListReportsResponseDataItemColumnsItemType,
    /// Metrics only. `money` is a Money object, `rate` a decimal fraction, the rest numbers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<ListReportsResponseDataItemColumnsItemUnit>,
    /// Metrics only. For `average` metrics, the column to weight by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<String>,
    /// Metrics only. The window tokens the metric supports, or `null` for every one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub windows: Option<Vec<String>>,
}

impl ListReportsResponseDataItemColumnsItem {
    pub fn builder() -> ListReportsResponseDataItemColumnsItemBuilder {
        <ListReportsResponseDataItemColumnsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListReportsResponseDataItemColumnsItemBuilder {
    aggregate: Option<ListReportsResponseDataItemColumnsItemAggregate>,
    available_on: Option<Vec<String>>,
    breakdowns: Option<Vec<String>>,
    default: Option<bool>,
    key: Option<String>,
    name: Option<String>,
    set: Option<ListReportsResponseDataItemColumnsItemSet>,
    r#type: Option<ListReportsResponseDataItemColumnsItemType>,
    unit: Option<ListReportsResponseDataItemColumnsItemUnit>,
    weight: Option<String>,
    windows: Option<Vec<String>>,
}

impl ListReportsResponseDataItemColumnsItemBuilder {
    pub fn aggregate(mut self, value: ListReportsResponseDataItemColumnsItemAggregate) -> Self {
        self.aggregate = Some(value);
        self
    }

    pub fn available_on(mut self, value: Vec<String>) -> Self {
        self.available_on = Some(value);
        self
    }

    pub fn breakdowns(mut self, value: Vec<String>) -> Self {
        self.breakdowns = Some(value);
        self
    }

    pub fn default(mut self, value: bool) -> Self {
        self.default = Some(value);
        self
    }

    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn set(mut self, value: ListReportsResponseDataItemColumnsItemSet) -> Self {
        self.set = Some(value);
        self
    }

    pub fn r#type(mut self, value: ListReportsResponseDataItemColumnsItemType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn unit(mut self, value: ListReportsResponseDataItemColumnsItemUnit) -> Self {
        self.unit = Some(value);
        self
    }

    pub fn weight(mut self, value: impl Into<String>) -> Self {
        self.weight = Some(value.into());
        self
    }

    pub fn windows(mut self, value: Vec<String>) -> Self {
        self.windows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListReportsResponseDataItemColumnsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`key`](ListReportsResponseDataItemColumnsItemBuilder::key)
    /// - [`name`](ListReportsResponseDataItemColumnsItemBuilder::name)
    /// - [`r#type`](ListReportsResponseDataItemColumnsItemBuilder::r#type)
    pub fn build(self) -> Result<ListReportsResponseDataItemColumnsItem, BuildError> {
        Ok(ListReportsResponseDataItemColumnsItem {
            aggregate: self.aggregate,
            available_on: self.available_on,
            breakdowns: self.breakdowns,
            default: self.default,
            key: self.key.ok_or_else(|| BuildError::missing_field("key"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            set: self.set,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            unit: self.unit,
            weight: self.weight,
            windows: self.windows,
        })
    }
}
