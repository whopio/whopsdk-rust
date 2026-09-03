pub use crate::prelude::*;

/// The membership attached to this payment.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DisputeLegacyPaymentMembership {
    /// The unique identifier for the membership.
    #[serde(default)]
    pub id: String,
    /// The state of the membership.
    pub status: MembershipStatus,
}

impl DisputeLegacyPaymentMembership {
    pub fn builder() -> DisputeLegacyPaymentMembershipBuilder {
        <DisputeLegacyPaymentMembershipBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DisputeLegacyPaymentMembershipBuilder {
    id: Option<String>,
    status: Option<MembershipStatus>,
}

impl DisputeLegacyPaymentMembershipBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn status(mut self, value: MembershipStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DisputeLegacyPaymentMembership`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](DisputeLegacyPaymentMembershipBuilder::id)
    /// - [`status`](DisputeLegacyPaymentMembershipBuilder::status)
    pub fn build(self) -> Result<DisputeLegacyPaymentMembership, BuildError> {
        Ok(DisputeLegacyPaymentMembership {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
