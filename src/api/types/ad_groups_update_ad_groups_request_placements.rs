pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum UpdateAdGroupsRequestPlacements {
    UpdateAdGroupsRequestPlacementsZero(UpdateAdGroupsRequestPlacementsZero),

    UpdateAdGroupsRequestPlacementsOneItemList(Vec<UpdateAdGroupsRequestPlacementsOneItem>),
}

impl UpdateAdGroupsRequestPlacements {
    pub fn is_update_ad_groups_request_placements_zero(&self) -> bool {
        matches!(self, Self::UpdateAdGroupsRequestPlacementsZero(_))
    }

    pub fn is_update_ad_groups_request_placements_one_item_list(&self) -> bool {
        matches!(self, Self::UpdateAdGroupsRequestPlacementsOneItemList(_))
    }

    pub fn as_update_ad_groups_request_placements_zero(
        &self,
    ) -> Option<&UpdateAdGroupsRequestPlacementsZero> {
        match self {
            Self::UpdateAdGroupsRequestPlacementsZero(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_update_ad_groups_request_placements_zero(
        self,
    ) -> Option<UpdateAdGroupsRequestPlacementsZero> {
        match self {
            Self::UpdateAdGroupsRequestPlacementsZero(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_update_ad_groups_request_placements_one_item_list(
        &self,
    ) -> Option<&Vec<UpdateAdGroupsRequestPlacementsOneItem>> {
        match self {
            Self::UpdateAdGroupsRequestPlacementsOneItemList(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_update_ad_groups_request_placements_one_item_list(
        self,
    ) -> Option<Vec<UpdateAdGroupsRequestPlacementsOneItem>> {
        match self {
            Self::UpdateAdGroupsRequestPlacementsOneItemList(value) => Some(value),
            _ => None,
        }
    }
}
