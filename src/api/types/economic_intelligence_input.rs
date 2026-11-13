pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EconomicIntelligenceInput {
    /// What the run used: one of the options, or your own text for a `whop_ai` recommendation. `null` until a run starts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub answer: Option<String>,
    /// Identifies the input when you answer it.
    #[serde(default)]
    pub id: String,
    /// What you are choosing, such as the daily ad budget.
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub options: Vec<String>,
    /// A plan sentence with one {answer} blank to fill with the selected answer.
    #[serde(default)]
    pub template: String,
}

impl EconomicIntelligenceInput {
    pub fn builder() -> EconomicIntelligenceInputBuilder {
        <EconomicIntelligenceInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EconomicIntelligenceInputBuilder {
    answer: Option<String>,
    id: Option<String>,
    label: Option<String>,
    options: Option<Vec<String>>,
    template: Option<String>,
}

impl EconomicIntelligenceInputBuilder {
    pub fn answer(mut self, value: impl Into<String>) -> Self {
        self.answer = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn options(mut self, value: Vec<String>) -> Self {
        self.options = Some(value);
        self
    }

    pub fn template(mut self, value: impl Into<String>) -> Self {
        self.template = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`EconomicIntelligenceInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](EconomicIntelligenceInputBuilder::id)
    /// - [`label`](EconomicIntelligenceInputBuilder::label)
    /// - [`options`](EconomicIntelligenceInputBuilder::options)
    /// - [`template`](EconomicIntelligenceInputBuilder::template)
    pub fn build(self) -> Result<EconomicIntelligenceInput, BuildError> {
        Ok(EconomicIntelligenceInput {
            answer: self.answer,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            label: self
                .label
                .ok_or_else(|| BuildError::missing_field("label"))?,
            options: self
                .options
                .ok_or_else(|| BuildError::missing_field("options"))?,
            template: self
                .template
                .ok_or_else(|| BuildError::missing_field("template"))?,
        })
    }
}
