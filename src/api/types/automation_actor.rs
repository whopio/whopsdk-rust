pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AutomationActor {
    /// ID of the user, account or app, prefixed `user_`, `biz_` or `app_`.
    #[serde(default)]
    pub id: String,
    /// What acted: a `user`, an `account` through one of its API keys, or an `app`.
    pub object: AutomationActorObject,
}

impl AutomationActor {
    pub fn builder() -> AutomationActorBuilder {
        <AutomationActorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AutomationActorBuilder {
    id: Option<String>,
    object: Option<AutomationActorObject>,
}

impl AutomationActorBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn object(mut self, value: AutomationActorObject) -> Self {
        self.object = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AutomationActor`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](AutomationActorBuilder::id)
    /// - [`object`](AutomationActorBuilder::object)
    pub fn build(self) -> Result<AutomationActor, BuildError> {
        Ok(AutomationActor {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            object: self
                .object
                .ok_or_else(|| BuildError::missing_field("object"))?,
        })
    }
}
