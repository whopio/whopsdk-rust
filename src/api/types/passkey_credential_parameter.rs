pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PasskeyCredentialParameter {
    /// A COSE algorithm identifier: `-7` is ES256, `-257` is RS256.
    #[serde(default)]
    pub alg: i64,
    /// Always `public-key`.
    pub r#type: PasskeyCredentialParameterType,
}

impl PasskeyCredentialParameter {
    pub fn builder() -> PasskeyCredentialParameterBuilder {
        <PasskeyCredentialParameterBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PasskeyCredentialParameterBuilder {
    alg: Option<i64>,
    r#type: Option<PasskeyCredentialParameterType>,
}

impl PasskeyCredentialParameterBuilder {
    pub fn alg(mut self, value: i64) -> Self {
        self.alg = Some(value);
        self
    }

    pub fn r#type(mut self, value: PasskeyCredentialParameterType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PasskeyCredentialParameter`].
    /// This method will fail if any of the following fields are not set:
    /// - [`alg`](PasskeyCredentialParameterBuilder::alg)
    /// - [`r#type`](PasskeyCredentialParameterBuilder::r#type)
    pub fn build(self) -> Result<PasskeyCredentialParameter, BuildError> {
        Ok(PasskeyCredentialParameter {
            alg: self.alg.ok_or_else(|| BuildError::missing_field("alg"))?,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
