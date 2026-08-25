pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Bounty {
    #[serde(default)]
    pub accepted_deliverable_types: Vec<BountyAcceptedDeliverableTypesItem>,
    /// Submissions accepted so far.
    #[serde(default)]
    pub accepted_submissions_count: i64,
    /// Number of submissions that can be accepted (winner slots).
    #[serde(default)]
    pub accepted_submissions_limit: i64,
    /// How many winner slots one worker can win. Defaults to `1`. Wins plus proofs awaiting review never exceed this number, and a worker runs one attempt at a time. Cannot exceed `accepted_submissions_limit`.
    #[serde(default)]
    pub accepted_submissions_per_user_limit: i64,
    #[serde(default)]
    pub active_proof_livestream_feeds: Vec<BountyActiveLivestreamFeed>,
    /// What a referrer earns per accepted submission when the worker arrived through their affiliate link, in whole currency units, at the standard platform fee rate. Taken out of the worker's post-fee reward rather than added on top. `0` when the bounty pays no affiliate share, including bounties tied to no account, which cannot record a referral.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub affiliate_share_amount: f64,
    #[serde(default)]
    pub allowed_country_codes: Vec<String>,
    /// Submissions delivered and waiting on review. A subset of `unresolved_submissions_count`, which also counts attempts still in progress.
    #[serde(default)]
    pub awaiting_review_submissions_count: i64,
    /// Total gross budget committed to the bounty: `gross_reward_amount` times `accepted_submissions_limit`.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub budget_amount: f64,
    /// What the poster wants the work to achieve, declared once at create. `null` for bounties created before the taxonomy rolled out.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_goal_type: Option<BountyBusinessGoalType>,
    /// When cancellation was requested, as an ISO 8601 timestamp. On a `closed` bounty this means the cancel is pending: submissions are stopped and the bounty cancels once in-flight submissions resolve. On a `canceled` bounty it records when the cancellation was requested. `null` when no cancellation was ever requested.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_requested_at: Option<String>,
    /// The technical contract footage must be recorded against. Present only on `data_capture` bounties; `null` for every other goal type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_spec: Option<CaptureSpec>,
    /// When the bounty was created, as an ISO 8601 timestamp.
    #[serde(default)]
    pub created_at: String,
    /// Currency for all amounts on the bounty, as a lowercase ISO 4217 code.
    pub currency: BountyCurrency,
    /// Submissions reviewed and turned down.
    #[serde(default)]
    pub denied_submissions_count: i64,
    /// Full task instructions shown to workers.
    #[serde(default)]
    pub description: String,
    /// Experience the bounty's discussion thread lives in, prefixed `exp_`. Read this — not `experience_id` — to open the thread: a platform-wide bounty has no hosting experience of its own but its discussion still lives in one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discussion_experience_id: Option<String>,
    /// Forum feed containing the bounty's discussion thread. `null` for a bounty with no forum post.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discussion_feed_id: Option<String>,
    /// Forum post anchoring the bounty's discussion thread. Read together with `discussion_experience_id` to address the thread. `null` for a bounty with no forum post.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discussion_post_id: Option<String>,
    /// Experience the bounty is hosted in, prefixed `exp_`. `null` for platform-wide bounties; may belong to a different account than the funder.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experience_id: Option<String>,
    /// Account whose balance funds the bounty pool, or `null` when a user funds it personally. May differ from the account hosting `experience_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funding_account: Option<AccountSummary>,
    /// Gross amount paid out from the bounty pool across accepted submissions — worker payouts, platform fees, and affiliate shares together. Tips and reviewer rewards are excluded.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub gross_paid_out_amount: f64,
    /// Gross bounty-pool amount allocated per accepted submission, in whole currency units.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub gross_reward_amount: f64,
    /// Account hosting the bounty's forum — the one whose `route` and `experience_id` address its discussion thread, and where its submissions dashboard lives. `null` for a platform-wide bounty with no host. May differ from `funding_account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosting_account: Option<StorefrontAccount>,
    /// Bounty ID, prefixed `bnty_`.
    #[serde(default)]
    pub id: String,
    /// What a worker is quoted per accepted submission after the platform fee, in whole currency units. The exact post-fee figure, at the standard platform fee rate — a worker who locked a different rate, or who arrived through an affiliate link, is paid a different amount.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub net_reward_amount: f64,
    /// User who posted the bounty — the account owner when created with an account API key.
    #[serde(default)]
    pub poster: UserSummary,
    /// How often the schedule creates a new bounty. Each occurrence is a separate bounty; the original is not republished.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_frequency: Option<BountyScheduledFrequency>,
    /// When a scheduled bounty will publish, as an ISO 8601 timestamp. `null` once published, for bounties that were never scheduled, and for terminally failed drafts parked for manual rescheduling.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_publish_at: Option<String>,
    /// Unfilled winner capacity: `accepted_submissions_limit` minus `accepted_submissions_count`, clamped to zero. Not on its own a signal that the bounty accepts new claims — read `status` for that: only an `open` bounty takes new submissions.
    #[serde(default)]
    pub spots_remaining: i64,
    /// Lifecycle state. `scheduled` bounties are unpublished drafts, visible to their poster and the account's authorized managers; `open` bounties accept new submissions; `closed` bounties are live but no longer accept new submissions; `completed` bounties paid out every winner slot; `canceled` bounties ended before filling their slots.
    pub status: BountyStatus,
    /// When new submissions stopped being accepted, as an ISO 8601 timestamp. Set when a cancellation is requested on a bounty with work in flight, so in-flight submissions can resolve before the bounty cancels. `null` when submissions were never stopped — including completed bounties that simply filled every winner slot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submissions_closed_at: Option<String>,
    /// Short name of the task shown to workers.
    #[serde(default)]
    pub title: String,
    /// Submissions still awaiting an outcome: in progress or pending review.
    #[serde(default)]
    pub unresolved_submissions_count: i64,
    /// When the bounty was last updated, as an ISO 8601 timestamp.
    #[serde(default)]
    pub updated_at: String,
    /// How many winner slots the authenticated user has already won on this bounty. Read against `accepted_submissions_per_user_limit` to show a worker their remaining allowance. `0` when the request has no authenticated user.
    #[serde(default)]
    pub viewer_accepted_submissions_count: i64,
}

impl Bounty {
    pub fn builder() -> BountyBuilder {
        <BountyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BountyBuilder {
    accepted_deliverable_types: Option<Vec<BountyAcceptedDeliverableTypesItem>>,
    accepted_submissions_count: Option<i64>,
    accepted_submissions_limit: Option<i64>,
    accepted_submissions_per_user_limit: Option<i64>,
    active_proof_livestream_feeds: Option<Vec<BountyActiveLivestreamFeed>>,
    affiliate_share_amount: Option<f64>,
    allowed_country_codes: Option<Vec<String>>,
    awaiting_review_submissions_count: Option<i64>,
    budget_amount: Option<f64>,
    business_goal_type: Option<BountyBusinessGoalType>,
    cancel_requested_at: Option<String>,
    capture_spec: Option<CaptureSpec>,
    created_at: Option<String>,
    currency: Option<BountyCurrency>,
    denied_submissions_count: Option<i64>,
    description: Option<String>,
    discussion_experience_id: Option<String>,
    discussion_feed_id: Option<String>,
    discussion_post_id: Option<String>,
    experience_id: Option<String>,
    funding_account: Option<AccountSummary>,
    gross_paid_out_amount: Option<f64>,
    gross_reward_amount: Option<f64>,
    hosting_account: Option<StorefrontAccount>,
    id: Option<String>,
    net_reward_amount: Option<f64>,
    poster: Option<UserSummary>,
    scheduled_frequency: Option<BountyScheduledFrequency>,
    scheduled_publish_at: Option<String>,
    spots_remaining: Option<i64>,
    status: Option<BountyStatus>,
    submissions_closed_at: Option<String>,
    title: Option<String>,
    unresolved_submissions_count: Option<i64>,
    updated_at: Option<String>,
    viewer_accepted_submissions_count: Option<i64>,
}

impl BountyBuilder {
    pub fn accepted_deliverable_types(
        mut self,
        value: Vec<BountyAcceptedDeliverableTypesItem>,
    ) -> Self {
        self.accepted_deliverable_types = Some(value);
        self
    }

    pub fn accepted_submissions_count(mut self, value: i64) -> Self {
        self.accepted_submissions_count = Some(value);
        self
    }

    pub fn accepted_submissions_limit(mut self, value: i64) -> Self {
        self.accepted_submissions_limit = Some(value);
        self
    }

    pub fn accepted_submissions_per_user_limit(mut self, value: i64) -> Self {
        self.accepted_submissions_per_user_limit = Some(value);
        self
    }

    pub fn active_proof_livestream_feeds(mut self, value: Vec<BountyActiveLivestreamFeed>) -> Self {
        self.active_proof_livestream_feeds = Some(value);
        self
    }

    pub fn affiliate_share_amount(mut self, value: f64) -> Self {
        self.affiliate_share_amount = Some(value);
        self
    }

    pub fn allowed_country_codes(mut self, value: Vec<String>) -> Self {
        self.allowed_country_codes = Some(value);
        self
    }

    pub fn awaiting_review_submissions_count(mut self, value: i64) -> Self {
        self.awaiting_review_submissions_count = Some(value);
        self
    }

    pub fn budget_amount(mut self, value: f64) -> Self {
        self.budget_amount = Some(value);
        self
    }

    pub fn business_goal_type(mut self, value: BountyBusinessGoalType) -> Self {
        self.business_goal_type = Some(value);
        self
    }

    pub fn cancel_requested_at(mut self, value: impl Into<String>) -> Self {
        self.cancel_requested_at = Some(value.into());
        self
    }

    pub fn capture_spec(mut self, value: CaptureSpec) -> Self {
        self.capture_spec = Some(value);
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn currency(mut self, value: BountyCurrency) -> Self {
        self.currency = Some(value);
        self
    }

    pub fn denied_submissions_count(mut self, value: i64) -> Self {
        self.denied_submissions_count = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn discussion_experience_id(mut self, value: impl Into<String>) -> Self {
        self.discussion_experience_id = Some(value.into());
        self
    }

    pub fn discussion_feed_id(mut self, value: impl Into<String>) -> Self {
        self.discussion_feed_id = Some(value.into());
        self
    }

    pub fn discussion_post_id(mut self, value: impl Into<String>) -> Self {
        self.discussion_post_id = Some(value.into());
        self
    }

    pub fn experience_id(mut self, value: impl Into<String>) -> Self {
        self.experience_id = Some(value.into());
        self
    }

    pub fn funding_account(mut self, value: AccountSummary) -> Self {
        self.funding_account = Some(value);
        self
    }

    pub fn gross_paid_out_amount(mut self, value: f64) -> Self {
        self.gross_paid_out_amount = Some(value);
        self
    }

    pub fn gross_reward_amount(mut self, value: f64) -> Self {
        self.gross_reward_amount = Some(value);
        self
    }

    pub fn hosting_account(mut self, value: StorefrontAccount) -> Self {
        self.hosting_account = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn net_reward_amount(mut self, value: f64) -> Self {
        self.net_reward_amount = Some(value);
        self
    }

    pub fn poster(mut self, value: UserSummary) -> Self {
        self.poster = Some(value);
        self
    }

    pub fn scheduled_frequency(mut self, value: BountyScheduledFrequency) -> Self {
        self.scheduled_frequency = Some(value);
        self
    }

    pub fn scheduled_publish_at(mut self, value: impl Into<String>) -> Self {
        self.scheduled_publish_at = Some(value.into());
        self
    }

    pub fn spots_remaining(mut self, value: i64) -> Self {
        self.spots_remaining = Some(value);
        self
    }

    pub fn status(mut self, value: BountyStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn submissions_closed_at(mut self, value: impl Into<String>) -> Self {
        self.submissions_closed_at = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn unresolved_submissions_count(mut self, value: i64) -> Self {
        self.unresolved_submissions_count = Some(value);
        self
    }

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    pub fn viewer_accepted_submissions_count(mut self, value: i64) -> Self {
        self.viewer_accepted_submissions_count = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Bounty`].
    /// This method will fail if any of the following fields are not set:
    /// - [`accepted_deliverable_types`](BountyBuilder::accepted_deliverable_types)
    /// - [`accepted_submissions_count`](BountyBuilder::accepted_submissions_count)
    /// - [`accepted_submissions_limit`](BountyBuilder::accepted_submissions_limit)
    /// - [`accepted_submissions_per_user_limit`](BountyBuilder::accepted_submissions_per_user_limit)
    /// - [`active_proof_livestream_feeds`](BountyBuilder::active_proof_livestream_feeds)
    /// - [`affiliate_share_amount`](BountyBuilder::affiliate_share_amount)
    /// - [`allowed_country_codes`](BountyBuilder::allowed_country_codes)
    /// - [`awaiting_review_submissions_count`](BountyBuilder::awaiting_review_submissions_count)
    /// - [`budget_amount`](BountyBuilder::budget_amount)
    /// - [`created_at`](BountyBuilder::created_at)
    /// - [`currency`](BountyBuilder::currency)
    /// - [`denied_submissions_count`](BountyBuilder::denied_submissions_count)
    /// - [`description`](BountyBuilder::description)
    /// - [`gross_paid_out_amount`](BountyBuilder::gross_paid_out_amount)
    /// - [`gross_reward_amount`](BountyBuilder::gross_reward_amount)
    /// - [`id`](BountyBuilder::id)
    /// - [`net_reward_amount`](BountyBuilder::net_reward_amount)
    /// - [`poster`](BountyBuilder::poster)
    /// - [`spots_remaining`](BountyBuilder::spots_remaining)
    /// - [`status`](BountyBuilder::status)
    /// - [`title`](BountyBuilder::title)
    /// - [`unresolved_submissions_count`](BountyBuilder::unresolved_submissions_count)
    /// - [`updated_at`](BountyBuilder::updated_at)
    /// - [`viewer_accepted_submissions_count`](BountyBuilder::viewer_accepted_submissions_count)
    pub fn build(self) -> Result<Bounty, BuildError> {
        Ok(Bounty {
            accepted_deliverable_types: self
                .accepted_deliverable_types
                .ok_or_else(|| BuildError::missing_field("accepted_deliverable_types"))?,
            accepted_submissions_count: self
                .accepted_submissions_count
                .ok_or_else(|| BuildError::missing_field("accepted_submissions_count"))?,
            accepted_submissions_limit: self
                .accepted_submissions_limit
                .ok_or_else(|| BuildError::missing_field("accepted_submissions_limit"))?,
            accepted_submissions_per_user_limit: self
                .accepted_submissions_per_user_limit
                .ok_or_else(|| BuildError::missing_field("accepted_submissions_per_user_limit"))?,
            active_proof_livestream_feeds: self
                .active_proof_livestream_feeds
                .ok_or_else(|| BuildError::missing_field("active_proof_livestream_feeds"))?,
            affiliate_share_amount: self
                .affiliate_share_amount
                .ok_or_else(|| BuildError::missing_field("affiliate_share_amount"))?,
            allowed_country_codes: self
                .allowed_country_codes
                .ok_or_else(|| BuildError::missing_field("allowed_country_codes"))?,
            awaiting_review_submissions_count: self
                .awaiting_review_submissions_count
                .ok_or_else(|| BuildError::missing_field("awaiting_review_submissions_count"))?,
            budget_amount: self
                .budget_amount
                .ok_or_else(|| BuildError::missing_field("budget_amount"))?,
            business_goal_type: self.business_goal_type,
            cancel_requested_at: self.cancel_requested_at,
            capture_spec: self.capture_spec,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            denied_submissions_count: self
                .denied_submissions_count
                .ok_or_else(|| BuildError::missing_field("denied_submissions_count"))?,
            description: self
                .description
                .ok_or_else(|| BuildError::missing_field("description"))?,
            discussion_experience_id: self.discussion_experience_id,
            discussion_feed_id: self.discussion_feed_id,
            discussion_post_id: self.discussion_post_id,
            experience_id: self.experience_id,
            funding_account: self.funding_account,
            gross_paid_out_amount: self
                .gross_paid_out_amount
                .ok_or_else(|| BuildError::missing_field("gross_paid_out_amount"))?,
            gross_reward_amount: self
                .gross_reward_amount
                .ok_or_else(|| BuildError::missing_field("gross_reward_amount"))?,
            hosting_account: self.hosting_account,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            net_reward_amount: self
                .net_reward_amount
                .ok_or_else(|| BuildError::missing_field("net_reward_amount"))?,
            poster: self
                .poster
                .ok_or_else(|| BuildError::missing_field("poster"))?,
            scheduled_frequency: self.scheduled_frequency,
            scheduled_publish_at: self.scheduled_publish_at,
            spots_remaining: self
                .spots_remaining
                .ok_or_else(|| BuildError::missing_field("spots_remaining"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            submissions_closed_at: self.submissions_closed_at,
            title: self
                .title
                .ok_or_else(|| BuildError::missing_field("title"))?,
            unresolved_submissions_count: self
                .unresolved_submissions_count
                .ok_or_else(|| BuildError::missing_field("unresolved_submissions_count"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
            viewer_accepted_submissions_count: self
                .viewer_accepted_submissions_count
                .ok_or_else(|| BuildError::missing_field("viewer_accepted_submissions_count"))?,
        })
    }
}
