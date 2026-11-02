pub use crate::prelude::*;

/// The member attached to this payment.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RefundLegacyPaymentMember {
    /// The unique identifier for the company member.
    #[serde(default)]
    pub id: String,
    /// The phone number for the member, if available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}

impl RefundLegacyPaymentMember {
    pub fn builder() -> RefundLegacyPaymentMemberBuilder {
        <RefundLegacyPaymentMemberBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RefundLegacyPaymentMemberBuilder {
    id: Option<String>,
    phone: Option<String>,
}

impl RefundLegacyPaymentMemberBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn phone(mut self, value: impl Into<String>) -> Self {
        self.phone = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RefundLegacyPaymentMember`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](RefundLegacyPaymentMemberBuilder::id)
    pub fn build(self) -> Result<RefundLegacyPaymentMember, BuildError> {
        Ok(RefundLegacyPaymentMember {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            phone: self.phone,
        })
    }
}
