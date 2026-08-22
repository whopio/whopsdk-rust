pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UserPreferences {
    /// Whether the user has dismissed the first-time bounty worker onboarding. Set to `false` to show it again.
    #[serde(default)]
    pub bounty_worker_onboarding_dismissed: bool,
    /// Whether investigation mode is enabled for the user. Only meaningful for staff users with investigation access.
    #[serde(default)]
    pub investigation_enabled: bool,
    /// Whether the user has accepted Whop's terms and policies. `false` until recorded via `PATCH` with `terms_accepted: true`.
    #[serde(default)]
    pub terms_accepted: bool,
    /// When the user most recently accepted Whop's terms and policies, as an ISO 8601 timestamp. `null` until accepted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms_accepted_at: Option<String>,
}

impl UserPreferences {
    pub fn builder() -> UserPreferencesBuilder {
        <UserPreferencesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UserPreferencesBuilder {
    bounty_worker_onboarding_dismissed: Option<bool>,
    investigation_enabled: Option<bool>,
    terms_accepted: Option<bool>,
    terms_accepted_at: Option<String>,
}

impl UserPreferencesBuilder {
    pub fn bounty_worker_onboarding_dismissed(mut self, value: bool) -> Self {
        self.bounty_worker_onboarding_dismissed = Some(value);
        self
    }

    pub fn investigation_enabled(mut self, value: bool) -> Self {
        self.investigation_enabled = Some(value);
        self
    }

    pub fn terms_accepted(mut self, value: bool) -> Self {
        self.terms_accepted = Some(value);
        self
    }

    pub fn terms_accepted_at(mut self, value: impl Into<String>) -> Self {
        self.terms_accepted_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UserPreferences`].
    /// This method will fail if any of the following fields are not set:
    /// - [`bounty_worker_onboarding_dismissed`](UserPreferencesBuilder::bounty_worker_onboarding_dismissed)
    /// - [`investigation_enabled`](UserPreferencesBuilder::investigation_enabled)
    /// - [`terms_accepted`](UserPreferencesBuilder::terms_accepted)
    pub fn build(self) -> Result<UserPreferences, BuildError> {
        Ok(UserPreferences {
            bounty_worker_onboarding_dismissed: self
                .bounty_worker_onboarding_dismissed
                .ok_or_else(|| BuildError::missing_field("bounty_worker_onboarding_dismissed"))?,
            investigation_enabled: self
                .investigation_enabled
                .ok_or_else(|| BuildError::missing_field("investigation_enabled"))?,
            terms_accepted: self
                .terms_accepted
                .ok_or_else(|| BuildError::missing_field("terms_accepted"))?,
            terms_accepted_at: self.terms_accepted_at,
        })
    }
}
