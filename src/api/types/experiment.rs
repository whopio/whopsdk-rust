pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Experiment {
    /// Owning account ID, or internal for Whop platform experiments.
    #[serde(default)]
    pub account_id: String,
    /// Assignment hashes UTF-8 seed + subject ID with CRC32 modulo 100 and selects the stored end-exclusive range.
    #[serde(default)]
    pub assignment_seed: String,
    /// Randomization unit — `user` buckets each user independently, `account` buckets whole accounts (every user of an account gets the same arm). `null` for feature flags.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_by: Option<ExperimentBucketBy>,
    /// Revision of the serving configuration. Does not change the assignment seed.
    #[serde(default)]
    pub configuration_revision: i64,
    #[serde(default)]
    pub control: ExperimentControl,
    /// When the experiment was created, as an ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// ID of the user who created the experiment, prefixed `user_`. `null` for experiments created before creators were recorded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    /// When the experiment stopped collecting data, as an ISO 8601 timestamp. `null` while still running.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ended_at: Option<String>,
    /// `true` when this was created as a feature flag rather than a full experiment. Feature flags share the same evaluation API but do not collect metric results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_flag_only: Option<bool>,
    /// What was learned and why this outcome, recorded when the experiment was ended. `null` until then.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub findings: Option<String>,
    /// Developer-chosen handle referenced from code. Anywhere the API takes an experiment identifier, the `expt_` id and the flag_key are interchangeable.
    #[serde(default)]
    pub flag_key: String,
    /// Hypothesis for this experiment. `null` when none is set, and always `null` for feature flags.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hypothesis: Option<String>,
    /// Unique identifier for the experiment, prefixed `expt_`.
    #[serde(default)]
    pub id: String,
    /// Human-readable display name.
    #[serde(default)]
    pub name: String,
    pub related_resource: ExperimentResourceReference,
    /// When the experiment began collecting data, as an ISO 8601 timestamp. `null` for drafts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<String>,
    /// Lifecycle state. `draft` — not yet live; `active` — currently running; `paused` — traffic paused; `ended` — concluded.
    pub status: ExperimentStatus,
    /// Rules gating who is in the experiment at all. Conditions within a rule are AND-ed, rules are OR-ed, and `exclude` rules always win. Empty means everyone qualifies.
    #[serde(default)]
    pub targeting_rules: Vec<ExperimentTargetingRulesItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    /// Treatment arms. Users outside every arm's allocation form the implicit `control` group. Weights only ever grow and arms are never removed, so a user moves from control into a treatment at most once.
    #[serde(default)]
    pub variants: Vec<ExperimentVariantsItem>,
    /// The treatment that won, set when the experiment was ended. Once set, every evaluation returns this arm to every caller regardless of targeting or allocation, and no further exposures are recorded. `null` means control won — an ended experiment with no winning arm evaluates to `control` for everyone. Always `null` for feature flags, which simply evaluate to disabled once ended.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub winning_arm: Option<String>,
}

impl Experiment {
    pub fn builder() -> ExperimentBuilder {
        <ExperimentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExperimentBuilder {
    account_id: Option<String>,
    assignment_seed: Option<String>,
    bucket_by: Option<ExperimentBucketBy>,
    configuration_revision: Option<i64>,
    control: Option<ExperimentControl>,
    created_at: Option<String>,
    created_by: Option<String>,
    ended_at: Option<String>,
    feature_flag_only: Option<bool>,
    findings: Option<String>,
    flag_key: Option<String>,
    hypothesis: Option<String>,
    id: Option<String>,
    name: Option<String>,
    related_resource: Option<ExperimentResourceReference>,
    started_at: Option<String>,
    status: Option<ExperimentStatus>,
    targeting_rules: Option<Vec<ExperimentTargetingRulesItem>>,
    updated_at: Option<String>,
    variants: Option<Vec<ExperimentVariantsItem>>,
    winning_arm: Option<String>,
}

impl ExperimentBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn assignment_seed(mut self, value: impl Into<String>) -> Self {
        self.assignment_seed = Some(value.into());
        self
    }

    pub fn bucket_by(mut self, value: ExperimentBucketBy) -> Self {
        self.bucket_by = Some(value);
        self
    }

    pub fn configuration_revision(mut self, value: i64) -> Self {
        self.configuration_revision = Some(value);
        self
    }

    pub fn control(mut self, value: ExperimentControl) -> Self {
        self.control = Some(value);
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn created_by(mut self, value: impl Into<String>) -> Self {
        self.created_by = Some(value.into());
        self
    }

    pub fn ended_at(mut self, value: impl Into<String>) -> Self {
        self.ended_at = Some(value.into());
        self
    }

    pub fn feature_flag_only(mut self, value: bool) -> Self {
        self.feature_flag_only = Some(value);
        self
    }

    pub fn findings(mut self, value: impl Into<String>) -> Self {
        self.findings = Some(value.into());
        self
    }

    pub fn flag_key(mut self, value: impl Into<String>) -> Self {
        self.flag_key = Some(value.into());
        self
    }

    pub fn hypothesis(mut self, value: impl Into<String>) -> Self {
        self.hypothesis = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn related_resource(mut self, value: ExperimentResourceReference) -> Self {
        self.related_resource = Some(value);
        self
    }

    pub fn started_at(mut self, value: impl Into<String>) -> Self {
        self.started_at = Some(value.into());
        self
    }

    pub fn status(mut self, value: ExperimentStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn targeting_rules(mut self, value: Vec<ExperimentTargetingRulesItem>) -> Self {
        self.targeting_rules = Some(value);
        self
    }

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    pub fn variants(mut self, value: Vec<ExperimentVariantsItem>) -> Self {
        self.variants = Some(value);
        self
    }

    pub fn winning_arm(mut self, value: impl Into<String>) -> Self {
        self.winning_arm = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`Experiment`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_id`](ExperimentBuilder::account_id)
    /// - [`assignment_seed`](ExperimentBuilder::assignment_seed)
    /// - [`configuration_revision`](ExperimentBuilder::configuration_revision)
    /// - [`control`](ExperimentBuilder::control)
    /// - [`flag_key`](ExperimentBuilder::flag_key)
    /// - [`id`](ExperimentBuilder::id)
    /// - [`name`](ExperimentBuilder::name)
    /// - [`related_resource`](ExperimentBuilder::related_resource)
    /// - [`status`](ExperimentBuilder::status)
    /// - [`targeting_rules`](ExperimentBuilder::targeting_rules)
    /// - [`variants`](ExperimentBuilder::variants)
    pub fn build(self) -> Result<Experiment, BuildError> {
        Ok(Experiment {
            account_id: self
                .account_id
                .ok_or_else(|| BuildError::missing_field("account_id"))?,
            assignment_seed: self
                .assignment_seed
                .ok_or_else(|| BuildError::missing_field("assignment_seed"))?,
            bucket_by: self.bucket_by,
            configuration_revision: self
                .configuration_revision
                .ok_or_else(|| BuildError::missing_field("configuration_revision"))?,
            control: self
                .control
                .ok_or_else(|| BuildError::missing_field("control"))?,
            created_at: self.created_at,
            created_by: self.created_by,
            ended_at: self.ended_at,
            feature_flag_only: self.feature_flag_only,
            findings: self.findings,
            flag_key: self
                .flag_key
                .ok_or_else(|| BuildError::missing_field("flag_key"))?,
            hypothesis: self.hypothesis,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            related_resource: self
                .related_resource
                .ok_or_else(|| BuildError::missing_field("related_resource"))?,
            started_at: self.started_at,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            targeting_rules: self
                .targeting_rules
                .ok_or_else(|| BuildError::missing_field("targeting_rules"))?,
            updated_at: self.updated_at,
            variants: self
                .variants
                .ok_or_else(|| BuildError::missing_field("variants"))?,
            winning_arm: self.winning_arm,
        })
    }
}
