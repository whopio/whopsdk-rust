pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostCardFrozenPayloadData {
    /// The billing address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing: Option<PostCardFrozenPayloadDataBilling>,
    /// When the card was canceled.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub canceled_at: Option<DateTime<FixedOffset>>,
    /// When the card was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub created_at: Option<DateTime<FixedOffset>>,
    /// Card expiration month.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_month: Option<String>,
    /// Card expiration year.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_year: Option<String>,
    /// Card ID, prefixed `icrd_`.
    #[serde(default)]
    pub id: String,
    /// Last four digits of the card number. `null` for pending invitation cards.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last4: Option<String>,
    /// The spending limit configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<PostCardFrozenPayloadDataLimit>,
    /// Card display name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub object: PostCardFrozenPayloadDataObject,
    /// Sensitive card details. Present only on `GET /cards/:id` for active cards; `null` when the card is inactive or details cannot be retrieved.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets: Option<PostCardFrozenPayloadDataSecrets>,
    /// Total spend in the last 30 days, in cents.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spent_last_month: Option<i64>,
    /// The card status. `denied` means the issuer declined the cardholder, so the card will never be issued.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<PostCardFrozenPayloadDataStatus>,
    /// The card type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<PostCardFrozenPayloadDataType>,
    /// Cardholder user ID, prefixed `user_`, when assigned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

impl PostCardFrozenPayloadData {
    pub fn builder() -> PostCardFrozenPayloadDataBuilder {
        <PostCardFrozenPayloadDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostCardFrozenPayloadDataBuilder {
    billing: Option<PostCardFrozenPayloadDataBilling>,
    canceled_at: Option<DateTime<FixedOffset>>,
    created_at: Option<DateTime<FixedOffset>>,
    expiration_month: Option<String>,
    expiration_year: Option<String>,
    id: Option<String>,
    last4: Option<String>,
    limit: Option<PostCardFrozenPayloadDataLimit>,
    name: Option<String>,
    object: Option<PostCardFrozenPayloadDataObject>,
    secrets: Option<PostCardFrozenPayloadDataSecrets>,
    spent_last_month: Option<i64>,
    status: Option<PostCardFrozenPayloadDataStatus>,
    r#type: Option<PostCardFrozenPayloadDataType>,
    user_id: Option<String>,
}

impl PostCardFrozenPayloadDataBuilder {
    pub fn billing(mut self, value: PostCardFrozenPayloadDataBilling) -> Self {
        self.billing = Some(value);
        self
    }

    pub fn canceled_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.canceled_at = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn expiration_month(mut self, value: impl Into<String>) -> Self {
        self.expiration_month = Some(value.into());
        self
    }

    pub fn expiration_year(mut self, value: impl Into<String>) -> Self {
        self.expiration_year = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn last4(mut self, value: impl Into<String>) -> Self {
        self.last4 = Some(value.into());
        self
    }

    pub fn limit(mut self, value: PostCardFrozenPayloadDataLimit) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn object(mut self, value: PostCardFrozenPayloadDataObject) -> Self {
        self.object = Some(value);
        self
    }

    pub fn secrets(mut self, value: PostCardFrozenPayloadDataSecrets) -> Self {
        self.secrets = Some(value);
        self
    }

    pub fn spent_last_month(mut self, value: i64) -> Self {
        self.spent_last_month = Some(value);
        self
    }

    pub fn status(mut self, value: PostCardFrozenPayloadDataStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn r#type(mut self, value: PostCardFrozenPayloadDataType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostCardFrozenPayloadData`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostCardFrozenPayloadDataBuilder::id)
    /// - [`object`](PostCardFrozenPayloadDataBuilder::object)
    pub fn build(self) -> Result<PostCardFrozenPayloadData, BuildError> {
        Ok(PostCardFrozenPayloadData {
            billing: self.billing,
            canceled_at: self.canceled_at,
            created_at: self.created_at,
            expiration_month: self.expiration_month,
            expiration_year: self.expiration_year,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            last4: self.last4,
            limit: self.limit,
            name: self.name,
            object: self
                .object
                .ok_or_else(|| BuildError::missing_field("object"))?,
            secrets: self.secrets,
            spent_last_month: self.spent_last_month,
            status: self.status,
            r#type: self.r#type,
            user_id: self.user_id,
        })
    }
}
