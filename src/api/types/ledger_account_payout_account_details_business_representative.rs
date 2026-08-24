pub use crate::prelude::*;

/// The business representative for this payout account
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LedgerAccountPayoutAccountDetailsBusinessRepresentative {
    /// The date of birth of the business representative in ISO 8601 format (YYYY-MM-DD).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<String>,
    /// The first name of the business representative.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// The last name of the business representative.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// The middle name of the business representative.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub middle_name: Option<String>,
}

impl LedgerAccountPayoutAccountDetailsBusinessRepresentative {
    pub fn builder() -> LedgerAccountPayoutAccountDetailsBusinessRepresentativeBuilder {
        <LedgerAccountPayoutAccountDetailsBusinessRepresentativeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LedgerAccountPayoutAccountDetailsBusinessRepresentativeBuilder {
    date_of_birth: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
    middle_name: Option<String>,
}

impl LedgerAccountPayoutAccountDetailsBusinessRepresentativeBuilder {
    pub fn date_of_birth(mut self, value: impl Into<String>) -> Self {
        self.date_of_birth = Some(value.into());
        self
    }

    pub fn first_name(mut self, value: impl Into<String>) -> Self {
        self.first_name = Some(value.into());
        self
    }

    pub fn last_name(mut self, value: impl Into<String>) -> Self {
        self.last_name = Some(value.into());
        self
    }

    pub fn middle_name(mut self, value: impl Into<String>) -> Self {
        self.middle_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`LedgerAccountPayoutAccountDetailsBusinessRepresentative`].
    pub fn build(
        self,
    ) -> Result<LedgerAccountPayoutAccountDetailsBusinessRepresentative, BuildError> {
        Ok(LedgerAccountPayoutAccountDetailsBusinessRepresentative {
            date_of_birth: self.date_of_birth,
            first_name: self.first_name,
            last_name: self.last_name,
            middle_name: self.middle_name,
        })
    }
}
