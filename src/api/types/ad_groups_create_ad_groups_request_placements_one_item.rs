pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateAdGroupsRequestPlacementsOneItem {
    /// Platform the ads run on.
    pub platform: CreateAdGroupsRequestPlacementsOneItemPlatform,
    /// Positions to target within the platform, such as `feed` or `story`. Omit to target all of the platform's positions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub positions: Option<Vec<String>>,
}

impl CreateAdGroupsRequestPlacementsOneItem {
    pub fn builder() -> CreateAdGroupsRequestPlacementsOneItemBuilder {
        <CreateAdGroupsRequestPlacementsOneItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateAdGroupsRequestPlacementsOneItemBuilder {
    platform: Option<CreateAdGroupsRequestPlacementsOneItemPlatform>,
    positions: Option<Vec<String>>,
}

impl CreateAdGroupsRequestPlacementsOneItemBuilder {
    pub fn platform(mut self, value: CreateAdGroupsRequestPlacementsOneItemPlatform) -> Self {
        self.platform = Some(value);
        self
    }

    pub fn positions(mut self, value: Vec<String>) -> Self {
        self.positions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateAdGroupsRequestPlacementsOneItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`platform`](CreateAdGroupsRequestPlacementsOneItemBuilder::platform)
    pub fn build(self) -> Result<CreateAdGroupsRequestPlacementsOneItem, BuildError> {
        Ok(CreateAdGroupsRequestPlacementsOneItem {
            platform: self
                .platform
                .ok_or_else(|| BuildError::missing_field("platform"))?,
            positions: self.positions,
        })
    }
}
