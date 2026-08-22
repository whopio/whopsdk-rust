pub use crate::prelude::*;

/// The membership attached to this payment.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PaymentMembership {
    /// The unique identifier for the membership.
    #[serde(default)]
    pub id: String,
    /// The phone number associated with this membership.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    /// The state of the membership.
    pub status: MembershipStatus,
}

impl PaymentMembership {
    pub fn builder() -> PaymentMembershipBuilder {
        <PaymentMembershipBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentMembershipBuilder {
    id: Option<String>,
    phone_number: Option<String>,
    status: Option<MembershipStatus>,
}

impl PaymentMembershipBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn phone_number(mut self, value: impl Into<String>) -> Self {
        self.phone_number = Some(value.into());
        self
    }

    pub fn status(mut self, value: MembershipStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaymentMembership`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PaymentMembershipBuilder::id)
    /// - [`status`](PaymentMembershipBuilder::status)
    pub fn build(self) -> Result<PaymentMembership, BuildError> {
        Ok(PaymentMembership {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            phone_number: self.phone_number,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
