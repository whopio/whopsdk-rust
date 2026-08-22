pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BountySubmission {
    /// The bounty the work was submitted to, prefixed `bnty_`.
    #[serde(default)]
    pub bounty_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_clips: Option<Vec<BountyCaptureClip>>,
    /// The vendor filename stem `Country_City_Site_Station_Operator`, derived from the capture metadata. `null` until every component is present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_filename: Option<String>,
    /// Number of verified capture clips accepted for this submission so far. `0` for submissions whose deliverable doesn't accumulate clips.
    #[serde(default)]
    pub captured_clip_count: i64,
    /// Total verified duration of accepted capture clips, in whole seconds. `0` for submissions whose deliverable doesn't accumulate clips.
    #[serde(default)]
    pub captured_duration_seconds: i64,
    /// Capture metadata: city the footage was recorded in. `null` unless capture metadata was provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// When the worker claimed the submission, as an ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub claimed_at: Option<String>,
    /// Written proof the worker submitted with their work.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// Capture metadata: country the footage was recorded in. `null` unless capture metadata was provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// When the submission was created, as an ISO 8601 timestamp.
    #[serde(default)]
    pub created_at: String,
    /// How the work arrived when it came in through the API in one shot, informational only — read the work from `deliverable_urls`, `files`, and `capture_clips` directly. `null` for submissions whose proof is a livestream recording, including ones that attached links or files on submit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deliverable_type: Option<BountySubmissionDeliverableType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deliverable_urls: Option<Vec<String>>,
    /// Why the submission was denied, when a presentable reason exists. Always `null` unless `status` is `denied`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub denial_reason: Option<String>,
    /// Capture metadata: device the footage was recorded on. `null` unless capture metadata was provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<String>,
    #[serde(default)]
    pub files: Vec<BountySubmissionFile>,
    /// Capture metadata: horizontal field of view in degrees. `null` when not reported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fov: Option<i64>,
    /// Submission ID, prefixed `btys_`.
    #[serde(default)]
    pub id: String,
    /// Latest public proof livestream attached to the submission.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_proof_livestream_feed: Option<BountySubmissionLivestreamFeed>,
    /// Capture metadata: identifier of the person who recorded the footage. `null` unless capture metadata was provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,
    /// When the submission was approved or denied, as an ISO 8601 timestamp. `null` until then.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved_at: Option<String>,
    /// Capture metadata: site or venue the footage was recorded at. `null` unless capture metadata was provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site: Option<String>,
    /// Capture metadata: station or position within the site. `null` unless capture metadata was provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub station: Option<String>,
    /// Lifecycle state. `in_progress` submissions are active attempts that have not submitted proof yet; `submitted` submissions await review; `approved` submissions were accepted and paid; `denied` submissions were rejected. `null` when the attempt ended without proof, taking it out of the public lifecycle — those attempts are absent from every public list and read.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<BountySubmissionStatus>,
    /// When proof was submitted for review, as an ISO 8601 timestamp. `null` while the attempt is in progress.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submitted_at: Option<String>,
    /// When the submission was last updated, as an ISO 8601 timestamp.
    #[serde(default)]
    pub updated_at: String,
    /// User who submitted the work.
    #[serde(default)]
    pub worker: UserSummary,
}

impl BountySubmission {
    pub fn builder() -> BountySubmissionBuilder {
        <BountySubmissionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BountySubmissionBuilder {
    bounty_id: Option<String>,
    capture_clips: Option<Vec<BountyCaptureClip>>,
    capture_filename: Option<String>,
    captured_clip_count: Option<i64>,
    captured_duration_seconds: Option<i64>,
    city: Option<String>,
    claimed_at: Option<String>,
    content: Option<String>,
    country: Option<String>,
    created_at: Option<String>,
    deliverable_type: Option<BountySubmissionDeliverableType>,
    deliverable_urls: Option<Vec<String>>,
    denial_reason: Option<String>,
    device: Option<String>,
    files: Option<Vec<BountySubmissionFile>>,
    fov: Option<i64>,
    id: Option<String>,
    latest_proof_livestream_feed: Option<BountySubmissionLivestreamFeed>,
    operator: Option<String>,
    resolved_at: Option<String>,
    site: Option<String>,
    station: Option<String>,
    status: Option<BountySubmissionStatus>,
    submitted_at: Option<String>,
    updated_at: Option<String>,
    worker: Option<UserSummary>,
}

impl BountySubmissionBuilder {
    pub fn bounty_id(mut self, value: impl Into<String>) -> Self {
        self.bounty_id = Some(value.into());
        self
    }

    pub fn capture_clips(mut self, value: Vec<BountyCaptureClip>) -> Self {
        self.capture_clips = Some(value);
        self
    }

    pub fn capture_filename(mut self, value: impl Into<String>) -> Self {
        self.capture_filename = Some(value.into());
        self
    }

    pub fn captured_clip_count(mut self, value: i64) -> Self {
        self.captured_clip_count = Some(value);
        self
    }

    pub fn captured_duration_seconds(mut self, value: i64) -> Self {
        self.captured_duration_seconds = Some(value);
        self
    }

    pub fn city(mut self, value: impl Into<String>) -> Self {
        self.city = Some(value.into());
        self
    }

    pub fn claimed_at(mut self, value: impl Into<String>) -> Self {
        self.claimed_at = Some(value.into());
        self
    }

    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.content = Some(value.into());
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

    pub fn deliverable_type(mut self, value: BountySubmissionDeliverableType) -> Self {
        self.deliverable_type = Some(value);
        self
    }

    pub fn deliverable_urls(mut self, value: Vec<String>) -> Self {
        self.deliverable_urls = Some(value);
        self
    }

    pub fn denial_reason(mut self, value: impl Into<String>) -> Self {
        self.denial_reason = Some(value.into());
        self
    }

    pub fn device(mut self, value: impl Into<String>) -> Self {
        self.device = Some(value.into());
        self
    }

    pub fn files(mut self, value: Vec<BountySubmissionFile>) -> Self {
        self.files = Some(value);
        self
    }

    pub fn fov(mut self, value: i64) -> Self {
        self.fov = Some(value);
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

    pub fn operator(mut self, value: impl Into<String>) -> Self {
        self.operator = Some(value.into());
        self
    }

    pub fn resolved_at(mut self, value: impl Into<String>) -> Self {
        self.resolved_at = Some(value.into());
        self
    }

    pub fn site(mut self, value: impl Into<String>) -> Self {
        self.site = Some(value.into());
        self
    }

    pub fn station(mut self, value: impl Into<String>) -> Self {
        self.station = Some(value.into());
        self
    }

    pub fn status(mut self, value: BountySubmissionStatus) -> Self {
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

    /// Consumes the builder and constructs a [`BountySubmission`].
    /// This method will fail if any of the following fields are not set:
    /// - [`bounty_id`](BountySubmissionBuilder::bounty_id)
    /// - [`captured_clip_count`](BountySubmissionBuilder::captured_clip_count)
    /// - [`captured_duration_seconds`](BountySubmissionBuilder::captured_duration_seconds)
    /// - [`created_at`](BountySubmissionBuilder::created_at)
    /// - [`files`](BountySubmissionBuilder::files)
    /// - [`id`](BountySubmissionBuilder::id)
    /// - [`updated_at`](BountySubmissionBuilder::updated_at)
    /// - [`worker`](BountySubmissionBuilder::worker)
    pub fn build(self) -> Result<BountySubmission, BuildError> {
        Ok(BountySubmission {
            bounty_id: self
                .bounty_id
                .ok_or_else(|| BuildError::missing_field("bounty_id"))?,
            capture_clips: self.capture_clips,
            capture_filename: self.capture_filename,
            captured_clip_count: self
                .captured_clip_count
                .ok_or_else(|| BuildError::missing_field("captured_clip_count"))?,
            captured_duration_seconds: self
                .captured_duration_seconds
                .ok_or_else(|| BuildError::missing_field("captured_duration_seconds"))?,
            city: self.city,
            claimed_at: self.claimed_at,
            content: self.content,
            country: self.country,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            deliverable_type: self.deliverable_type,
            deliverable_urls: self.deliverable_urls,
            denial_reason: self.denial_reason,
            device: self.device,
            files: self
                .files
                .ok_or_else(|| BuildError::missing_field("files"))?,
            fov: self.fov,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            latest_proof_livestream_feed: self.latest_proof_livestream_feed,
            operator: self.operator,
            resolved_at: self.resolved_at,
            site: self.site,
            station: self.station,
            status: self.status,
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
