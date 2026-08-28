pub use crate::prelude::*;

/// Query parameters for listRecipients
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListRecipientsQueryRequest {
    /// The account sending the money: a company account ID (`biz_`), or a user ID (`user_`) for that user's own personal balance.
    #[serde(default)]
    pub origin_id: String,
    /// Search anyone on Whop by name or username, plus your own accounts by name or ID. An exact business ID (`biz_`) returns that business first. Omit it to get the team around the balance, the people you follow, and your own accounts. The list is the same whether the balance belongs to a company or to you. Searching from a `biz_` origin additionally requires the member:basic:read scope. A credential scoped to a single company is the exception to the search itself: it only ever sees that company's own people. Complete email addresses return no matches.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// Number of recipients per page. Search queries preserve the dashboard's 20-result maximum.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<i64>,
    /// Cursor to fetch the page after (from page_info.end_cursor).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
}

impl ListRecipientsQueryRequest {
    pub fn builder() -> ListRecipientsQueryRequestBuilder {
        <ListRecipientsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListRecipientsQueryRequestBuilder {
    origin_id: Option<String>,
    query: Option<String>,
    first: Option<i64>,
    after: Option<String>,
}

impl ListRecipientsQueryRequestBuilder {
    pub fn origin_id(mut self, value: impl Into<String>) -> Self {
        self.origin_id = Some(value.into());
        self
    }

    pub fn query(mut self, value: impl Into<String>) -> Self {
        self.query = Some(value.into());
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

    /// Consumes the builder and constructs a [`ListRecipientsQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`origin_id`](ListRecipientsQueryRequestBuilder::origin_id)
    pub fn build(self) -> Result<ListRecipientsQueryRequest, BuildError> {
        Ok(ListRecipientsQueryRequest {
            origin_id: self
                .origin_id
                .ok_or_else(|| BuildError::missing_field("origin_id"))?,
            query: self.query,
            first: self.first,
            after: self.after,
        })
    }
}
