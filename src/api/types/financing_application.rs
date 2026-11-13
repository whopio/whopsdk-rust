pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FinancingApplication {
    /// The ID of the merchant account applying for payment financing, starting with biz_.
    #[serde(default)]
    pub account_id: String,
    /// When the application was created, as an ISO 8601 timestamp.
    #[serde(default)]
    pub created_at: String,
    /// Public review feedback when more information is requested or the application is denied. Internal notes are never included.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feedback: Option<String>,
    /// The financing application ID, starting with inrq_.
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requirements: Option<Vec<FinancingRequirement>>,
    /// The application's review state. Approval does not by itself indicate that financing payment methods are enabled on the account.
    pub status: FinancingApplicationStatus,
    /// Current terms to present before submitting. Present on individual application responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms: Option<FinancingTerms>,
    /// When the application was last updated, as an ISO 8601 timestamp.
    #[serde(default)]
    pub updated_at: String,
}

impl FinancingApplication {
    pub fn builder() -> FinancingApplicationBuilder {
        <FinancingApplicationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FinancingApplicationBuilder {
    account_id: Option<String>,
    created_at: Option<String>,
    feedback: Option<String>,
    id: Option<String>,
    requirements: Option<Vec<FinancingRequirement>>,
    status: Option<FinancingApplicationStatus>,
    terms: Option<FinancingTerms>,
    updated_at: Option<String>,
}

impl FinancingApplicationBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn feedback(mut self, value: impl Into<String>) -> Self {
        self.feedback = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn requirements(mut self, value: Vec<FinancingRequirement>) -> Self {
        self.requirements = Some(value);
        self
    }

    pub fn status(mut self, value: FinancingApplicationStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn terms(mut self, value: FinancingTerms) -> Self {
        self.terms = Some(value);
        self
    }

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`FinancingApplication`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_id`](FinancingApplicationBuilder::account_id)
    /// - [`created_at`](FinancingApplicationBuilder::created_at)
    /// - [`id`](FinancingApplicationBuilder::id)
    /// - [`status`](FinancingApplicationBuilder::status)
    /// - [`updated_at`](FinancingApplicationBuilder::updated_at)
    pub fn build(self) -> Result<FinancingApplication, BuildError> {
        Ok(FinancingApplication {
            account_id: self
                .account_id
                .ok_or_else(|| BuildError::missing_field("account_id"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            feedback: self.feedback,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            requirements: self.requirements,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            terms: self.terms,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
