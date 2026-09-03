pub use crate::prelude::*;

/// The member attached to this payment.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentLegacyMember {
    /// The unique identifier for the company member.
    #[serde(default)]
    pub id: String,
    /// The phone number for the member, if available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}

impl PaymentLegacyMember {
    pub fn builder() -> PaymentLegacyMemberBuilder {
        <PaymentLegacyMemberBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentLegacyMemberBuilder {
    id: Option<String>,
    phone: Option<String>,
}

impl PaymentLegacyMemberBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn phone(mut self, value: impl Into<String>) -> Self {
        self.phone = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PaymentLegacyMember`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PaymentLegacyMemberBuilder::id)
    pub fn build(self) -> Result<PaymentLegacyMember, BuildError> {
        Ok(PaymentLegacyMember {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            phone: self.phone,
        })
    }
}
