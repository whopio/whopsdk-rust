pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateAdGroupsRequestDeliveryScheduleMondayItem {
    /// When delivery stops, as a 24-hour `HH:00` time up to `24:00` for midnight. Later than `start`.
    #[serde(default)]
    pub end: String,
    /// When delivery starts, as a 24-hour `HH:00` time from `00:00` to `23:00`.
    #[serde(default)]
    pub start: String,
}

impl CreateAdGroupsRequestDeliveryScheduleMondayItem {
    pub fn builder() -> CreateAdGroupsRequestDeliveryScheduleMondayItemBuilder {
        <CreateAdGroupsRequestDeliveryScheduleMondayItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateAdGroupsRequestDeliveryScheduleMondayItemBuilder {
    end: Option<String>,
    start: Option<String>,
}

impl CreateAdGroupsRequestDeliveryScheduleMondayItemBuilder {
    pub fn end(mut self, value: impl Into<String>) -> Self {
        self.end = Some(value.into());
        self
    }

    pub fn start(mut self, value: impl Into<String>) -> Self {
        self.start = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateAdGroupsRequestDeliveryScheduleMondayItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`end`](CreateAdGroupsRequestDeliveryScheduleMondayItemBuilder::end)
    /// - [`start`](CreateAdGroupsRequestDeliveryScheduleMondayItemBuilder::start)
    pub fn build(self) -> Result<CreateAdGroupsRequestDeliveryScheduleMondayItem, BuildError> {
        Ok(CreateAdGroupsRequestDeliveryScheduleMondayItem {
            end: self.end.ok_or_else(|| BuildError::missing_field("end"))?,
            start: self
                .start
                .ok_or_else(|| BuildError::missing_field("start"))?,
        })
    }
}
