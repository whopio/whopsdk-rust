pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AccountFinancing {
    /// The ID of the account's most recent financing application, starting with inrq_. Retrieve it for requirements and review feedback.
    #[serde(default)]
    pub application_id: String,
    /// The most recent financing application's review state: `requires_collection` (waiting on the merchant, including after a request for more information), `awaiting_review`, `approved`, `denied`, or `completed`. Whether financing payment methods are enabled is `capabilities.accept_bnpl_payments`.
    pub status: AccountFinancingStatus,
}

impl AccountFinancing {
    pub fn builder() -> AccountFinancingBuilder {
        <AccountFinancingBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AccountFinancingBuilder {
    application_id: Option<String>,
    status: Option<AccountFinancingStatus>,
}

impl AccountFinancingBuilder {
    pub fn application_id(mut self, value: impl Into<String>) -> Self {
        self.application_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: AccountFinancingStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AccountFinancing`].
    /// This method will fail if any of the following fields are not set:
    /// - [`application_id`](AccountFinancingBuilder::application_id)
    /// - [`status`](AccountFinancingBuilder::status)
    pub fn build(self) -> Result<AccountFinancing, BuildError> {
        Ok(AccountFinancing {
            application_id: self
                .application_id
                .ok_or_else(|| BuildError::missing_field("application_id"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
