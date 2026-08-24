pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct LedgerActivityResourceLogoUrl {
    /// Account ID.
    #[serde(default)]
    pub id: String,
    /// Account logo URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_url: Option<String>,
    pub object: LedgerActivityResourceLogoUrlObject,
    /// Account route.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route: Option<String>,
    /// Account display name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl LedgerActivityResourceLogoUrl {
    pub fn builder() -> LedgerActivityResourceLogoUrlBuilder {
        <LedgerActivityResourceLogoUrlBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LedgerActivityResourceLogoUrlBuilder {
    id: Option<String>,
    logo_url: Option<String>,
    object: Option<LedgerActivityResourceLogoUrlObject>,
    route: Option<String>,
    title: Option<String>,
}

impl LedgerActivityResourceLogoUrlBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn logo_url(mut self, value: impl Into<String>) -> Self {
        self.logo_url = Some(value.into());
        self
    }

    pub fn object(mut self, value: LedgerActivityResourceLogoUrlObject) -> Self {
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

    /// Consumes the builder and constructs a [`LedgerActivityResourceLogoUrl`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](LedgerActivityResourceLogoUrlBuilder::id)
    /// - [`object`](LedgerActivityResourceLogoUrlBuilder::object)
    pub fn build(self) -> Result<LedgerActivityResourceLogoUrl, BuildError> {
        Ok(LedgerActivityResourceLogoUrl {
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
