pub use crate::prelude::*;

/// The member attached to this payment.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentListItemMember {
    /// The unique identifier for the company member.
    #[serde(default)]
    pub id: String,
    /// The phone number for the member, if available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}

impl PaymentListItemMember {
    pub fn builder() -> PaymentListItemMemberBuilder {
        <PaymentListItemMemberBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentListItemMemberBuilder {
    id: Option<String>,
    phone: Option<String>,
}

impl PaymentListItemMemberBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn phone(mut self, value: impl Into<String>) -> Self {
        self.phone = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PaymentListItemMember`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PaymentListItemMemberBuilder::id)
    pub fn build(self) -> Result<PaymentListItemMember, BuildError> {
        Ok(PaymentListItemMember {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            phone: self.phone,
        })
    }
}
