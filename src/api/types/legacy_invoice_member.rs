pub use crate::prelude::*;

/// The member that the invoice was created for. Null when the invoice is addressed to an email address with no member record behind it.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LegacyInvoiceMember {
    /// The unique identifier for the company member.
    #[serde(default)]
    pub id: String,
}

impl LegacyInvoiceMember {
    pub fn builder() -> LegacyInvoiceMemberBuilder {
        <LegacyInvoiceMemberBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LegacyInvoiceMemberBuilder {
    id: Option<String>,
}

impl LegacyInvoiceMemberBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`LegacyInvoiceMember`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](LegacyInvoiceMemberBuilder::id)
    pub fn build(self) -> Result<LegacyInvoiceMember, BuildError> {
        Ok(LegacyInvoiceMember {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
