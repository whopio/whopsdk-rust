pub use crate::prelude::*;

/// The partner business whose team member made this referral, the one its commission is paid to. Null for a solo partner's referral.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RetrieveBusinessesResponseReferringBusiness {
    /// Referred account ID.
    #[serde(default)]
    pub id: String,
    /// Referred account logo URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_url: Option<String>,
    /// Referred account route.
    #[serde(default)]
    pub route: String,
    /// Referred account display name.
    #[serde(default)]
    pub title: String,
}

impl RetrieveBusinessesResponseReferringBusiness {
    pub fn builder() -> RetrieveBusinessesResponseReferringBusinessBuilder {
        <RetrieveBusinessesResponseReferringBusinessBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RetrieveBusinessesResponseReferringBusinessBuilder {
    id: Option<String>,
    logo_url: Option<String>,
    route: Option<String>,
    title: Option<String>,
}

impl RetrieveBusinessesResponseReferringBusinessBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn logo_url(mut self, value: impl Into<String>) -> Self {
        self.logo_url = Some(value.into());
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

    /// Consumes the builder and constructs a [`RetrieveBusinessesResponseReferringBusiness`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](RetrieveBusinessesResponseReferringBusinessBuilder::id)
    /// - [`route`](RetrieveBusinessesResponseReferringBusinessBuilder::route)
    /// - [`title`](RetrieveBusinessesResponseReferringBusinessBuilder::title)
    pub fn build(self) -> Result<RetrieveBusinessesResponseReferringBusiness, BuildError> {
        Ok(RetrieveBusinessesResponseReferringBusiness {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            logo_url: self.logo_url,
            route: self
                .route
                .ok_or_else(|| BuildError::missing_field("route"))?,
            title: self
                .title
                .ok_or_else(|| BuildError::missing_field("title"))?,
        })
    }
}
