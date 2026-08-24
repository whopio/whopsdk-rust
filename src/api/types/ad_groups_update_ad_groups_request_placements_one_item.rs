pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct UpdateAdGroupsRequestPlacementsOneItem {
    /// Platform the ads run on.
    pub platform: UpdateAdGroupsRequestPlacementsOneItemPlatform,
    /// Positions to target within the platform, such as `feed` or `story`. Omit to target all of the platform's positions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub positions: Option<Vec<String>>,
}

impl UpdateAdGroupsRequestPlacementsOneItem {
    pub fn builder() -> UpdateAdGroupsRequestPlacementsOneItemBuilder {
        <UpdateAdGroupsRequestPlacementsOneItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateAdGroupsRequestPlacementsOneItemBuilder {
    platform: Option<UpdateAdGroupsRequestPlacementsOneItemPlatform>,
    positions: Option<Vec<String>>,
}

impl UpdateAdGroupsRequestPlacementsOneItemBuilder {
    pub fn platform(mut self, value: UpdateAdGroupsRequestPlacementsOneItemPlatform) -> Self {
        self.platform = Some(value);
        self
    }

    pub fn positions(mut self, value: Vec<String>) -> Self {
        self.positions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateAdGroupsRequestPlacementsOneItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`platform`](UpdateAdGroupsRequestPlacementsOneItemBuilder::platform)
    pub fn build(self) -> Result<UpdateAdGroupsRequestPlacementsOneItem, BuildError> {
        Ok(UpdateAdGroupsRequestPlacementsOneItem {
            platform: self
                .platform
                .ok_or_else(|| BuildError::missing_field("platform"))?,
            positions: self.positions,
        })
    }
}
