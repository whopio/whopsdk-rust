pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum ListEarningsResponseDataItemResource {
    OptionalListEarningsResponseDataItemResourceAlternativePaymentMethod(
        Option<ListEarningsResponseDataItemResourceAlternativePaymentMethod>,
    ),

    ListEarningsResponseDataItemResourceOne(ListEarningsResponseDataItemResourceOne),

    ListEarningsResponseDataItemResourceCreatedAt(ListEarningsResponseDataItemResourceCreatedAt),
}

impl ListEarningsResponseDataItemResource {
    pub fn is_optional_list_earnings_response_data_item_resource_alternative_payment_method(
        &self,
    ) -> bool {
        matches!(
            self,
            Self::OptionalListEarningsResponseDataItemResourceAlternativePaymentMethod(_)
        )
    }

    pub fn is_list_earnings_response_data_item_resource_one(&self) -> bool {
        matches!(self, Self::ListEarningsResponseDataItemResourceOne(_))
    }

    pub fn is_list_earnings_response_data_item_resource_created_at(&self) -> bool {
        matches!(self, Self::ListEarningsResponseDataItemResourceCreatedAt(_))
    }

    pub fn as_optional_list_earnings_response_data_item_resource_alternative_payment_method(
        &self,
    ) -> Option<&ListEarningsResponseDataItemResourceAlternativePaymentMethod> {
        match self {
            Self::OptionalListEarningsResponseDataItemResourceAlternativePaymentMethod(value) => {
                value.as_ref()
            }
            _ => None,
        }
    }

    pub fn into_optional_list_earnings_response_data_item_resource_alternative_payment_method(
        self,
    ) -> Option<ListEarningsResponseDataItemResourceAlternativePaymentMethod> {
        match self {
            Self::OptionalListEarningsResponseDataItemResourceAlternativePaymentMethod(value) => {
                value
            }
            _ => None,
        }
    }

    pub fn as_list_earnings_response_data_item_resource_one(
        &self,
    ) -> Option<&ListEarningsResponseDataItemResourceOne> {
        match self {
            Self::ListEarningsResponseDataItemResourceOne(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_list_earnings_response_data_item_resource_one(
        self,
    ) -> Option<ListEarningsResponseDataItemResourceOne> {
        match self {
            Self::ListEarningsResponseDataItemResourceOne(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_list_earnings_response_data_item_resource_created_at(
        &self,
    ) -> Option<&ListEarningsResponseDataItemResourceCreatedAt> {
        match self {
            Self::ListEarningsResponseDataItemResourceCreatedAt(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_list_earnings_response_data_item_resource_created_at(
        self,
    ) -> Option<ListEarningsResponseDataItemResourceCreatedAt> {
        match self {
            Self::ListEarningsResponseDataItemResourceCreatedAt(value) => Some(value),
            _ => None,
        }
    }
}
