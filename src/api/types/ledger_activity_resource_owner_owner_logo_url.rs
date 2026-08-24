pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct LedgerActivityResourceOwnerOwnerLogoUrl {
    /// Account ID.
    #[serde(default)]
    pub id: String,
    /// Account logo URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_url: Option<String>,
    pub object: LedgerActivityResourceOwnerOwnerLogoUrlObject,
    /// Account route.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route: Option<String>,
    /// Account display name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl LedgerActivityResourceOwnerOwnerLogoUrl {
    pub fn builder() -> LedgerActivityResourceOwnerOwnerLogoUrlBuilder {
        <LedgerActivityResourceOwnerOwnerLogoUrlBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LedgerActivityResourceOwnerOwnerLogoUrlBuilder {
    id: Option<String>,
    logo_url: Option<String>,
    object: Option<LedgerActivityResourceOwnerOwnerLogoUrlObject>,
    route: Option<String>,
    title: Option<String>,
}

impl LedgerActivityResourceOwnerOwnerLogoUrlBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn logo_url(mut self, value: impl Into<String>) -> Self {
        self.logo_url = Some(value.into());
        self
    }

    pub fn object(mut self, value: LedgerActivityResourceOwnerOwnerLogoUrlObject) -> Self {
        self.object = Some(value);
        self
    }

    pub fn route(mut self, value: impl Into<String>) -> Self {
        self.route = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`LedgerActivityResourceOwnerOwnerLogoUrl`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](LedgerActivityResourceOwnerOwnerLogoUrlBuilder::id)
    /// - [`object`](LedgerActivityResourceOwnerOwnerLogoUrlBuilder::object)
    pub fn build(self) -> Result<LedgerActivityResourceOwnerOwnerLogoUrl, BuildError> {
        Ok(LedgerActivityResourceOwnerOwnerLogoUrl {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            logo_url: self.logo_url,
            object: self
                .object
                .ok_or_else(|| BuildError::missing_field("object"))?,
            route: self.route,
            title: self.title,
        })
    }
}
