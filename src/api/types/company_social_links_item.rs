pub use crate::prelude::*;

/// A social link attached to a resource on the site.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CompanySocialLinksItem {
    /// The unique identifier for the social link.
    #[serde(default)]
    pub id: String,
    /// The URL of the social media profile or external link.
    #[serde(default)]
    pub url: String,
    /// The website
    pub website: SocialLinkWebsites,
}

impl CompanySocialLinksItem {
    pub fn builder() -> CompanySocialLinksItemBuilder {
        <CompanySocialLinksItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CompanySocialLinksItemBuilder {
    id: Option<String>,
    url: Option<String>,
    website: Option<SocialLinkWebsites>,
}

impl CompanySocialLinksItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    pub fn website(mut self, value: SocialLinkWebsites) -> Self {
        self.website = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CompanySocialLinksItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](CompanySocialLinksItemBuilder::id)
    /// - [`url`](CompanySocialLinksItemBuilder::url)
    /// - [`website`](CompanySocialLinksItemBuilder::website)
    pub fn build(self) -> Result<CompanySocialLinksItem, BuildError> {
        Ok(CompanySocialLinksItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
            website: self
                .website
                .ok_or_else(|| BuildError::missing_field("website"))?,
        })
    }
}
