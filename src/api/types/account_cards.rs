pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AccountCards {
    /// Whether the card application verifies a business (`business`, KYB) or a person (`individual`, consumer identity). `null` when the application is not yet linked to a verification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<AccountCardsKind>,
    /// Where the card application stands. `approved` means cards can be issued. `needs_verification` means the applicant has not completed identity verification yet; `needs_information` means they did, but the documents were rejected for a fixable reason and must be resubmitted. `pending` and `manual_review` are in flight. `denied`, `locked`, and `canceled` are terminal.
    pub status: AccountCardsStatus,
}

impl AccountCards {
    pub fn builder() -> AccountCardsBuilder {
        <AccountCardsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AccountCardsBuilder {
    kind: Option<AccountCardsKind>,
    status: Option<AccountCardsStatus>,
}

impl AccountCardsBuilder {
    pub fn kind(mut self, value: AccountCardsKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn status(mut self, value: AccountCardsStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AccountCards`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](AccountCardsBuilder::status)
    pub fn build(self) -> Result<AccountCards, BuildError> {
        Ok(AccountCards {
            kind: self.kind,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
