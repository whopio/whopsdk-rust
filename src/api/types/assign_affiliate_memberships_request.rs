pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AssignAffiliateMembershipsRequest {
    /// Whether the commission is a percentage of each payment or a fixed amount per payment.
    pub commission_type: AssignAffiliateMembershipsRequestCommissionType,
    /// A whole number from 1 to 90 for `percentage`, or an amount greater than 1 in the membership currency for `flat_fee`. The payout cannot exceed 90% of the next renewal amount. Flat fees need matching billing and settlement currencies.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub commission_value: f64,
    /// Email address of the user to assign. Pass exactly one of `user_id`, `email`, or `username`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// The user to assign, prefixed `user_`. Pass exactly one of `user_id`, `email`, or `username`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// Whop username of the user to assign. Pass exactly one of `user_id`, `email`, or `username`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

impl AssignAffiliateMembershipsRequest {
    pub fn builder() -> AssignAffiliateMembershipsRequestBuilder {
        <AssignAffiliateMembershipsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AssignAffiliateMembershipsRequestBuilder {
    commission_type: Option<AssignAffiliateMembershipsRequestCommissionType>,
    commission_value: Option<f64>,
    email: Option<String>,
    user_id: Option<String>,
    username: Option<String>,
}

impl AssignAffiliateMembershipsRequestBuilder {
    pub fn commission_type(
        mut self,
        value: AssignAffiliateMembershipsRequestCommissionType,
    ) -> Self {
        self.commission_type = Some(value);
        self
    }

    pub fn commission_value(mut self, value: f64) -> Self {
        self.commission_value = Some(value);
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    pub fn username(mut self, value: impl Into<String>) -> Self {
        self.username = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AssignAffiliateMembershipsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`commission_type`](AssignAffiliateMembershipsRequestBuilder::commission_type)
    /// - [`commission_value`](AssignAffiliateMembershipsRequestBuilder::commission_value)
    pub fn build(self) -> Result<AssignAffiliateMembershipsRequest, BuildError> {
        Ok(AssignAffiliateMembershipsRequest {
            commission_type: self
                .commission_type
                .ok_or_else(|| BuildError::missing_field("commission_type"))?,
            commission_value: self
                .commission_value
                .ok_or_else(|| BuildError::missing_field("commission_value"))?,
            email: self.email,
            user_id: self.user_id,
            username: self.username,
        })
    }
}
