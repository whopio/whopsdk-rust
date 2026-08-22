pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateAdsRequestSocialAccountsItem {
    /// Social account ID, prefixed `sacc_`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl CreateAdsRequestSocialAccountsItem {
    pub fn builder() -> CreateAdsRequestSocialAccountsItemBuilder {
        <CreateAdsRequestSocialAccountsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateAdsRequestSocialAccountsItemBuilder {
    id: Option<String>,
}

impl CreateAdsRequestSocialAccountsItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateAdsRequestSocialAccountsItem`].
    pub fn build(self) -> Result<CreateAdsRequestSocialAccountsItem, BuildError> {
        Ok(CreateAdsRequestSocialAccountsItem { id: self.id })
    }
}
