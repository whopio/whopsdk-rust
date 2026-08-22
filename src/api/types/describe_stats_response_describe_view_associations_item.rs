pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DescribeStatsResponseDescribeViewAssociationsItem {
    /// The association name.
    #[serde(default)]
    pub name: String,
    /// The type (belongs_to, has_many, has_one, event, namespace).
    #[serde(default)]
    pub r#type: String,
    /// The associated model class name (for model associations).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// The full path (for event associations).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// The event name (for event type).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_name: Option<String>,
}

impl DescribeStatsResponseDescribeViewAssociationsItem {
    pub fn builder() -> DescribeStatsResponseDescribeViewAssociationsItemBuilder {
        <DescribeStatsResponseDescribeViewAssociationsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DescribeStatsResponseDescribeViewAssociationsItemBuilder {
    name: Option<String>,
    r#type: Option<String>,
    model: Option<String>,
    path: Option<String>,
    event_name: Option<String>,
}

impl DescribeStatsResponseDescribeViewAssociationsItemBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn model(mut self, value: impl Into<String>) -> Self {
        self.model = Some(value.into());
        self
    }

    pub fn path(mut self, value: impl Into<String>) -> Self {
        self.path = Some(value.into());
        self
    }

    pub fn event_name(mut self, value: impl Into<String>) -> Self {
        self.event_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DescribeStatsResponseDescribeViewAssociationsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](DescribeStatsResponseDescribeViewAssociationsItemBuilder::name)
    /// - [`r#type`](DescribeStatsResponseDescribeViewAssociationsItemBuilder::r#type)
    pub fn build(self) -> Result<DescribeStatsResponseDescribeViewAssociationsItem, BuildError> {
        Ok(DescribeStatsResponseDescribeViewAssociationsItem {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            model: self.model,
            path: self.path,
            event_name: self.event_name,
        })
    }
}
