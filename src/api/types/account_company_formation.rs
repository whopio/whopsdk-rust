pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AccountCompanyFormation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documents: Option<Vec<AccountCompanyFormationDocument>>,
    /// Whether the company's EIN has been issued by the IRS. Present once `status` leaves `draft`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ein_registered: Option<bool>,
    /// Registered company name including the entity ending, for example `Acme, LLC`. Present once `status` leaves `draft`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legal_name: Option<String>,
    /// IRS forms still awaiting a founder's signature, each with a hosted signing URL. Present once `status` leaves `draft`; empty when nothing needs signing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signatures: Option<AccountCompanyFormationSignatures>,
    /// Whether the state formation filing is complete. Present once `status` leaves `draft`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_registered: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<AccountCompanyFormationStatus>,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl AccountCompanyFormation {
    pub fn builder() -> AccountCompanyFormationBuilder {
        <AccountCompanyFormationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AccountCompanyFormationBuilder {
    documents: Option<Vec<AccountCompanyFormationDocument>>,
    ein_registered: Option<bool>,
    legal_name: Option<String>,
    signatures: Option<AccountCompanyFormationSignatures>,
    state_registered: Option<bool>,
    status: Option<AccountCompanyFormationStatus>,
}

impl AccountCompanyFormationBuilder {
    pub fn documents(mut self, value: Vec<AccountCompanyFormationDocument>) -> Self {
        self.documents = Some(value);
        self
    }

    pub fn ein_registered(mut self, value: bool) -> Self {
        self.ein_registered = Some(value);
        self
    }

    pub fn legal_name(mut self, value: impl Into<String>) -> Self {
        self.legal_name = Some(value.into());
        self
    }

    pub fn signatures(mut self, value: AccountCompanyFormationSignatures) -> Self {
        self.signatures = Some(value);
        self
    }

    pub fn state_registered(mut self, value: bool) -> Self {
        self.state_registered = Some(value);
        self
    }

    pub fn status(mut self, value: AccountCompanyFormationStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AccountCompanyFormation`].
    pub fn build(self) -> Result<AccountCompanyFormation, BuildError> {
        Ok(AccountCompanyFormation {
            documents: self.documents,
            ein_registered: self.ein_registered,
            legal_name: self.legal_name,
            signatures: self.signatures,
            state_registered: self.state_registered,
            status: self.status,
            extra: Default::default(),
        })
    }
}
