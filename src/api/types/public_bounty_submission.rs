pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PublicBountySubmission {
    /// The bounty the work was submitted to, prefixed `bnty_`.
    #[serde(default)]
    pub bounty_id: String,
    /// When the worker claimed the submission, as an ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub claimed_at: Option<String>,
    /// When the submission was created, as an ISO 8601 timestamp.
    #[serde(default)]
    pub created_at: String,
    /// Why the submission was denied, when a presentable reason exists. Always `null` unless `status` is `denied`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub denial_reason: Option<String>,
    /// Submission ID, prefixed `btys_`.
    #[serde(default)]
    pub id: String,
    /// Latest public proof livestream attached to the submission.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_proof_livestream_feed: Option<BountySubmissionLivestreamFeed>,
    /// When the submission was approved or denied, as an ISO 8601 timestamp. `null` until then.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved_at: Option<String>,
    /// Lifecycle state. `submitted` submissions await review; `approved` submissions were accepted and paid; `denied` submissions were rejected. In-progress attempts never appear on the public list.
    pub status: PublicBountySubmissionStatus,
    /// When proof was submitted for review, as an ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submitted_at: Option<String>,
    /// When the submission was last updated, as an ISO 8601 timestamp.
    #[serde(default)]
    pub updated_at: String,
    /// User who submitted the work.
    #[serde(default)]
    pub worker: UserSummary,
}

impl PublicBountySubmission {
    pub fn builder() -> PublicBountySubmissionBuilder {
        <PublicBountySubmissionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PublicBountySubmissionBuilder {
    bounty_id: Option<String>,
    claimed_at: Option<String>,
    created_at: Option<String>,
    denial_reason: Option<String>,
    id: Option<String>,
    latest_proof_livestream_feed: Option<BountySubmissionLivestreamFeed>,
    resolved_at: Option<String>,
    status: Option<PublicBountySubmissionStatus>,
    submitted_at: Option<String>,
    updated_at: Option<String>,
    worker: Option<UserSummary>,
}

impl PublicBountySubmissionBuilder {
    pub fn bounty_id(mut self, value: impl Into<String>) -> Self {
        self.bounty_id = Some(value.into());
        self
    }

    pub fn claimed_at(mut self, value: impl Into<String>) -> Self {
        self.claimed_at = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn denial_reason(mut self, value: impl Into<String>) -> Self {
        self.denial_reason = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn latest_proof_livestream_feed(mut self, value: BountySubmissionLivestreamFeed) -> Self {
        self.latest_proof_livestream_feed = Some(value);
        self
    }

    pub fn resolved_at(mut self, value: impl Into<String>) -> Self {
        self.resolved_at = Some(value.into());
        self
    }

    pub fn status(mut self, value: PublicBountySubmissionStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn submitted_at(mut self, value: impl Into<String>) -> Self {
        self.submitted_at = Some(value.into());
        self
    }

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    pub fn worker(mut self, value: UserSummary) -> Self {
        self.worker = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PublicBountySubmission`].
    /// This method will fail if any of the following fields are not set:
    /// - [`bounty_id`](PublicBountySubmissionBuilder::bounty_id)
    /// - [`created_at`](PublicBountySubmissionBuilder::created_at)
    /// - [`id`](PublicBountySubmissionBuilder::id)
    /// - [`status`](PublicBountySubmissionBuilder::status)
    /// - [`updated_at`](PublicBountySubmissionBuilder::updated_at)
    /// - [`worker`](PublicBountySubmissionBuilder::worker)
    pub fn build(self) -> Result<PublicBountySubmission, BuildError> {
        Ok(PublicBountySubmission {
            bounty_id: self
                .bounty_id
                .ok_or_else(|| BuildError::missing_field("bounty_id"))?,
            claimed_at: self.claimed_at,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            denial_reason: self.denial_reason,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            latest_proof_livestream_feed: self.latest_proof_livestream_feed,
            resolved_at: self.resolved_at,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            submitted_at: self.submitted_at,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
            worker: self
                .worker
                .ok_or_else(|| BuildError::missing_field("worker"))?,
        })
    }
}
