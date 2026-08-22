pub use crate::prelude::*;

/// The company this membership belongs to.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MembershipListItemCompany {
    /// The unique identifier for the company.
    #[serde(default)]
    pub id: String,
    /// The display name of the company shown to customers.
    #[serde(default)]
    pub title: String,
}

impl MembershipListItemCompany {
    pub fn builder() -> MembershipListItemCompanyBuilder {
        <MembershipListItemCompanyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MembershipListItemCompanyBuilder {
    id: Option<String>,
    title: Option<String>,
}

impl MembershipListItemCompanyBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`MembershipListItemCompany`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](MembershipListItemCompanyBuilder::id)
    /// - [`title`](MembershipListItemCompanyBuilder::title)
    pub fn build(self) -> Result<MembershipListItemCompany, BuildError> {
        Ok(MembershipListItemCompany {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            title: self
                .title
                .ok_or_else(|| BuildError::missing_field("title"))?,
        })
    }
}
