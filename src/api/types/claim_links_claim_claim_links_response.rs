pub use crate::prelude::*;

/// A shareable link anyone holding its code can open to claim the funds.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ClaimClaimLinksResponse {
    /// Amount each claim receives.
    #[serde(default)]
    pub amount: String,
    /// Shareable claim URL. Null when retrieving by ID without claim-link management access.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub claim_url: Option<String>,
    /// Whether this link can currently be claimed.
    #[serde(default)]
    pub claimable: bool,
    /// Redeemable claim code. Null when retrieving by ID without claim-link management access.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(default)]
    pub currency: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub expires_at: Option<DateTime<FixedOffset>>,
    /// Claim-link ID, prefixed `airdrp_`.
    #[serde(default)]
    pub id: String,
    pub object: ClaimClaimLinksResponseObject,
    /// Maximum number of claims.
    #[serde(default)]
    pub redeemable_count: i64,
    /// Number of completed claims.
    #[serde(default)]
    pub redeemed_count: i64,
    /// Number of available claims, excluding slots reserved by pending claims.
    #[serde(default)]
    pub remaining_claims: i64,
    /// The public account or user funding the claim link.
    pub sender: ClaimClaimLinksResponseSender,
    #[serde(default)]
    pub source: ClaimClaimLinksResponseSource,
    /// The link's lifecycle status. A newly funded link is `pending`. Check `claimable` for current availability, including expiry and reserved claims.
    pub status: ClaimClaimLinksResponseStatus,
}

impl ClaimClaimLinksResponse {
    pub fn builder() -> ClaimClaimLinksResponseBuilder {
        <ClaimClaimLinksResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ClaimClaimLinksResponseBuilder {
    amount: Option<String>,
    claim_url: Option<String>,
    claimable: Option<bool>,
    code: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    currency: Option<String>,
    expires_at: Option<DateTime<FixedOffset>>,
    id: Option<String>,
    object: Option<ClaimClaimLinksResponseObject>,
    redeemable_count: Option<i64>,
    redeemed_count: Option<i64>,
    remaining_claims: Option<i64>,
    sender: Option<ClaimClaimLinksResponseSender>,
    source: Option<ClaimClaimLinksResponseSource>,
    status: Option<ClaimClaimLinksResponseStatus>,
}

impl ClaimClaimLinksResponseBuilder {
    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    pub fn claim_url(mut self, value: impl Into<String>) -> Self {
        self.claim_url = Some(value.into());
        self
    }

    pub fn claimable(mut self, value: bool) -> Self {
        self.claimable = Some(value);
        self
    }

    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn expires_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.expires_at = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn object(mut self, value: ClaimClaimLinksResponseObject) -> Self {
        self.object = Some(value);
        self
    }

    pub fn redeemable_count(mut self, value: i64) -> Self {
        self.redeemable_count = Some(value);
        self
    }

    pub fn redeemed_count(mut self, value: i64) -> Self {
        self.redeemed_count = Some(value);
        self
    }

    pub fn remaining_claims(mut self, value: i64) -> Self {
        self.remaining_claims = Some(value);
        self
    }

    pub fn sender(mut self, value: ClaimClaimLinksResponseSender) -> Self {
        self.sender = Some(value);
        self
    }

    pub fn source(mut self, value: ClaimClaimLinksResponseSource) -> Self {
        self.source = Some(value);
        self
    }

    pub fn status(mut self, value: ClaimClaimLinksResponseStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ClaimClaimLinksResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount`](ClaimClaimLinksResponseBuilder::amount)
    /// - [`claimable`](ClaimClaimLinksResponseBuilder::claimable)
    /// - [`created_at`](ClaimClaimLinksResponseBuilder::created_at)
    /// - [`currency`](ClaimClaimLinksResponseBuilder::currency)
    /// - [`id`](ClaimClaimLinksResponseBuilder::id)
    /// - [`object`](ClaimClaimLinksResponseBuilder::object)
    /// - [`redeemable_count`](ClaimClaimLinksResponseBuilder::redeemable_count)
    /// - [`redeemed_count`](ClaimClaimLinksResponseBuilder::redeemed_count)
    /// - [`remaining_claims`](ClaimClaimLinksResponseBuilder::remaining_claims)
    /// - [`sender`](ClaimClaimLinksResponseBuilder::sender)
    /// - [`source`](ClaimClaimLinksResponseBuilder::source)
    /// - [`status`](ClaimClaimLinksResponseBuilder::status)
    pub fn build(self) -> Result<ClaimClaimLinksResponse, BuildError> {
        Ok(ClaimClaimLinksResponse {
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            claim_url: self.claim_url,
            claimable: self
                .claimable
                .ok_or_else(|| BuildError::missing_field("claimable"))?,
            code: self.code,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            expires_at: self.expires_at,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            object: self
                .object
                .ok_or_else(|| BuildError::missing_field("object"))?,
            redeemable_count: self
                .redeemable_count
                .ok_or_else(|| BuildError::missing_field("redeemable_count"))?,
            redeemed_count: self
                .redeemed_count
                .ok_or_else(|| BuildError::missing_field("redeemed_count"))?,
            remaining_claims: self
                .remaining_claims
                .ok_or_else(|| BuildError::missing_field("remaining_claims"))?,
            sender: self
                .sender
                .ok_or_else(|| BuildError::missing_field("sender"))?,
            source: self
                .source
                .ok_or_else(|| BuildError::missing_field("source"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
