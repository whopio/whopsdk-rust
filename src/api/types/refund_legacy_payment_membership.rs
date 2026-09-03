pub use crate::prelude::*;

/// The membership attached to this payment.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RefundLegacyPaymentMembership {
    /// The unique identifier for the membership.
    #[serde(default)]
    pub id: String,
    /// The state of the membership.
    pub status: MembershipStatus,
}

impl RefundLegacyPaymentMembership {
    pub fn builder() -> RefundLegacyPaymentMembershipBuilder {
        <RefundLegacyPaymentMembershipBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RefundLegacyPaymentMembershipBuilder {
    id: Option<String>,
    status: Option<MembershipStatus>,
}

impl RefundLegacyPaymentMembershipBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn status(mut self, value: MembershipStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RefundLegacyPaymentMembership`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](RefundLegacyPaymentMembershipBuilder::id)
    /// - [`status`](RefundLegacyPaymentMembershipBuilder::status)
    pub fn build(self) -> Result<RefundLegacyPaymentMembership, BuildError> {
        Ok(RefundLegacyPaymentMembership {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
