pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AccountRecommendedActionChain {
    #[serde(default)]
    pub actions: Vec<AccountRecommendedActionChainStep>,
    /// What running the chain accomplishes
    #[serde(default)]
    pub description: String,
    /// Chain ID — `rac_seed_<chain>_<nonce>` for seeded chains, `rac_chain_*` for generated ones
    #[serde(default)]
    pub id: String,
    /// Why this chain was recommended, or `null` when unavailable
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<HashMap<String, serde_json::Value>>,
    /// Headline for the chain
    #[serde(default)]
    pub title: String,
}

impl AccountRecommendedActionChain {
    pub fn builder() -> AccountRecommendedActionChainBuilder {
        <AccountRecommendedActionChainBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AccountRecommendedActionChainBuilder {
    actions: Option<Vec<AccountRecommendedActionChainStep>>,
    description: Option<String>,
    id: Option<String>,
    reasoning: Option<HashMap<String, serde_json::Value>>,
    title: Option<String>,
}

impl AccountRecommendedActionChainBuilder {
    pub fn actions(mut self, value: Vec<AccountRecommendedActionChainStep>) -> Self {
        self.actions = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn reasoning(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.reasoning = Some(value);
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AccountRecommendedActionChain`].
    /// This method will fail if any of the following fields are not set:
    /// - [`actions`](AccountRecommendedActionChainBuilder::actions)
    /// - [`description`](AccountRecommendedActionChainBuilder::description)
    /// - [`id`](AccountRecommendedActionChainBuilder::id)
    /// - [`title`](AccountRecommendedActionChainBuilder::title)
    pub fn build(self) -> Result<AccountRecommendedActionChain, BuildError> {
        Ok(AccountRecommendedActionChain {
            actions: self
                .actions
                .ok_or_else(|| BuildError::missing_field("actions"))?,
            description: self
                .description
                .ok_or_else(|| BuildError::missing_field("description"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            reasoning: self.reasoning,
            title: self
                .title
                .ok_or_else(|| BuildError::missing_field("title"))?,
        })
    }
}
