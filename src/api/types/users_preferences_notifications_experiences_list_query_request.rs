pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UsersPreferencesNotificationsExperiencesListQueryRequest {
    /// Number of results to return from the start of the range.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<i64>,
    /// Return results after this cursor. Use `page_info.end_cursor` from the previous response to fetch the next page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
}

impl UsersPreferencesNotificationsExperiencesListQueryRequest {
    pub fn builder() -> UsersPreferencesNotificationsExperiencesListQueryRequestBuilder {
        <UsersPreferencesNotificationsExperiencesListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UsersPreferencesNotificationsExperiencesListQueryRequestBuilder {
    first: Option<i64>,
    after: Option<String>,
}

impl UsersPreferencesNotificationsExperiencesListQueryRequestBuilder {
    pub fn first(mut self, value: i64) -> Self {
        self.first = Some(value);
        self
    }

    pub fn after(mut self, value: impl Into<String>) -> Self {
        self.after = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UsersPreferencesNotificationsExperiencesListQueryRequest`].
    pub fn build(
        self,
    ) -> Result<UsersPreferencesNotificationsExperiencesListQueryRequest, BuildError> {
        Ok(UsersPreferencesNotificationsExperiencesListQueryRequest {
            first: self.first,
            after: self.after,
        })
    }
}
