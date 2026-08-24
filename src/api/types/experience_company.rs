pub use crate::prelude::*;

/// The company that owns this experience.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ExperienceCompany {
    /// The unique identifier for the company.
    #[serde(default)]
    pub id: String,
    /// URL slug for the account's store page, e.g. `pickaxe` in whop.com/pickaxe.
    #[serde(default)]
    pub route: String,
    /// The display name of the company shown to customers.
    #[serde(default)]
    pub title: String,
}

impl ExperienceCompany {
    pub fn builder() -> ExperienceCompanyBuilder {
        <ExperienceCompanyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExperienceCompanyBuilder {
    id: Option<String>,
    route: Option<String>,
    title: Option<String>,
}

impl ExperienceCompanyBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
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

    /// Consumes the builder and constructs a [`ExperienceCompany`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ExperienceCompanyBuilder::id)
    /// - [`route`](ExperienceCompanyBuilder::route)
    /// - [`title`](ExperienceCompanyBuilder::title)
    pub fn build(self) -> Result<ExperienceCompany, BuildError> {
        Ok(ExperienceCompany {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            route: self
                .route
                .ok_or_else(|| BuildError::missing_field("route"))?,
            title: self
                .title
                .ok_or_else(|| BuildError::missing_field("title"))?,
        })
    }
}
