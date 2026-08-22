pub use crate::prelude::*;

/// A temporary, time-limited URL that grants a user access to an external account management page.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AccountLink {
    /// The timestamp after which this account link URL is no longer valid.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub expires_at: DateTime<FixedOffset>,
    /// The temporary URL to redirect the user to for account access. Expires at the time specified by expires_at.
    #[serde(default)]
    pub url: String,
}

impl AccountLink {
    pub fn builder() -> AccountLinkBuilder {
        <AccountLinkBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AccountLinkBuilder {
    expires_at: Option<DateTime<FixedOffset>>,
    url: Option<String>,
}

impl AccountLinkBuilder {
    pub fn expires_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.expires_at = Some(value);
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AccountLink`].
    /// This method will fail if any of the following fields are not set:
    /// - [`expires_at`](AccountLinkBuilder::expires_at)
    /// - [`url`](AccountLinkBuilder::url)
    pub fn build(self) -> Result<AccountLink, BuildError> {
        Ok(AccountLink {
            expires_at: self
                .expires_at
                .ok_or_else(|| BuildError::missing_field("expires_at"))?,
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
        })
    }
}
