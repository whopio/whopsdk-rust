pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateVerificationsResponseOwnersItem {
    /// The owner's date of birth on file, formatted as `YYYY-MM-DD`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// Owner ID, prefixed `own_`. Send it in Update Verification's `owners` to change or remove the owner, including one added in the hosted session.
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Share of the business the person owns, from 0 to 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub ownership_percentage: Option<f64>,
    /// What the provider rejected when the owner's verification was sent back: `details` (their name, date of birth, or other details don't match their ID), `identity_document`, or `selfie`. Fix only these with Update Verification. Empty otherwise.
    #[serde(default)]
    pub rejected_checks: Vec<CreateVerificationsResponseOwnersItemRejectedChecksItem>,
    /// The owner's documents and their progress. Empty for an owner added with `verification_id`.
    #[serde(default)]
    pub required_documents: Vec<CreateVerificationsResponseOwnersItemRequiredDocumentsItem>,
    /// What the person is to the business.
    #[serde(default)]
    pub roles: Vec<CreateVerificationsResponseOwnersItemRolesItem>,
    /// Where the owner's own verification stands, in the same states as `status`. `action_required` when their documents need sending again.
    pub status: CreateVerificationsResponseOwnersItemStatus,
}

impl CreateVerificationsResponseOwnersItem {
    pub fn builder() -> CreateVerificationsResponseOwnersItemBuilder {
        <CreateVerificationsResponseOwnersItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateVerificationsResponseOwnersItemBuilder {
    date_of_birth: Option<String>,
    first_name: Option<String>,
    id: Option<String>,
    last_name: Option<String>,
    ownership_percentage: Option<f64>,
    rejected_checks: Option<Vec<CreateVerificationsResponseOwnersItemRejectedChecksItem>>,
    required_documents: Option<Vec<CreateVerificationsResponseOwnersItemRequiredDocumentsItem>>,
    roles: Option<Vec<CreateVerificationsResponseOwnersItemRolesItem>>,
    status: Option<CreateVerificationsResponseOwnersItemStatus>,
}

impl CreateVerificationsResponseOwnersItemBuilder {
    pub fn date_of_birth(mut self, value: impl Into<String>) -> Self {
        self.date_of_birth = Some(value.into());
        self
    }

    pub fn first_name(mut self, value: impl Into<String>) -> Self {
        self.first_name = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn last_name(mut self, value: impl Into<String>) -> Self {
        self.last_name = Some(value.into());
        self
    }

    pub fn ownership_percentage(mut self, value: f64) -> Self {
        self.ownership_percentage = Some(value);
        self
    }

    pub fn rejected_checks(
        mut self,
        value: Vec<CreateVerificationsResponseOwnersItemRejectedChecksItem>,
    ) -> Self {
        self.rejected_checks = Some(value);
        self
    }

    pub fn required_documents(
        mut self,
        value: Vec<CreateVerificationsResponseOwnersItemRequiredDocumentsItem>,
    ) -> Self {
        self.required_documents = Some(value);
        self
    }

    pub fn roles(mut self, value: Vec<CreateVerificationsResponseOwnersItemRolesItem>) -> Self {
        self.roles = Some(value);
        self
    }

    pub fn status(mut self, value: CreateVerificationsResponseOwnersItemStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateVerificationsResponseOwnersItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](CreateVerificationsResponseOwnersItemBuilder::id)
    /// - [`rejected_checks`](CreateVerificationsResponseOwnersItemBuilder::rejected_checks)
    /// - [`required_documents`](CreateVerificationsResponseOwnersItemBuilder::required_documents)
    /// - [`roles`](CreateVerificationsResponseOwnersItemBuilder::roles)
    /// - [`status`](CreateVerificationsResponseOwnersItemBuilder::status)
    pub fn build(self) -> Result<CreateVerificationsResponseOwnersItem, BuildError> {
        Ok(CreateVerificationsResponseOwnersItem {
            date_of_birth: self.date_of_birth,
            first_name: self.first_name,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            last_name: self.last_name,
            ownership_percentage: self.ownership_percentage,
            rejected_checks: self
                .rejected_checks
                .ok_or_else(|| BuildError::missing_field("rejected_checks"))?,
            required_documents: self
                .required_documents
                .ok_or_else(|| BuildError::missing_field("required_documents"))?,
            roles: self
                .roles
                .ok_or_else(|| BuildError::missing_field("roles"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
