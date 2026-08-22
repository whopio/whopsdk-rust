pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AccountStorePageConfig {
    /// Accent color used on the account store page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accent_color: Option<AccountStorePageConfigAccentColor>,
    /// Layout used on the account store page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layout: Option<AccountStorePageConfigLayout>,
    /// Profile presentation used on the account store page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_variant: Option<AccountStorePageConfigProfileVariant>,
    /// Whether the account store page shows a Whop affiliate link.
    #[serde(default)]
    pub whop_affiliate_link: bool,
}

impl AccountStorePageConfig {
    pub fn builder() -> AccountStorePageConfigBuilder {
        <AccountStorePageConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AccountStorePageConfigBuilder {
    accent_color: Option<AccountStorePageConfigAccentColor>,
    layout: Option<AccountStorePageConfigLayout>,
    profile_variant: Option<AccountStorePageConfigProfileVariant>,
    whop_affiliate_link: Option<bool>,
}

impl AccountStorePageConfigBuilder {
    pub fn accent_color(mut self, value: AccountStorePageConfigAccentColor) -> Self {
        self.accent_color = Some(value);
        self
    }

    pub fn layout(mut self, value: AccountStorePageConfigLayout) -> Self {
        self.layout = Some(value);
        self
    }

    pub fn profile_variant(mut self, value: AccountStorePageConfigProfileVariant) -> Self {
        self.profile_variant = Some(value);
        self
    }

    pub fn whop_affiliate_link(mut self, value: bool) -> Self {
        self.whop_affiliate_link = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AccountStorePageConfig`].
    /// This method will fail if any of the following fields are not set:
    /// - [`whop_affiliate_link`](AccountStorePageConfigBuilder::whop_affiliate_link)
    pub fn build(self) -> Result<AccountStorePageConfig, BuildError> {
        Ok(AccountStorePageConfig {
            accent_color: self.accent_color,
            layout: self.layout,
            profile_variant: self.profile_variant,
            whop_affiliate_link: self
                .whop_affiliate_link
                .ok_or_else(|| BuildError::missing_field("whop_affiliate_link"))?,
        })
    }
}
