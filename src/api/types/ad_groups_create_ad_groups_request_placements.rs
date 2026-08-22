pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum CreateAdGroupsRequestPlacements {
    CreateAdGroupsRequestPlacementsZero(CreateAdGroupsRequestPlacementsZero),

    CreateAdGroupsRequestPlacementsOneItemList(Vec<CreateAdGroupsRequestPlacementsOneItem>),
}

impl CreateAdGroupsRequestPlacements {
    pub fn is_create_ad_groups_request_placements_zero(&self) -> bool {
        matches!(self, Self::CreateAdGroupsRequestPlacementsZero(_))
    }

    pub fn is_create_ad_groups_request_placements_one_item_list(&self) -> bool {
        matches!(self, Self::CreateAdGroupsRequestPlacementsOneItemList(_))
    }

    pub fn as_create_ad_groups_request_placements_zero(
        &self,
    ) -> Option<&CreateAdGroupsRequestPlacementsZero> {
        match self {
            Self::CreateAdGroupsRequestPlacementsZero(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_create_ad_groups_request_placements_zero(
        self,
    ) -> Option<CreateAdGroupsRequestPlacementsZero> {
        match self {
            Self::CreateAdGroupsRequestPlacementsZero(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_create_ad_groups_request_placements_one_item_list(
        &self,
    ) -> Option<&Vec<CreateAdGroupsRequestPlacementsOneItem>> {
        match self {
            Self::CreateAdGroupsRequestPlacementsOneItemList(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_create_ad_groups_request_placements_one_item_list(
        self,
    ) -> Option<Vec<CreateAdGroupsRequestPlacementsOneItem>> {
        match self {
            Self::CreateAdGroupsRequestPlacementsOneItemList(value) => Some(value),
            _ => None,
        }
    }
}
