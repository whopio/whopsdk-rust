pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateAdsRequestSocialAccountsItem {
    /// Social account ID, prefixed `sacc_`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl UpdateAdsRequestSocialAccountsItem {
    pub fn builder() -> UpdateAdsRequestSocialAccountsItemBuilder {
        <UpdateAdsRequestSocialAccountsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateAdsRequestSocialAccountsItemBuilder {
    id: Option<String>,
}

impl UpdateAdsRequestSocialAccountsItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateAdsRequestSocialAccountsItem`].
    pub fn build(self) -> Result<UpdateAdsRequestSocialAccountsItem, BuildError> {
        Ok(UpdateAdsRequestSocialAccountsItem { id: self.id })
    }
}
