pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "override_type")]
#[non_exhaustive]
pub enum CreateOverridesRequestBody {
    #[serde(rename = "standard")]
    #[non_exhaustive]
    Standard {
        #[serde(skip_serializing_if = "Option::is_none")]
        applies_to_payments: Option<AffiliateAppliesToPayments>,
        #[serde(skip_serializing_if = "Option::is_none")]
        commission_type: Option<AffiliatePayoutTypes>,
        #[serde(default)]
        #[serde(with = "crate::core::number_serializers")]
        commission_value: f64,
        #[serde(default)]
        id: String,
        #[serde(default)]
        plan_id: String,
    },

    #[serde(rename = "rev_share")]
    #[non_exhaustive]
    RevShare {
        #[serde(skip_serializing_if = "Option::is_none")]
        commission_type: Option<AffiliatePayoutTypes>,
        #[serde(default)]
        #[serde(with = "crate::core::number_serializers")]
        commission_value: f64,
        #[serde(default)]
        id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        product_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        revenue_basis: Option<AffiliateRevenueBases>,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl CreateOverridesRequestBody {
    pub fn standard(commission_value: f64, id: String, plan_id: String) -> Self {
        Self::Standard {
            applies_to_payments: None,
            commission_type: None,
            commission_value,
            id,
            plan_id,
        }
    }

    pub fn rev_share(commission_value: f64, id: String) -> Self {
        Self::RevShare {
            commission_type: None,
            commission_value,
            id,
            product_id: None,
            revenue_basis: None,
        }
    }

    pub fn standard_with_applies_to_payments(
        applies_to_payments: AffiliateAppliesToPayments,
        commission_type: Option<AffiliatePayoutTypes>,
        commission_value: f64,
        id: String,
        plan_id: String,
    ) -> Self {
        Self::Standard {
            applies_to_payments: Some(applies_to_payments),
            commission_type,
            commission_value,
            id,
            plan_id,
        }
    }

    pub fn standard_with_commission_type(
        applies_to_payments: Option<AffiliateAppliesToPayments>,
        commission_type: AffiliatePayoutTypes,
        commission_value: f64,
        id: String,
        plan_id: String,
    ) -> Self {
        Self::Standard {
            applies_to_payments,
            commission_type: Some(commission_type),
            commission_value,
            id,
            plan_id,
        }
    }

    pub fn rev_share_with_commission_type(
        commission_type: AffiliatePayoutTypes,
        commission_value: f64,
        id: String,
        product_id: Option<String>,
        revenue_basis: Option<AffiliateRevenueBases>,
    ) -> Self {
        Self::RevShare {
            commission_type: Some(commission_type),
            commission_value,
            id,
            product_id,
            revenue_basis,
        }
    }

    pub fn rev_share_with_product_id(
        commission_type: Option<AffiliatePayoutTypes>,
        commission_value: f64,
        id: String,
        product_id: String,
        revenue_basis: Option<AffiliateRevenueBases>,
    ) -> Self {
        Self::RevShare {
            commission_type,
            commission_value,
            id,
            product_id: Some(product_id),
            revenue_basis,
        }
    }

    pub fn rev_share_with_revenue_basis(
        commission_type: Option<AffiliatePayoutTypes>,
        commission_value: f64,
        id: String,
        product_id: Option<String>,
        revenue_basis: AffiliateRevenueBases,
    ) -> Self {
        Self::RevShare {
            commission_type,
            commission_value,
            id,
            product_id,
            revenue_basis: Some(revenue_basis),
        }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
