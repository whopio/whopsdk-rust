pub use crate::prelude::*;

/// Input for creating a social link for a company
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct UpdateCompaniesRequestSocialLinksItem {
    /// The custom image for the social link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<UpdateCompaniesRequestSocialLinksItemImage>,
    /// The order of the social link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
    /// The title of the social link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The URL of the social link
    #[serde(default)]
    pub url: String,
    /// The website this link is for
    pub website: SocialLinkWebsites,
    /// The order of the website social link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website_order: Option<String>,
}

impl UpdateCompaniesRequestSocialLinksItem {
    pub fn builder() -> UpdateCompaniesRequestSocialLinksItemBuilder {
        <UpdateCompaniesRequestSocialLinksItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateCompaniesRequestSocialLinksItemBuilder {
    image: Option<UpdateCompaniesRequestSocialLinksItemImage>,
    order: Option<String>,
    title: Option<String>,
    url: Option<String>,
    website: Option<SocialLinkWebsites>,
    website_order: Option<String>,
}

impl UpdateCompaniesRequestSocialLinksItemBuilder {
    pub fn image(mut self, value: UpdateCompaniesRequestSocialLinksItemImage) -> Self {
        self.image = Some(value);
        self
    }

    pub fn order(mut self, value: impl Into<String>) -> Self {
        self.order = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
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

    pub fn website_order(mut self, value: impl Into<String>) -> Self {
        self.website_order = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateCompaniesRequestSocialLinksItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`url`](UpdateCompaniesRequestSocialLinksItemBuilder::url)
    /// - [`website`](UpdateCompaniesRequestSocialLinksItemBuilder::website)
    pub fn build(self) -> Result<UpdateCompaniesRequestSocialLinksItem, BuildError> {
        Ok(UpdateCompaniesRequestSocialLinksItem {
            image: self.image,
            order: self.order,
            title: self.title,
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
            website: self
                .website
                .ok_or_else(|| BuildError::missing_field("website"))?,
            website_order: self.website_order,
        })
    }
}
