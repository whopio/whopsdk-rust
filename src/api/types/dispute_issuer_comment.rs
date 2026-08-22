pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DisputeIssuerComment {
    /// When the comment was received, as an ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub received_at: Option<String>,
    /// What the issuer wrote, as received.
    #[serde(default)]
    pub text: String,
}

impl DisputeIssuerComment {
    pub fn builder() -> DisputeIssuerCommentBuilder {
        <DisputeIssuerCommentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DisputeIssuerCommentBuilder {
    received_at: Option<String>,
    text: Option<String>,
}

impl DisputeIssuerCommentBuilder {
    pub fn received_at(mut self, value: impl Into<String>) -> Self {
        self.received_at = Some(value.into());
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DisputeIssuerComment`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](DisputeIssuerCommentBuilder::text)
    pub fn build(self) -> Result<DisputeIssuerComment, BuildError> {
        Ok(DisputeIssuerComment {
            received_at: self.received_at,
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
        })
    }
}
