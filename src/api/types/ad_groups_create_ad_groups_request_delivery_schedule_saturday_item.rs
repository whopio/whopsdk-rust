pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateAdGroupsRequestDeliveryScheduleSaturdayItem {
    /// When delivery stops, as a 24-hour `HH:00` time up to `24:00` for midnight. Later than `start`.
    #[serde(default)]
    pub end: String,
    /// When delivery starts, as a 24-hour `HH:00` time from `00:00` to `23:00`.
    #[serde(default)]
    pub start: String,
}

impl CreateAdGroupsRequestDeliveryScheduleSaturdayItem {
    pub fn builder() -> CreateAdGroupsRequestDeliveryScheduleSaturdayItemBuilder {
        <CreateAdGroupsRequestDeliveryScheduleSaturdayItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateAdGroupsRequestDeliveryScheduleSaturdayItemBuilder {
    end: Option<String>,
    start: Option<String>,
}

impl CreateAdGroupsRequestDeliveryScheduleSaturdayItemBuilder {
    pub fn end(mut self, value: impl Into<String>) -> Self {
        self.end = Some(value.into());
        self
    }

    pub fn start(mut self, value: impl Into<String>) -> Self {
        self.start = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateAdGroupsRequestDeliveryScheduleSaturdayItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`end`](CreateAdGroupsRequestDeliveryScheduleSaturdayItemBuilder::end)
    /// - [`start`](CreateAdGroupsRequestDeliveryScheduleSaturdayItemBuilder::start)
    pub fn build(self) -> Result<CreateAdGroupsRequestDeliveryScheduleSaturdayItem, BuildError> {
        Ok(CreateAdGroupsRequestDeliveryScheduleSaturdayItem {
            end: self.end.ok_or_else(|| BuildError::missing_field("end"))?,
            start: self
                .start
                .ok_or_else(|| BuildError::missing_field("start"))?,
        })
    }
}
