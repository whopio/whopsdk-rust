pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RetrieveVerificationsResponse {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<RetrieveVerificationsResponseKind>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Phone number on the verification profile.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// What Whop still needs before review can continue — one requirement per entry. Answer with Update Verification; nothing from the response is echoed back. Keys that don't apply are omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_information: Option<Vec<RetrieveVerificationsResponseRequestedInformationItem>>,
    /// Documents for a document-upload verification and their progress. Present only on verifications created by sending `documents`. `pending_upload` documents were not accepted yet — send the full set again with another Create Verification call.
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
    address: Option<RetrieveVerificationsResponseAddress>,
    business_name: Option<String>,
    business_structure: Option<String>,
    country: Option<String>,
    created_at: Option<String>,
    date_of_birth: Option<String>,
    email: Option<String>,
    first_name: Option<String>,
    id: Option<String>,
    kind: Option<RetrieveVerificationsResponseKind>,
    last_name: Option<String>,
    phone: Option<String>,
    requested_information: Option<Vec<RetrieveVerificationsResponseRequestedInformationItem>>,
    required_documents: Option<Vec<RetrieveVerificationsResponseRequiredDocumentsItem>>,
    session_url: Option<String>,
    status: Option<RetrieveVerificationsResponseStatus>,
    updated_at: Option<String>,
}

impl RetrieveVerificationsResponseBuilder {
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

    pub fn kind(mut self, value: RetrieveVerificationsResponseKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn last_name(mut self, value: impl Into<String>) -> Self {
        self.last_name = Some(value.into());
        self
    }

    pub fn phone(mut self, value: impl Into<String>) -> Self {
        self.phone = Some(value.into());
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
    pub fn build(self) -> Result<RetrieveVerificationsResponse, BuildError> {
        Ok(RetrieveVerificationsResponse {
            address: self.address,
            business_name: self.business_name,
            business_structure: self.business_structure,
            country: self.country,
            created_at: self.created_at,
            date_of_birth: self.date_of_birth,
            email: self.email,
            first_name: self.first_name,
            id: self.id,
            kind: self.kind,
            last_name: self.last_name,
            phone: self.phone,
            requested_information: self.requested_information,
            required_documents: self.required_documents,
            session_url: self.session_url,
            status: self.status,
            updated_at: self.updated_at,
        })
    }
}
