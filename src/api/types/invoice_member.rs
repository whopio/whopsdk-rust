pub use crate::prelude::*;

/// The member that the invoice was created for. Null when the invoice is addressed to an email address with no member record behind it.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvoiceMember {
    /// The unique identifier for the company member.
    #[serde(default)]
    pub id: String,
}

impl InvoiceMember {
    pub fn builder() -> InvoiceMemberBuilder {
        <InvoiceMemberBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoiceMemberBuilder {
    id: Option<String>,
}

impl InvoiceMemberBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`InvoiceMember`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](InvoiceMemberBuilder::id)
    pub fn build(self) -> Result<InvoiceMember, BuildError> {
        Ok(InvoiceMember {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
