pub use crate::prelude::*;

/// Account store page display configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateAccountsRequestStorePageConfig {
    /// Accent color used on the account store page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accent_color: Option<UpdateAccountsRequestStorePageConfigAccentColor>,
    /// Layout used on the account store page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layout: Option<UpdateAccountsRequestStorePageConfigLayout>,
    /// Profile presentation used on the account store page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_variant: Option<UpdateAccountsRequestStorePageConfigProfileVariant>,
    /// Whether the account store page shows a Whop affiliate link.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whop_affiliate_link: Option<bool>,
}

impl UpdateAccountsRequestStorePageConfig {
    pub fn builder() -> UpdateAccountsRequestStorePageConfigBuilder {
        <UpdateAccountsRequestStorePageConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateAccountsRequestStorePageConfigBuilder {
    accent_color: Option<UpdateAccountsRequestStorePageConfigAccentColor>,
    layout: Option<UpdateAccountsRequestStorePageConfigLayout>,
    profile_variant: Option<UpdateAccountsRequestStorePageConfigProfileVariant>,
    whop_affiliate_link: Option<bool>,
}

impl UpdateAccountsRequestStorePageConfigBuilder {
    pub fn accent_color(mut self, value: UpdateAccountsRequestStorePageConfigAccentColor) -> Self {
        self.accent_color = Some(value);
        self
    }

    pub fn layout(mut self, value: UpdateAccountsRequestStorePageConfigLayout) -> Self {
        self.layout = Some(value);
        self
    }

    pub fn profile_variant(
        mut self,
        value: UpdateAccountsRequestStorePageConfigProfileVariant,
    ) -> Self {
        self.profile_variant = Some(value);
        self
    }

    pub fn whop_affiliate_link(mut self, value: bool) -> Self {
        self.whop_affiliate_link = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateAccountsRequestStorePageConfig`].
    pub fn build(self) -> Result<UpdateAccountsRequestStorePageConfig, BuildError> {
        Ok(UpdateAccountsRequestStorePageConfig {
            accent_color: self.accent_color,
            layout: self.layout,
            profile_variant: self.profile_variant,
            whop_affiliate_link: self.whop_affiliate_link,
        })
    }
}
