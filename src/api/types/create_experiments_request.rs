pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateExperimentsRequest {
    /// Owning account tag or internal. Required; ownership cannot change.
    #[serde(default)]
    pub account_id: String,
    /// Randomization unit, and the only identity the assignment is keyed on — evaluation fails rather than falling back to another. `user` (default) uses `subject[user_id]` for account experiments and the signed-in user for internal experiments; `account` uses `subject[account_id]`, so every user of an account gets the same arm; `anonymous` uses the anonymous id and survives sign-in. Fixed after creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_by: Option<CreateExperimentsRequestBucketBy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control: Option<CreateExperimentsRequestControl>,
    /// When `true`, creates a binary feature flag rather than a full experiment. Feature flags expose the same evaluation API but do not collect metric results. Defaults to `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_flag_only: Option<bool>,
    /// Developer-chosen handle referenced from code, used in evaluation and results calls. Interchangeable with the returned `expt_` id anywhere the API takes an experiment identifier.
    #[serde(default)]
    pub flag_key: String,
    /// Required for full experiments; rejected on feature flags. Structure it as "If we [change] for [cohort], then [measurable behavior] will [increase/decrease], resulting in [business outcome], because [evidence]. Created by [name]." Evidence should be something real — a baseline number, a funnel breakdown — not a guess, and [name] is you, not something to leave blank.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hypothesis: Option<String>,
    /// Human-readable display name. Defaults to `flag_key` when omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_resource: Option<ExperimentResourceReference>,
    /// Rules that determine which subjects qualify for the experiment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targeting_rules: Option<Vec<CreateExperimentsRequestTargetingRulesItem>>,
    /// Treatment arms to create. Users outside every arm form the implicit `control` group. Required unless `feature_flag_only` is `true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variants: Option<Vec<CreateExperimentsRequestVariantsItem>>,
}

impl CreateExperimentsRequest {
    pub fn builder() -> CreateExperimentsRequestBuilder {
        <CreateExperimentsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateExperimentsRequestBuilder {
    account_id: Option<String>,
    bucket_by: Option<CreateExperimentsRequestBucketBy>,
    control: Option<CreateExperimentsRequestControl>,
    feature_flag_only: Option<bool>,
    flag_key: Option<String>,
    hypothesis: Option<String>,
    name: Option<String>,
    related_resource: Option<ExperimentResourceReference>,
    targeting_rules: Option<Vec<CreateExperimentsRequestTargetingRulesItem>>,
    variants: Option<Vec<CreateExperimentsRequestVariantsItem>>,
}

impl CreateExperimentsRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn bucket_by(mut self, value: CreateExperimentsRequestBucketBy) -> Self {
        self.bucket_by = Some(value);
        self
    }

    pub fn control(mut self, value: CreateExperimentsRequestControl) -> Self {
        self.control = Some(value);
        self
    }

    pub fn feature_flag_only(mut self, value: bool) -> Self {
        self.feature_flag_only = Some(value);
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

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn related_resource(mut self, value: ExperimentResourceReference) -> Self {
        self.related_resource = Some(value);
        self
    }

    pub fn targeting_rules(
        mut self,
        value: Vec<CreateExperimentsRequestTargetingRulesItem>,
    ) -> Self {
        self.targeting_rules = Some(value);
        self
    }

    pub fn variants(mut self, value: Vec<CreateExperimentsRequestVariantsItem>) -> Self {
        self.variants = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateExperimentsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_id`](CreateExperimentsRequestBuilder::account_id)
    /// - [`flag_key`](CreateExperimentsRequestBuilder::flag_key)
    pub fn build(self) -> Result<CreateExperimentsRequest, BuildError> {
        Ok(CreateExperimentsRequest {
            account_id: self
                .account_id
                .ok_or_else(|| BuildError::missing_field("account_id"))?,
            bucket_by: self.bucket_by,
            control: self.control,
            feature_flag_only: self.feature_flag_only,
            flag_key: self
                .flag_key
                .ok_or_else(|| BuildError::missing_field("flag_key"))?,
            hypothesis: self.hypothesis,
            name: self.name,
            related_resource: self.related_resource,
            targeting_rules: self.targeting_rules,
            variants: self.variants,
        })
    }
}
