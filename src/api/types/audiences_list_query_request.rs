pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AudiencesListQueryRequest {
    /// Account ID, prefixed `biz_`.
    #[serde(default)]
    pub account_id: String,
    /// Audience ID, prefixed `adaud_`, used to filter the response to one audience.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audience_id: Option<String>,
    /// Filter by audience type: `custom` (uploaded lists) or `lookalike`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audience_type: Option<ListAudiencesRequestAudienceType>,
    /// Filter by member source: `csv_upload` (uploaded lists) or `people_filter` (automatic audiences built from saved People filters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<ListAudiencesRequestSourceType>,
    /// Number of audiences to return. Defaults to 20; maximum 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<i64>,
    /// Cursor for the next page of audiences.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
}

impl AudiencesListQueryRequest {
    pub fn builder() -> AudiencesListQueryRequestBuilder {
        <AudiencesListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AudiencesListQueryRequestBuilder {
    account_id: Option<String>,
    audience_id: Option<String>,
    audience_type: Option<ListAudiencesRequestAudienceType>,
    source_type: Option<ListAudiencesRequestSourceType>,
    first: Option<i64>,
    after: Option<String>,
}

impl AudiencesListQueryRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn audience_id(mut self, value: impl Into<String>) -> Self {
        self.audience_id = Some(value.into());
        self
    }

    pub fn audience_type(mut self, value: ListAudiencesRequestAudienceType) -> Self {
        self.audience_type = Some(value);
        self
    }

    pub fn source_type(mut self, value: ListAudiencesRequestSourceType) -> Self {
        self.source_type = Some(value);
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

    /// Consumes the builder and constructs a [`AudiencesListQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_id`](AudiencesListQueryRequestBuilder::account_id)
    pub fn build(self) -> Result<AudiencesListQueryRequest, BuildError> {
        Ok(AudiencesListQueryRequest {
            account_id: self
                .account_id
                .ok_or_else(|| BuildError::missing_field("account_id"))?,
            audience_id: self.audience_id,
            audience_type: self.audience_type,
            source_type: self.source_type,
            first: self.first,
            after: self.after,
        })
    }
}
