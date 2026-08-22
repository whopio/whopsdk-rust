pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateMethodsRequest {
    /// The account to add the payout method for, prefixed `biz_`. Provide this or `user_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// Currency the supported payout method delivers payouts in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_currency: Option<String>,
    /// The supported payout method's required field values, keyed by field id — list them with `GET /payouts/supported_methods?supported_payout_method_id=...`. Field ids are stable `fld_` identifiers you may hardcode; they never change for a given field. A Basis Theory token id may be passed in place of a raw value. For a U.S. bank routing-number field, a raw nine-digit value must also pass the ABA checksum. A validation failure returns the method's full required_fields schema alongside the error. Required whenever the account details are supplied directly.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<HashMap<String, String>>,
    /// Whether to make this the account's default payout method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    /// A label for the payout method, unique per destination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    /// The supported payout method to save (a podst_ identifier from a previous listing).
    #[serde(default)]
    pub supported_payout_method_id: String,
    /// The user to add the payout method for, prefixed `user_`. Provide this or `account_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

impl CreateMethodsRequest {
    pub fn builder() -> CreateMethodsRequestBuilder {
        <CreateMethodsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateMethodsRequestBuilder {
    account_id: Option<String>,
    destination_currency: Option<String>,
    fields: Option<HashMap<String, String>>,
    is_default: Option<bool>,
    nickname: Option<String>,
    supported_payout_method_id: Option<String>,
    user_id: Option<String>,
}

impl CreateMethodsRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn destination_currency(mut self, value: impl Into<String>) -> Self {
        self.destination_currency = Some(value.into());
        self
    }

    pub fn fields(mut self, value: HashMap<String, String>) -> Self {
        self.fields = Some(value);
        self
    }

    pub fn is_default(mut self, value: bool) -> Self {
        self.is_default = Some(value);
        self
    }

    pub fn nickname(mut self, value: impl Into<String>) -> Self {
        self.nickname = Some(value.into());
        self
    }

    pub fn supported_payout_method_id(mut self, value: impl Into<String>) -> Self {
        self.supported_payout_method_id = Some(value.into());
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateMethodsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`supported_payout_method_id`](CreateMethodsRequestBuilder::supported_payout_method_id)
    pub fn build(self) -> Result<CreateMethodsRequest, BuildError> {
        Ok(CreateMethodsRequest {
            account_id: self.account_id,
            destination_currency: self.destination_currency,
            fields: self.fields,
            is_default: self.is_default,
            nickname: self.nickname,
            supported_payout_method_id: self
                .supported_payout_method_id
                .ok_or_else(|| BuildError::missing_field("supported_payout_method_id"))?,
            user_id: self.user_id,
        })
    }
}
