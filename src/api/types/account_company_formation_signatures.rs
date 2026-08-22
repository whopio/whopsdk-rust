pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AccountCompanyFormationSignatures {
    /// Signature state for IRS Form 8821, the tax information authorization. Present only while the form still needs the founder's action.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub form8821: Option<AccountCompanyFormationSignatureRequest>,
    /// Signature state for IRS Form SS-4, the EIN application. Present only while the form still needs the founder's action.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ss4: Option<AccountCompanyFormationSignatureRequest>,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl AccountCompanyFormationSignatures {
    pub fn builder() -> AccountCompanyFormationSignaturesBuilder {
        <AccountCompanyFormationSignaturesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AccountCompanyFormationSignaturesBuilder {
    form8821: Option<AccountCompanyFormationSignatureRequest>,
    ss4: Option<AccountCompanyFormationSignatureRequest>,
}

impl AccountCompanyFormationSignaturesBuilder {
    pub fn form8821(mut self, value: AccountCompanyFormationSignatureRequest) -> Self {
        self.form8821 = Some(value);
        self
    }

    pub fn ss4(mut self, value: AccountCompanyFormationSignatureRequest) -> Self {
        self.ss4 = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AccountCompanyFormationSignatures`].
    pub fn build(self) -> Result<AccountCompanyFormationSignatures, BuildError> {
        Ok(AccountCompanyFormationSignatures {
            form8821: self.form8821,
            ss4: self.ss4,
            extra: Default::default(),
        })
    }
}
