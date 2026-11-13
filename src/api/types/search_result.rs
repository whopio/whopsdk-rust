pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SearchResult {
    /// Why the row matched, or `null` for a ranked row whose whole ID or whole title isn't the query. `id`: its ID, or a related one such as its user's, its dispute's, or a plan or payout ID for an account. `link`: a whop.com store or product link. `name`: its whole title. `email`: the email of the user it belongs to. `owner_email`, `team_member_email` or `customer_email`: the email of a subaccount's owner, a member of its team, or one of its customers. `card`: its card's first 6 or 8 and last 4 digits, or the last 4 alone. `phone`: the phone number of the user it belongs to. `payment_number`: the number of the invoice a payment collects, so an invoice from `GET /invoices` comes back as its `payment`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_reason: Option<SearchResultMatchReason>,
    /// The matched record. Get the full record from its own endpoint by `id`.
    pub record: SearchRecord,
}

impl SearchResult {
    pub fn builder() -> SearchResultBuilder {
        <SearchResultBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SearchResultBuilder {
    match_reason: Option<SearchResultMatchReason>,
    record: Option<SearchRecord>,
}

impl SearchResultBuilder {
    pub fn match_reason(mut self, value: SearchResultMatchReason) -> Self {
        self.match_reason = Some(value);
        self
    }

    pub fn record(mut self, value: SearchRecord) -> Self {
        self.record = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SearchResult`].
    /// This method will fail if any of the following fields are not set:
    /// - [`record`](SearchResultBuilder::record)
    pub fn build(self) -> Result<SearchResult, BuildError> {
        Ok(SearchResult {
            match_reason: self.match_reason,
            record: self
                .record
                .ok_or_else(|| BuildError::missing_field("record"))?,
        })
    }
}
