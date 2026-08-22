pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdatePreferencesRequest2 {
    /// Whether the user has dismissed the first-time bounty worker onboarding. Set to `false` to show it again.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bounty_worker_onboarding_dismissed: Option<bool>,
    /// Whether investigation mode is enabled for the user. Only meaningful for staff users with investigation access.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub investigation_enabled: Option<bool>,
    /// Records the user's acceptance of Whop's terms and policies. Only `true` is accepted — the server stamps `terms_accepted_at` and acceptance cannot be withdrawn here.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms_accepted: Option<bool>,
}

impl UpdatePreferencesRequest2 {
    pub fn builder() -> UpdatePreferencesRequest2Builder {
        <UpdatePreferencesRequest2Builder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdatePreferencesRequest2Builder {
    bounty_worker_onboarding_dismissed: Option<bool>,
    investigation_enabled: Option<bool>,
    terms_accepted: Option<bool>,
}

impl UpdatePreferencesRequest2Builder {
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

    /// Consumes the builder and constructs a [`UpdatePreferencesRequest2`].
    pub fn build(self) -> Result<UpdatePreferencesRequest2, BuildError> {
        Ok(UpdatePreferencesRequest2 {
            bounty_worker_onboarding_dismissed: self.bounty_worker_onboarding_dismissed,
            investigation_enabled: self.investigation_enabled,
            terms_accepted: self.terms_accepted,
        })
    }
}
