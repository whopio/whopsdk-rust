pub use crate::prelude::*;

/// The membership attached to this payment.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PaymentListItemMembership {
    /// The unique identifier for the membership.
    #[serde(default)]
    pub id: String,
    /// The phone number associated with this membership.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    /// The state of the membership.
    pub status: MembershipStatus,
}

impl PaymentListItemMembership {
    pub fn builder() -> PaymentListItemMembershipBuilder {
        <PaymentListItemMembershipBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentListItemMembershipBuilder {
    id: Option<String>,
    phone_number: Option<String>,
    status: Option<MembershipStatus>,
}

impl PaymentListItemMembershipBuilder {
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

    /// Consumes the builder and constructs a [`PaymentListItemMembership`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PaymentListItemMembershipBuilder::id)
    /// - [`status`](PaymentListItemMembershipBuilder::status)
    pub fn build(self) -> Result<PaymentListItemMembership, BuildError> {
        Ok(PaymentListItemMembership {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            phone_number: self.phone_number,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
