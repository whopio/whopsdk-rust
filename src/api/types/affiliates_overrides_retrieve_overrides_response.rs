pub use crate::prelude::*;

/// A commission configuration for an affiliate, defining payout terms for a specific plan or revenue share
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RetrieveOverridesResponse {
    /// Whether the commission applies to the first payment only or all payments (standard overrides only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applies_to_payments: Option<AffiliateAppliesToPayments>,
    /// Whether this rev-share override applies to a single product or all products (rev-share only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applies_to_products: Option<AffiliateAppliesToProducts>,
    /// The checkout direct link for referrals (standard overrides only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkout_direct_link: Option<String>,
    /// The type of commission (percentage or flat_fee).
    pub commission_type: AffiliatePayoutTypes,
    /// The commission amount. A percentage (1-100) when commission_type is percentage, or a dollar amount when flat_fee.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub commission_value: f64,
    /// The unique identifier for the affiliate override.
    #[serde(default)]
    pub id: String,
    /// The type of override (standard or rev_share).
    pub override_type: AffiliateOverrideRoles,
    /// The plan ID (for standard overrides).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    /// The product page direct link for referrals (standard overrides only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_direct_link: Option<String>,
    /// The product ID (for rev-share overrides).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    /// The revenue calculation basis (pre_fees or post_fees).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revenue_basis: Option<AffiliateRevenueBases>,
    /// The total earnings paid to this affiliate for referrals to this specific plan, in USD.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub total_referral_earnings_usd: f64,
}

impl RetrieveOverridesResponse {
    pub fn builder() -> RetrieveOverridesResponseBuilder {
        <RetrieveOverridesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RetrieveOverridesResponseBuilder {
    applies_to_payments: Option<AffiliateAppliesToPayments>,
    applies_to_products: Option<AffiliateAppliesToProducts>,
    checkout_direct_link: Option<String>,
    commission_type: Option<AffiliatePayoutTypes>,
    commission_value: Option<f64>,
    id: Option<String>,
    override_type: Option<AffiliateOverrideRoles>,
    plan_id: Option<String>,
    product_direct_link: Option<String>,
    product_id: Option<String>,
    revenue_basis: Option<AffiliateRevenueBases>,
    total_referral_earnings_usd: Option<f64>,
}

impl RetrieveOverridesResponseBuilder {
    pub fn applies_to_payments(mut self, value: AffiliateAppliesToPayments) -> Self {
        self.applies_to_payments = Some(value);
        self
    }

    pub fn applies_to_products(mut self, value: AffiliateAppliesToProducts) -> Self {
        self.applies_to_products = Some(value);
        self
    }

    pub fn checkout_direct_link(mut self, value: impl Into<String>) -> Self {
        self.checkout_direct_link = Some(value.into());
        self
    }

    pub fn commission_type(mut self, value: AffiliatePayoutTypes) -> Self {
        self.commission_type = Some(value);
        self
    }

    pub fn commission_value(mut self, value: f64) -> Self {
        self.commission_value = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn override_type(mut self, value: AffiliateOverrideRoles) -> Self {
        self.override_type = Some(value);
        self
    }

    pub fn plan_id(mut self, value: impl Into<String>) -> Self {
        self.plan_id = Some(value.into());
        self
    }

    pub fn product_direct_link(mut self, value: impl Into<String>) -> Self {
        self.product_direct_link = Some(value.into());
        self
    }

    pub fn product_id(mut self, value: impl Into<String>) -> Self {
        self.product_id = Some(value.into());
        self
    }

    pub fn revenue_basis(mut self, value: AffiliateRevenueBases) -> Self {
        self.revenue_basis = Some(value);
        self
    }

    pub fn total_referral_earnings_usd(mut self, value: f64) -> Self {
        self.total_referral_earnings_usd = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RetrieveOverridesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`commission_type`](RetrieveOverridesResponseBuilder::commission_type)
    /// - [`commission_value`](RetrieveOverridesResponseBuilder::commission_value)
    /// - [`id`](RetrieveOverridesResponseBuilder::id)
    /// - [`override_type`](RetrieveOverridesResponseBuilder::override_type)
    /// - [`total_referral_earnings_usd`](RetrieveOverridesResponseBuilder::total_referral_earnings_usd)
    pub fn build(self) -> Result<RetrieveOverridesResponse, BuildError> {
        Ok(RetrieveOverridesResponse {
            applies_to_payments: self.applies_to_payments,
            applies_to_products: self.applies_to_products,
            checkout_direct_link: self.checkout_direct_link,
            commission_type: self
                .commission_type
                .ok_or_else(|| BuildError::missing_field("commission_type"))?,
            commission_value: self
                .commission_value
                .ok_or_else(|| BuildError::missing_field("commission_value"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            override_type: self
                .override_type
                .ok_or_else(|| BuildError::missing_field("override_type"))?,
            plan_id: self.plan_id,
            product_direct_link: self.product_direct_link,
            product_id: self.product_id,
            revenue_basis: self.revenue_basis,
            total_referral_earnings_usd: self
                .total_referral_earnings_usd
                .ok_or_else(|| BuildError::missing_field("total_referral_earnings_usd"))?,
        })
    }
}
