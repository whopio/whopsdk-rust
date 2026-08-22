pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MembershipAccount {
    /// Account ID, prefixed `biz_`.
    #[serde(default)]
    pub id: String,
    /// Account logo image URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_url: Option<String>,
    /// Account public route identifier — the `whop.com/{route}` storefront path.
    #[serde(default)]
    pub route: String,
    /// Account display name.
    #[serde(default)]
    pub title: String,
}

impl MembershipAccount {
    pub fn builder() -> MembershipAccountBuilder {
        <MembershipAccountBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MembershipAccountBuilder {
    id: Option<String>,
    logo_url: Option<String>,
    route: Option<String>,
    title: Option<String>,
}

impl MembershipAccountBuilder {
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

    /// Consumes the builder and constructs a [`MembershipAccount`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](MembershipAccountBuilder::id)
    /// - [`route`](MembershipAccountBuilder::route)
    /// - [`title`](MembershipAccountBuilder::title)
    pub fn build(self) -> Result<MembershipAccount, BuildError> {
        Ok(MembershipAccount {
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
