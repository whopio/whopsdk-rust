pub use crate::prelude::*;

/// A consolidated identity or business profile synced from verification provider data.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct IdentityProfile {
    /// Registered business address reported by the identity provider. Present on `business` profiles.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_address: Option<IdentityProfileBusinessAddress>,
    /// Business entity name. Present on `business` profiles.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_name: Option<String>,
    /// Reported legal structure of a business profile (e.g. `corp`, `llc`). Provider-specific values; present on `business` profiles.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_structure: Option<String>,
    /// ISO 3166-1 alpha-2 country code reported by the identity provider, such as `US` or `GB`. For individuals this is the country of citizenship or residence; for businesses, the country of incorporation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// When the identity profile was first created.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// ISO date (`YYYY-MM-DD`) reported by the identity provider. Present on `individual` profiles.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<String>,
    /// Email address reported by the identity provider. Typically present on `individual` profiles.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Individual's first name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// The tag of the identity profile (idpf_xxx).
    #[serde(default)]
    pub id: String,
    /// Individual's last name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// The companies this identity profile is currently linked to. Only populated for direct Whop user sessions; always empty when authenticated via API key, app, or OAuth scope (a single identity can be linked to companies the calling platform is not entitled to see).
    #[serde(default)]
    pub linked_companies: Vec<IdentityProfileLinkedCompaniesItem>,
    /// Progress of payout-account setup for this profile, independent of holds. `connected` means onboarding is complete; a `connected` status paired with `payouts_enabled: false` indicates an active account restriction rather than incomplete setup.
    pub payout_status: PayoutAccountCalculatedStatuses,
    /// Whether this profile can receive payouts right now. True only when payout onboarding is complete and no payout holds are active on the linked account. Treat this as the single source of truth for payout readiness.
    #[serde(default)]
    pub payouts_enabled: bool,
    /// Residential address reported by the identity provider. Present on `individual` profiles.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personal_address: Option<IdentityProfilePersonalAddress>,
    /// Phone number reported by the identity provider. Typically present on `individual` profiles.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// Whether this is an 'individual' or 'business' profile.
    #[serde(default)]
    pub profile_type: String,
    /// Derived verification status across all linked verifications. Returns `action_required` whenever the profile has an open request for information (whether a verification, payout, or audit RFI) — i.e. the merchant must submit something before it is in good standing.
    pub status: IdentityProfileStatuses,
    /// When the identity profile was last synced from a verification.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
    /// All verification attempts attached to this identity profile, ordered most-recent first.
    #[serde(default)]
    pub verifications: Vec<IdentityProfileVerificationsItem>,
}

impl IdentityProfile {
    pub fn builder() -> IdentityProfileBuilder {
        <IdentityProfileBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct IdentityProfileBuilder {
    business_address: Option<IdentityProfileBusinessAddress>,
    business_name: Option<String>,
    business_structure: Option<String>,
    country: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    date_of_birth: Option<String>,
    email: Option<String>,
    first_name: Option<String>,
    id: Option<String>,
    last_name: Option<String>,
    linked_companies: Option<Vec<IdentityProfileLinkedCompaniesItem>>,
    payout_status: Option<PayoutAccountCalculatedStatuses>,
    payouts_enabled: Option<bool>,
    personal_address: Option<IdentityProfilePersonalAddress>,
    phone: Option<String>,
    profile_type: Option<String>,
    status: Option<IdentityProfileStatuses>,
    updated_at: Option<DateTime<FixedOffset>>,
    verifications: Option<Vec<IdentityProfileVerificationsItem>>,
}

impl IdentityProfileBuilder {
    pub fn business_address(mut self, value: IdentityProfileBusinessAddress) -> Self {
        self.business_address = Some(value);
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

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
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

    pub fn last_name(mut self, value: impl Into<String>) -> Self {
        self.last_name = Some(value.into());
        self
    }

    pub fn linked_companies(mut self, value: Vec<IdentityProfileLinkedCompaniesItem>) -> Self {
        self.linked_companies = Some(value);
        self
    }

    pub fn payout_status(mut self, value: PayoutAccountCalculatedStatuses) -> Self {
        self.payout_status = Some(value);
        self
    }

    pub fn payouts_enabled(mut self, value: bool) -> Self {
        self.payouts_enabled = Some(value);
        self
    }

    pub fn personal_address(mut self, value: IdentityProfilePersonalAddress) -> Self {
        self.personal_address = Some(value);
        self
    }

    pub fn phone(mut self, value: impl Into<String>) -> Self {
        self.phone = Some(value.into());
        self
    }

    pub fn profile_type(mut self, value: impl Into<String>) -> Self {
        self.profile_type = Some(value.into());
        self
    }

    pub fn status(mut self, value: IdentityProfileStatuses) -> Self {
        self.status = Some(value);
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn verifications(mut self, value: Vec<IdentityProfileVerificationsItem>) -> Self {
        self.verifications = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`IdentityProfile`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](IdentityProfileBuilder::created_at)
    /// - [`id`](IdentityProfileBuilder::id)
    /// - [`linked_companies`](IdentityProfileBuilder::linked_companies)
    /// - [`payout_status`](IdentityProfileBuilder::payout_status)
    /// - [`payouts_enabled`](IdentityProfileBuilder::payouts_enabled)
    /// - [`profile_type`](IdentityProfileBuilder::profile_type)
    /// - [`status`](IdentityProfileBuilder::status)
    /// - [`updated_at`](IdentityProfileBuilder::updated_at)
    /// - [`verifications`](IdentityProfileBuilder::verifications)
    pub fn build(self) -> Result<IdentityProfile, BuildError> {
        Ok(IdentityProfile {
            business_address: self.business_address,
            business_name: self.business_name,
            business_structure: self.business_structure,
            country: self.country,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            date_of_birth: self.date_of_birth,
            email: self.email,
            first_name: self.first_name,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            last_name: self.last_name,
            linked_companies: self
                .linked_companies
                .ok_or_else(|| BuildError::missing_field("linked_companies"))?,
            payout_status: self
                .payout_status
                .ok_or_else(|| BuildError::missing_field("payout_status"))?,
            payouts_enabled: self
                .payouts_enabled
                .ok_or_else(|| BuildError::missing_field("payouts_enabled"))?,
            personal_address: self.personal_address,
            phone: self.phone,
            profile_type: self
                .profile_type
                .ok_or_else(|| BuildError::missing_field("profile_type"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
            verifications: self
                .verifications
                .ok_or_else(|| BuildError::missing_field("verifications"))?,
        })
    }
}
