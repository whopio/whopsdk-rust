pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateBountiesRequest {
    /// Number of submissions that can be accepted (winner slots). Defaults to 1. The escrowed total is `gross_reward_amount` times this limit and must be at least $5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accepted_submissions_limit: Option<i64>,
    /// How many winner slots one worker can win. Defaults to `1`. Wins plus proofs awaiting review never exceed this number, and a worker runs one attempt at a time. Cannot exceed `accepted_submissions_limit`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accepted_submissions_per_user_limit: Option<i64>,
    /// Account whose balance funds the bounty pool (`biz_` tag). Defaults to the caller's personal balance. Requires permission to move the account's funds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// Countries whose residents can work the bounty, as ISO 3166 alpha-2 codes. Empty means worldwide.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_country_codes: Option<Vec<String>>,
    /// What the poster wants the work to achieve, declared once here.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_goal_type: Option<CreateBountiesRequestBusinessGoalType>,
    /// Per-bounty overrides of the served capture contract. Only accepted when `business_goal_type` is `data_capture`; omitted fields keep the platform defaults, and the resulting contract is echoed back as `capture_spec` on the bounty.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_spec: Option<CreateBountiesRequestCaptureSpec>,
    /// Full task instructions shown to workers.
    #[serde(default)]
    pub description: String,
    /// Experience to host the bounty in (`exp_` tag). Any visibility — public for an open bounty, private for an invited one. Required unless account_id is set, in which case the bounty anchors in that account's public forum.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experience_id: Option<String>,
    /// How often the schedule creates a new bounty. Each occurrence is a separate bounty. Defaults to `once`; only applies with `publish_at`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<CreateBountiesRequestFrequency>,
    /// Gross bounty-pool amount (USD) escrowed per accepted submission, in whole dollars. Platform fees and affiliate shares are paid from this amount.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub gross_reward_amount: f64,
    /// ISO 8601 time to publish the bounty. When set, the bounty is created as a hidden draft and funded + published at this time instead of immediately.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish_at: Option<String>,
    /// IANA timezone for recurring occurrences. Required when publish_at is set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish_at_timezone: Option<String>,
    /// Short name of the task shown to workers.
    #[serde(default)]
    pub title: String,
}

impl CreateBountiesRequest {
    pub fn builder() -> CreateBountiesRequestBuilder {
        <CreateBountiesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateBountiesRequestBuilder {
    accepted_submissions_limit: Option<i64>,
    accepted_submissions_per_user_limit: Option<i64>,
    account_id: Option<String>,
    allowed_country_codes: Option<Vec<String>>,
    business_goal_type: Option<CreateBountiesRequestBusinessGoalType>,
    capture_spec: Option<CreateBountiesRequestCaptureSpec>,
    description: Option<String>,
    experience_id: Option<String>,
    frequency: Option<CreateBountiesRequestFrequency>,
    gross_reward_amount: Option<f64>,
    publish_at: Option<String>,
    publish_at_timezone: Option<String>,
    title: Option<String>,
}

impl CreateBountiesRequestBuilder {
    pub fn accepted_submissions_limit(mut self, value: i64) -> Self {
        self.accepted_submissions_limit = Some(value);
        self
    }

    pub fn accepted_submissions_per_user_limit(mut self, value: i64) -> Self {
        self.accepted_submissions_per_user_limit = Some(value);
        self
    }

    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn allowed_country_codes(mut self, value: Vec<String>) -> Self {
        self.allowed_country_codes = Some(value);
        self
    }

    pub fn business_goal_type(mut self, value: CreateBountiesRequestBusinessGoalType) -> Self {
        self.business_goal_type = Some(value);
        self
    }

    pub fn capture_spec(mut self, value: CreateBountiesRequestCaptureSpec) -> Self {
        self.capture_spec = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn experience_id(mut self, value: impl Into<String>) -> Self {
        self.experience_id = Some(value.into());
        self
    }

    pub fn frequency(mut self, value: CreateBountiesRequestFrequency) -> Self {
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

    /// Consumes the builder and constructs a [`CreateBountiesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`description`](CreateBountiesRequestBuilder::description)
    /// - [`gross_reward_amount`](CreateBountiesRequestBuilder::gross_reward_amount)
    /// - [`title`](CreateBountiesRequestBuilder::title)
    pub fn build(self) -> Result<CreateBountiesRequest, BuildError> {
        Ok(CreateBountiesRequest {
            accepted_submissions_limit: self.accepted_submissions_limit,
            accepted_submissions_per_user_limit: self.accepted_submissions_per_user_limit,
            account_id: self.account_id,
            allowed_country_codes: self.allowed_country_codes,
            business_goal_type: self.business_goal_type,
            capture_spec: self.capture_spec,
            description: self
                .description
                .ok_or_else(|| BuildError::missing_field("description"))?,
            experience_id: self.experience_id,
            frequency: self.frequency,
            gross_reward_amount: self
                .gross_reward_amount
                .ok_or_else(|| BuildError::missing_field("gross_reward_amount"))?,
            publish_at: self.publish_at,
            publish_at_timezone: self.publish_at_timezone,
            title: self
                .title
                .ok_or_else(|| BuildError::missing_field("title"))?,
        })
    }
}
