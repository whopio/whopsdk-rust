pub use crate::prelude::*;

/// The company that the dispute was filed against.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DisputeListItemCompany {
    /// The unique identifier for the company.
    #[serde(default)]
    pub id: String,
    /// The written name of the company.
    #[serde(default)]
    pub title: String,
}

impl DisputeListItemCompany {
    pub fn builder() -> DisputeListItemCompanyBuilder {
        <DisputeListItemCompanyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DisputeListItemCompanyBuilder {
    id: Option<String>,
    title: Option<String>,
}

impl DisputeListItemCompanyBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DisputeListItemCompany`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](DisputeListItemCompanyBuilder::id)
    /// - [`title`](DisputeListItemCompanyBuilder::title)
    pub fn build(self) -> Result<DisputeListItemCompany, BuildError> {
        Ok(DisputeListItemCompany {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            title: self
                .title
                .ok_or_else(|| BuildError::missing_field("title"))?,
        })
    }
}
