pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum ListEarningsResponseDataItemResource {
    OptionalListEarningsResponseDataItemResourceAlternativePaymentMethod(
        Option<ListEarningsResponseDataItemResourceAlternativePaymentMethod>,
    ),

    ListEarningsResponseDataItemResourceOne(ListEarningsResponseDataItemResourceOne),

    ListEarningsResponseDataItemResourceTwo(ListEarningsResponseDataItemResourceTwo),

    ListEarningsResponseDataItemResourceMerchantName(
        ListEarningsResponseDataItemResourceMerchantName,
    ),

    ListEarningsResponseDataItemResourceSlug(ListEarningsResponseDataItemResourceSlug),

    ListEarningsResponseDataItemResourceFive(ListEarningsResponseDataItemResourceFive),

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

    pub fn is_list_earnings_response_data_item_resource_two(&self) -> bool {
        matches!(self, Self::ListEarningsResponseDataItemResourceTwo(_))
    }

    pub fn is_list_earnings_response_data_item_resource_merchant_name(&self) -> bool {
        matches!(
            self,
            Self::ListEarningsResponseDataItemResourceMerchantName(_)
        )
    }

    pub fn is_list_earnings_response_data_item_resource_slug(&self) -> bool {
        matches!(self, Self::ListEarningsResponseDataItemResourceSlug(_))
    }

    pub fn is_list_earnings_response_data_item_resource_five(&self) -> bool {
        matches!(self, Self::ListEarningsResponseDataItemResourceFive(_))
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

    pub fn as_list_earnings_response_data_item_resource_two(
        &self,
    ) -> Option<&ListEarningsResponseDataItemResourceTwo> {
        match self {
            Self::ListEarningsResponseDataItemResourceTwo(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_list_earnings_response_data_item_resource_two(
        self,
    ) -> Option<ListEarningsResponseDataItemResourceTwo> {
        match self {
            Self::ListEarningsResponseDataItemResourceTwo(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_list_earnings_response_data_item_resource_merchant_name(
        &self,
    ) -> Option<&ListEarningsResponseDataItemResourceMerchantName> {
        match self {
            Self::ListEarningsResponseDataItemResourceMerchantName(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_list_earnings_response_data_item_resource_merchant_name(
        self,
    ) -> Option<ListEarningsResponseDataItemResourceMerchantName> {
        match self {
            Self::ListEarningsResponseDataItemResourceMerchantName(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_list_earnings_response_data_item_resource_slug(
        &self,
    ) -> Option<&ListEarningsResponseDataItemResourceSlug> {
        match self {
            Self::ListEarningsResponseDataItemResourceSlug(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_list_earnings_response_data_item_resource_slug(
        self,
    ) -> Option<ListEarningsResponseDataItemResourceSlug> {
        match self {
            Self::ListEarningsResponseDataItemResourceSlug(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_list_earnings_response_data_item_resource_five(
        &self,
    ) -> Option<&ListEarningsResponseDataItemResourceFive> {
        match self {
            Self::ListEarningsResponseDataItemResourceFive(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_list_earnings_response_data_item_resource_five(
        self,
    ) -> Option<ListEarningsResponseDataItemResourceFive> {
        match self {
            Self::ListEarningsResponseDataItemResourceFive(value) => Some(value),
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
