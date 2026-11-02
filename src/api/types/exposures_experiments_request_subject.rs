pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ExposuresExperimentsRequestSubject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anonymous_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

impl ExposuresExperimentsRequestSubject {
    pub fn builder() -> ExposuresExperimentsRequestSubjectBuilder {
        <ExposuresExperimentsRequestSubjectBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExposuresExperimentsRequestSubjectBuilder {
    account_id: Option<String>,
    anonymous_id: Option<String>,
    user_id: Option<String>,
}

impl ExposuresExperimentsRequestSubjectBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn anonymous_id(mut self, value: impl Into<String>) -> Self {
        self.anonymous_id = Some(value.into());
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ExposuresExperimentsRequestSubject`].
    pub fn build(self) -> Result<ExposuresExperimentsRequestSubject, BuildError> {
        Ok(ExposuresExperimentsRequestSubject {
            account_id: self.account_id,
            anonymous_id: self.anonymous_id,
            user_id: self.user_id,
        })
    }
}
