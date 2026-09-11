pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateExperimentsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control: Option<UpdateExperimentsRequestControl>,
    /// Omit to leave unchanged. Send an empty string to clear it. Not accepted on feature flags. When setting it, structure it as "If we [change] for [cohort], then [measurable behavior] will [increase/decrease], resulting in [business outcome], because [evidence]. Created by [name]." same as on create.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hypothesis: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_resource: Option<ExperimentResourceReference>,
    /// Replace the targeting rules with this set. Omit to leave unchanged.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targeting_rules: Option<Vec<UpdateExperimentsRequestTargetingRulesItem>>,
    /// Grow treatment allocation. Pass every existing treatment with an equal-or-higher weight; append new names to add arms. Weights never decrease and arms are never removed. Omit to leave unchanged.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variants: Option<Vec<UpdateExperimentsRequestVariantsItem>>,
    /// Owning account or internal. Required when id is a flag key; optional for an expt_ ID.
    #[serde(skip)]
    pub account_id: Option<String>,
}

impl UpdateExperimentsRequest {
    pub fn builder() -> UpdateExperimentsRequestBuilder {
        <UpdateExperimentsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateExperimentsRequestBuilder {
    control: Option<UpdateExperimentsRequestControl>,
    hypothesis: Option<String>,
    related_resource: Option<ExperimentResourceReference>,
    targeting_rules: Option<Vec<UpdateExperimentsRequestTargetingRulesItem>>,
    variants: Option<Vec<UpdateExperimentsRequestVariantsItem>>,
    account_id: Option<String>,
}

impl UpdateExperimentsRequestBuilder {
    pub fn control(mut self, value: UpdateExperimentsRequestControl) -> Self {
        self.control = Some(value);
        self
    }

    pub fn hypothesis(mut self, value: impl Into<String>) -> Self {
        self.hypothesis = Some(value.into());
        self
    }

    pub fn related_resource(mut self, value: ExperimentResourceReference) -> Self {
        self.related_resource = Some(value);
        self
    }

    pub fn targeting_rules(
        mut self,
        value: Vec<UpdateExperimentsRequestTargetingRulesItem>,
    ) -> Self {
        self.targeting_rules = Some(value);
        self
    }

    pub fn variants(mut self, value: Vec<UpdateExperimentsRequestVariantsItem>) -> Self {
        self.variants = Some(value);
        self
    }

    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateExperimentsRequest`].
    pub fn build(self) -> Result<UpdateExperimentsRequest, BuildError> {
        Ok(UpdateExperimentsRequest {
            control: self.control,
            hypothesis: self.hypothesis,
            related_resource: self.related_resource,
            targeting_rules: self.targeting_rules,
            variants: self.variants,
            account_id: self.account_id,
        })
    }
}
