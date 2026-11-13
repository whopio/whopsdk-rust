pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EmbedField {
    #[serde(default)]
    pub inline: bool,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub value: String,
}

impl EmbedField {
    pub fn builder() -> EmbedFieldBuilder {
        <EmbedFieldBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EmbedFieldBuilder {
    inline: Option<bool>,
    name: Option<String>,
    value: Option<String>,
}

impl EmbedFieldBuilder {
    pub fn inline(mut self, value: bool) -> Self {
        self.inline = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`EmbedField`].
    /// This method will fail if any of the following fields are not set:
    /// - [`inline`](EmbedFieldBuilder::inline)
    /// - [`name`](EmbedFieldBuilder::name)
    /// - [`value`](EmbedFieldBuilder::value)
    pub fn build(self) -> Result<EmbedField, BuildError> {
        Ok(EmbedField {
            inline: self
                .inline
                .ok_or_else(|| BuildError::missing_field("inline"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}
