pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateOverridesRequest {
    /// Whether commission applies to first payment or all payments (standard only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applies_to_payments: Option<AffiliateAppliesToPayments>,
    /// The commission type (percentage or flat_fee).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commission_type: Option<AffiliatePayoutTypes>,
    /// The commission value (percentage 1-100 or flat fee in dollars).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commission_value: Option<f64>,
    /// The revenue calculation basis (rev-share only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revenue_basis: Option<AffiliateRevenueBases>,
}

impl UpdateOverridesRequest {
    pub fn builder() -> UpdateOverridesRequestBuilder {
        <UpdateOverridesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateOverridesRequestBuilder {
    applies_to_payments: Option<AffiliateAppliesToPayments>,
    commission_type: Option<AffiliatePayoutTypes>,
    commission_value: Option<f64>,
    revenue_basis: Option<AffiliateRevenueBases>,
}

impl UpdateOverridesRequestBuilder {
    pub fn applies_to_payments(mut self, value: AffiliateAppliesToPayments) -> Self {
        self.applies_to_payments = Some(value);
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

    pub fn revenue_basis(mut self, value: AffiliateRevenueBases) -> Self {
        self.revenue_basis = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateOverridesRequest`].
    pub fn build(self) -> Result<UpdateOverridesRequest, BuildError> {
        Ok(UpdateOverridesRequest {
            applies_to_payments: self.applies_to_payments,
            commission_type: self.commission_type,
            commission_value: self.commission_value,
            revenue_basis: self.revenue_basis,
        })
    }
}
