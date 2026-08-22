pub use crate::prelude::*;

/// The membership attached to this payment.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RefundPaymentMembership {
    /// The unique identifier for the membership.
    #[serde(default)]
    pub id: String,
    /// The state of the membership.
    pub status: MembershipStatus,
}

impl RefundPaymentMembership {
    pub fn builder() -> RefundPaymentMembershipBuilder {
        <RefundPaymentMembershipBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RefundPaymentMembershipBuilder {
    id: Option<String>,
    status: Option<MembershipStatus>,
}

impl RefundPaymentMembershipBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn status(mut self, value: MembershipStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RefundPaymentMembership`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](RefundPaymentMembershipBuilder::id)
    /// - [`status`](RefundPaymentMembershipBuilder::status)
    pub fn build(self) -> Result<RefundPaymentMembership, BuildError> {
        Ok(RefundPaymentMembership {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
