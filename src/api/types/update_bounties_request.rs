pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateBountiesRequest {
    /// Scheduled drafts only. Number of submissions that can be accepted (winner slots).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accepted_submissions_limit: Option<i64>,
    /// How many winner slots one worker can win. Defaults to `1`. Wins plus proofs awaiting review never exceed this number, and a worker runs one attempt at a time. Cannot exceed `accepted_submissions_limit`. Editable while the bounty is still open with nothing under review.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accepted_submissions_per_user_limit: Option<i64>,
    /// Replace the countries whose residents can work the bounty, as ISO 3166 alpha-2 codes. Empty means worldwide.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_country_codes: Option<Vec<String>>,
    /// What the poster wants the work to achieve, declared once here.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_goal_type: Option<UpdateBountiesRequestBusinessGoalType>,
    /// New full task instructions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Scheduled drafts only. How often the schedule creates a new bounty.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<UpdateBountiesRequestFrequency>,
    /// Scheduled drafts only. Gross bounty-pool amount (USD) escrowed per accepted submission. The escrowed total (this times accepted_submissions_limit) must stay at least $5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gross_reward_amount: Option<f64>,
    /// Scheduled drafts only. New ISO 8601 time to publish the draft. Must be in the future.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish_at: Option<String>,
    /// Scheduled drafts only. IANA timezone for recurring occurrences.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish_at_timezone: Option<String>,
    /// New short name of the task.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl UpdateBountiesRequest {
    pub fn builder() -> UpdateBountiesRequestBuilder {
        <UpdateBountiesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateBountiesRequestBuilder {
    accepted_submissions_limit: Option<i64>,
    accepted_submissions_per_user_limit: Option<i64>,
    allowed_country_codes: Option<Vec<String>>,
    business_goal_type: Option<UpdateBountiesRequestBusinessGoalType>,
    description: Option<String>,
    frequency: Option<UpdateBountiesRequestFrequency>,
    gross_reward_amount: Option<f64>,
    publish_at: Option<String>,
    publish_at_timezone: Option<String>,
    title: Option<String>,
}

impl UpdateBountiesRequestBuilder {
    pub fn accepted_submissions_limit(mut self, value: i64) -> Self {
        self.accepted_submissions_limit = Some(value);
        self
    }

    pub fn accepted_submissions_per_user_limit(mut self, value: i64) -> Self {
        self.accepted_submissions_per_user_limit = Some(value);
        self
    }

    pub fn allowed_country_codes(mut self, value: Vec<String>) -> Self {
        self.allowed_country_codes = Some(value);
        self
    }

    pub fn business_goal_type(mut self, value: UpdateBountiesRequestBusinessGoalType) -> Self {
        self.business_goal_type = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn frequency(mut self, value: UpdateBountiesRequestFrequency) -> Self {
        self.frequency = Some(value);
        self
    }

    pub fn gross_reward_amount(mut self, value: f64) -> Self {
        self.gross_reward_amount = Some(value);
        self
    }

    pub fn publish_at(mut self, value: impl Into<String>) -> Self {
        self.publish_at = Some(value.into());
        self
    }

    pub fn publish_at_timezone(mut self, value: impl Into<String>) -> Self {
        self.publish_at_timezone = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateBountiesRequest`].
    pub fn build(self) -> Result<UpdateBountiesRequest, BuildError> {
        Ok(UpdateBountiesRequest {
            accepted_submissions_limit: self.accepted_submissions_limit,
            accepted_submissions_per_user_limit: self.accepted_submissions_per_user_limit,
            allowed_country_codes: self.allowed_country_codes,
            business_goal_type: self.business_goal_type,
            description: self.description,
            frequency: self.frequency,
            gross_reward_amount: self.gross_reward_amount,
            publish_at: self.publish_at,
            publish_at_timezone: self.publish_at_timezone,
            title: self.title,
        })
    }
}
