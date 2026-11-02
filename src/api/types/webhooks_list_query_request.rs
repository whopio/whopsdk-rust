pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WebhooksListQueryRequest {
    /// The unique identifier of the account to list webhooks for.
    #[serde(default)]
    pub account_id: String,
    /// Only return webhooks attached to this app. Omit to list the account's own webhooks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    /// Also return webhooks attached to the account's apps, not just the account's own. Cannot be combined with `app_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_app_webhooks: Option<bool>,
    /// Only return webhooks whose endpoint is currently failing — every delivery since the current failure streak began has been rejected. Clears as soon as a delivery succeeds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_failures: Option<bool>,
    /// Number of results to return from the start of the range.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<i64>,
    /// Return results after this cursor. Use `page_info.end_cursor` from the previous response to fetch the next page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Number of results to return from the end of the range.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last: Option<i64>,
    /// Return results before this cursor. Use `page_info.start_cursor` from the previous response to fetch the previous page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
}

impl WebhooksListQueryRequest {
    pub fn builder() -> WebhooksListQueryRequestBuilder {
        <WebhooksListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebhooksListQueryRequestBuilder {
    account_id: Option<String>,
    app_id: Option<String>,
    include_app_webhooks: Option<bool>,
    has_failures: Option<bool>,
    first: Option<i64>,
    after: Option<String>,
    last: Option<i64>,
    before: Option<String>,
}

impl WebhooksListQueryRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn app_id(mut self, value: impl Into<String>) -> Self {
        self.app_id = Some(value.into());
        self
    }

    pub fn include_app_webhooks(mut self, value: bool) -> Self {
        self.include_app_webhooks = Some(value);
        self
    }

    pub fn has_failures(mut self, value: bool) -> Self {
        self.has_failures = Some(value);
        self
    }

    pub fn first(mut self, value: i64) -> Self {
        self.first = Some(value);
        self
    }

    pub fn after(mut self, value: impl Into<String>) -> Self {
        self.after = Some(value.into());
        self
    }

    pub fn last(mut self, value: i64) -> Self {
        self.last = Some(value);
        self
    }

    pub fn before(mut self, value: impl Into<String>) -> Self {
        self.before = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WebhooksListQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_id`](WebhooksListQueryRequestBuilder::account_id)
    pub fn build(self) -> Result<WebhooksListQueryRequest, BuildError> {
        Ok(WebhooksListQueryRequest {
            account_id: self
                .account_id
                .ok_or_else(|| BuildError::missing_field("account_id"))?,
            app_id: self.app_id,
            include_app_webhooks: self.include_app_webhooks,
            has_failures: self.has_failures,
            first: self.first,
            after: self.after,
            last: self.last,
            before: self.before,
        })
    }
}
