pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RetrieveVerificationsResponse {
    /// Accounts this verification was read for, each prefixed `biz_`: the account you listed by, the account or subaccount it belongs to with `include_subaccounts`, or every account you can read that it belongs to when retrieved by ID. Usually one. Empty for a personal verification.
    #[serde(default)]
    pub account_ids: Vec<String>,
    /// Address on the verification profile. `null` when no address is set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<RetrieveVerificationsResponseAddress>,
    /// Legal business name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_name: Option<String>,
    /// Legal entity structure of the business, such as `private_corporation` or `sole_proprietorship`. Supported values vary by country of incorporation — see [Business structures](/developer/verification/business-structures).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_structure: Option<String>,
    /// Two-letter ISO 3166-1 country code, for example `US`, `DE`, or `GB`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// When the verification profile was created, as an ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Formatted as `YYYY-MM-DD`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<String>,
    /// Email address on the verification profile.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// Verification profile ID, prefixed `idpf_`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Date the business was incorporated, formatted as `YYYY-MM-DD`, as on file for the business verification. Business verifications only; `null` until it is sent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incorporation_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<RetrieveVerificationsResponseKind>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Owners of the business and the progress of each one's own verification. Empty for individual verifications.
    #[serde(default)]
    pub owners: Vec<RetrieveVerificationsResponseOwnersItem>,
    /// Phone number on the verification profile.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// State or region where the business is incorporated, as on file for the business verification. Business verifications only; `null` until it is sent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place_of_incorporation: Option<String>,
    /// Number the business was registered under in its place of incorporation, such as a state file number in the US or a Company Number in the UK. Business verifications only; as on file for the business verification, or `null` until it is sent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_number: Option<String>,
    /// What Whop still needs before review can continue — one requirement per entry. Answer with Update Verification; nothing from the response is echoed back. Keys that don't apply are omitted. For a business verification created with `documents` or `owners`, this includes the company details and documents still missing before it can be sent for review; each owner's documents are listed under that owner.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_information: Option<Vec<RetrieveVerificationsResponseRequestedInformationItem>>,
    /// Documents for a document-upload verification and their progress. Present only on individual verifications created by sending `documents`. `pending_upload` documents were not accepted yet — send the full set again with another Create Verification call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_documents: Option<Vec<RetrieveVerificationsResponseRequiredDocumentsItem>>,
    /// Hosted verification session URL for the user to complete identity checks. Expires 7 days after creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_url: Option<String>,
    /// Current verification state. `not_started` before any session exists; `pending` while a session needs the user's input; `processing` while the provider's automated checks run on a fresh submission; `action_required` when `requested_information` needs answers; `manual_review` while information already sent is under review — an audit answer, or a document the payout provider holds — nothing to submit, usually done within 3 business days; `approved` on success; `rejected` on failure. Call Create Verification again to start a new session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<RetrieveVerificationsResponseStatus>,
    /// When the verification profile was last updated, as an ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

impl RetrieveVerificationsResponse {
    pub fn builder() -> RetrieveVerificationsResponseBuilder {
        <RetrieveVerificationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RetrieveVerificationsResponseBuilder {
    account_ids: Option<Vec<String>>,
    address: Option<RetrieveVerificationsResponseAddress>,
    business_name: Option<String>,
    business_structure: Option<String>,
    country: Option<String>,
    created_at: Option<String>,
    date_of_birth: Option<String>,
    email: Option<String>,
    first_name: Option<String>,
    id: Option<String>,
    incorporation_date: Option<String>,
    kind: Option<RetrieveVerificationsResponseKind>,
    last_name: Option<String>,
    owners: Option<Vec<RetrieveVerificationsResponseOwnersItem>>,
    phone: Option<String>,
    place_of_incorporation: Option<String>,
    registration_number: Option<String>,
    requested_information: Option<Vec<RetrieveVerificationsResponseRequestedInformationItem>>,
    required_documents: Option<Vec<RetrieveVerificationsResponseRequiredDocumentsItem>>,
    session_url: Option<String>,
    status: Option<RetrieveVerificationsResponseStatus>,
    updated_at: Option<String>,
}

impl RetrieveVerificationsResponseBuilder {
    pub fn account_ids(mut self, value: Vec<String>) -> Self {
        self.account_ids = Some(value);
        self
    }

    pub fn address(mut self, value: RetrieveVerificationsResponseAddress) -> Self {
        self.address = Some(value);
        self
    }

    pub fn business_name(mut self, value: impl Into<String>) -> Self {
        self.business_name = Some(value.into());
        self
    }

    pub fn business_structure(mut self, value: impl Into<String>) -> Self {
        self.business_structure = Some(value.into());
        self
    }

    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn date_of_birth(mut self, value: impl Into<String>) -> Self {
        self.date_of_birth = Some(value.into());
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
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

    pub fn incorporation_date(mut self, value: impl Into<String>) -> Self {
        self.incorporation_date = Some(value.into());
        self
    }

    pub fn kind(mut self, value: RetrieveVerificationsResponseKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn last_name(mut self, value: impl Into<String>) -> Self {
        self.last_name = Some(value.into());
        self
    }

    pub fn owners(mut self, value: Vec<RetrieveVerificationsResponseOwnersItem>) -> Self {
        self.owners = Some(value);
        self
    }

    pub fn phone(mut self, value: impl Into<String>) -> Self {
        self.phone = Some(value.into());
        self
    }

    pub fn place_of_incorporation(mut self, value: impl Into<String>) -> Self {
        self.place_of_incorporation = Some(value.into());
        self
    }

    pub fn registration_number(mut self, value: impl Into<String>) -> Self {
        self.registration_number = Some(value.into());
        self
    }

    pub fn requested_information(
        mut self,
        value: Vec<RetrieveVerificationsResponseRequestedInformationItem>,
    ) -> Self {
        self.requested_information = Some(value);
        self
    }

    pub fn required_documents(
        mut self,
        value: Vec<RetrieveVerificationsResponseRequiredDocumentsItem>,
    ) -> Self {
        self.required_documents = Some(value);
        self
    }

    pub fn session_url(mut self, value: impl Into<String>) -> Self {
        self.session_url = Some(value.into());
        self
    }

    pub fn status(mut self, value: RetrieveVerificationsResponseStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RetrieveVerificationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_ids`](RetrieveVerificationsResponseBuilder::account_ids)
    /// - [`owners`](RetrieveVerificationsResponseBuilder::owners)
    pub fn build(self) -> Result<RetrieveVerificationsResponse, BuildError> {
        Ok(RetrieveVerificationsResponse {
            account_ids: self
                .account_ids
                .ok_or_else(|| BuildError::missing_field("account_ids"))?,
            address: self.address,
            business_name: self.business_name,
            business_structure: self.business_structure,
            country: self.country,
            created_at: self.created_at,
            date_of_birth: self.date_of_birth,
            email: self.email,
            first_name: self.first_name,
            id: self.id,
            incorporation_date: self.incorporation_date,
            kind: self.kind,
            last_name: self.last_name,
            owners: self
                .owners
                .ok_or_else(|| BuildError::missing_field("owners"))?,
            phone: self.phone,
            place_of_incorporation: self.place_of_incorporation,
            registration_number: self.registration_number,
            requested_information: self.requested_information,
            required_documents: self.required_documents,
            session_url: self.session_url,
            status: self.status,
            updated_at: self.updated_at,
        })
    }
}
