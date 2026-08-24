pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostsSocialAccountsResponsePageInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_cursor: Option<String>,
    #[serde(default)]
    pub has_next_page: bool,
}

impl PostsSocialAccountsResponsePageInfo {
    pub fn builder() -> PostsSocialAccountsResponsePageInfoBuilder {
        <PostsSocialAccountsResponsePageInfoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostsSocialAccountsResponsePageInfoBuilder {
    end_cursor: Option<String>,
    has_next_page: Option<bool>,
}

impl PostsSocialAccountsResponsePageInfoBuilder {
    pub fn end_cursor(mut self, value: impl Into<String>) -> Self {
        self.end_cursor = Some(value.into());
        self
    }

    pub fn has_next_page(mut self, value: bool) -> Self {
        self.has_next_page = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostsSocialAccountsResponsePageInfo`].
    /// This method will fail if any of the following fields are not set:
    /// - [`has_next_page`](PostsSocialAccountsResponsePageInfoBuilder::has_next_page)
    pub fn build(self) -> Result<PostsSocialAccountsResponsePageInfo, BuildError> {
        Ok(PostsSocialAccountsResponsePageInfo {
            end_cursor: self.end_cursor,
            has_next_page: self
                .has_next_page
                .ok_or_else(|| BuildError::missing_field("has_next_page"))?,
        })
    }
}
