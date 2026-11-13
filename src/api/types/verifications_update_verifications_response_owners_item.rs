pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateVerificationsResponseOwnersItem {
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
    pub rejected_checks: Vec<UpdateVerificationsResponseOwnersItemRejectedChecksItem>,
    /// The owner's documents and their progress. Empty for an owner added with `verification_id`.
    #[serde(default)]
    pub required_documents: Vec<UpdateVerificationsResponseOwnersItemRequiredDocumentsItem>,
    /// What the person is to the business.
    #[serde(default)]
    pub roles: Vec<UpdateVerificationsResponseOwnersItemRolesItem>,
    /// Where the owner's own verification stands, in the same states as `status`. `action_required` when their documents need sending again.
    pub status: UpdateVerificationsResponseOwnersItemStatus,
}

impl UpdateVerificationsResponseOwnersItem {
    pub fn builder() -> UpdateVerificationsResponseOwnersItemBuilder {
        <UpdateVerificationsResponseOwnersItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateVerificationsResponseOwnersItemBuilder {
    date_of_birth: Option<String>,
    first_name: Option<String>,
    id: Option<String>,
    last_name: Option<String>,
    ownership_percentage: Option<f64>,
    rejected_checks: Option<Vec<UpdateVerificationsResponseOwnersItemRejectedChecksItem>>,
    required_documents: Option<Vec<UpdateVerificationsResponseOwnersItemRequiredDocumentsItem>>,
    roles: Option<Vec<UpdateVerificationsResponseOwnersItemRolesItem>>,
    status: Option<UpdateVerificationsResponseOwnersItemStatus>,
}

impl UpdateVerificationsResponseOwnersItemBuilder {
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
        value: Vec<UpdateVerificationsResponseOwnersItemRejectedChecksItem>,
    ) -> Self {
        self.rejected_checks = Some(value);
        self
    }

    pub fn required_documents(
        mut self,
        value: Vec<UpdateVerificationsResponseOwnersItemRequiredDocumentsItem>,
    ) -> Self {
        self.required_documents = Some(value);
        self
    }

    pub fn roles(mut self, value: Vec<UpdateVerificationsResponseOwnersItemRolesItem>) -> Self {
        self.roles = Some(value);
        self
    }

    pub fn status(mut self, value: UpdateVerificationsResponseOwnersItemStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateVerificationsResponseOwnersItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](UpdateVerificationsResponseOwnersItemBuilder::id)
    /// - [`rejected_checks`](UpdateVerificationsResponseOwnersItemBuilder::rejected_checks)
    /// - [`required_documents`](UpdateVerificationsResponseOwnersItemBuilder::required_documents)
    /// - [`roles`](UpdateVerificationsResponseOwnersItemBuilder::roles)
    /// - [`status`](UpdateVerificationsResponseOwnersItemBuilder::status)
    pub fn build(self) -> Result<UpdateVerificationsResponseOwnersItem, BuildError> {
        Ok(UpdateVerificationsResponseOwnersItem {
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
