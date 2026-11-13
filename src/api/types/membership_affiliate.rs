pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MembershipAffiliate {
    /// Whether the commission is paid on the membership's first payment only or on all its payments.
    pub applies_to_payments: MembershipAffiliateAppliesToPayments,
    /// The flat fee commission in the membership currency, or `null` for a percentage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commission_amount: Option<Money>,
    /// The percentage commission in percentage points, so `20` means 20%, or `null` for a flat fee.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub commission_percentage: Option<f64>,
    /// How the commission rate was set: `custom` for a rate set for this affiliate, `member` for the member affiliate rate of the plan or product, `global` for the product's global affiliate rate, or `marketplace` for a Whop marketplace referral.
    pub commission_source: MembershipAffiliateCommissionSource,
    /// Whether the commission is a percentage or a flat fee.
    pub commission_type: MembershipAffiliateCommissionType,
    /// Whether this membership pays affiliate commissions. `false` means the affiliate is kept on the membership but is no longer paid.
    #[serde(default)]
    pub enabled: bool,
    /// Affiliate ID, prefixed `aff_`.
    #[serde(default)]
    pub id: String,
    /// ID of the affiliate's user, prefixed `user_`.
    #[serde(default)]
    pub user_id: String,
}

impl MembershipAffiliate {
    pub fn builder() -> MembershipAffiliateBuilder {
        <MembershipAffiliateBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MembershipAffiliateBuilder {
    applies_to_payments: Option<MembershipAffiliateAppliesToPayments>,
    commission_amount: Option<Money>,
    commission_percentage: Option<f64>,
    commission_source: Option<MembershipAffiliateCommissionSource>,
    commission_type: Option<MembershipAffiliateCommissionType>,
    enabled: Option<bool>,
    id: Option<String>,
    user_id: Option<String>,
}

impl MembershipAffiliateBuilder {
    pub fn applies_to_payments(mut self, value: MembershipAffiliateAppliesToPayments) -> Self {
        self.applies_to_payments = Some(value);
        self
    }

    pub fn commission_amount(mut self, value: Money) -> Self {
        self.commission_amount = Some(value);
        self
    }

    pub fn commission_percentage(mut self, value: f64) -> Self {
        self.commission_percentage = Some(value);
        self
    }

    pub fn commission_source(mut self, value: MembershipAffiliateCommissionSource) -> Self {
        self.commission_source = Some(value);
        self
    }

    pub fn commission_type(mut self, value: MembershipAffiliateCommissionType) -> Self {
        self.commission_type = Some(value);
        self
    }

    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`MembershipAffiliate`].
    /// This method will fail if any of the following fields are not set:
    /// - [`applies_to_payments`](MembershipAffiliateBuilder::applies_to_payments)
    /// - [`commission_source`](MembershipAffiliateBuilder::commission_source)
    /// - [`commission_type`](MembershipAffiliateBuilder::commission_type)
    /// - [`enabled`](MembershipAffiliateBuilder::enabled)
    /// - [`id`](MembershipAffiliateBuilder::id)
    /// - [`user_id`](MembershipAffiliateBuilder::user_id)
    pub fn build(self) -> Result<MembershipAffiliate, BuildError> {
        Ok(MembershipAffiliate {
            applies_to_payments: self
                .applies_to_payments
                .ok_or_else(|| BuildError::missing_field("applies_to_payments"))?,
            commission_amount: self.commission_amount,
            commission_percentage: self.commission_percentage,
            commission_source: self
                .commission_source
                .ok_or_else(|| BuildError::missing_field("commission_source"))?,
            commission_type: self
                .commission_type
                .ok_or_else(|| BuildError::missing_field("commission_type"))?,
            enabled: self
                .enabled
                .ok_or_else(|| BuildError::missing_field("enabled"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            user_id: self
                .user_id
                .ok_or_else(|| BuildError::missing_field("user_id"))?,
        })
    }
}
