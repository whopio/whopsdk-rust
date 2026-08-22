pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateDepositsRequest {
    /// Amount to prefill on hosted deposit page.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub amount: Option<f64>,
    /// Destination account ID or wallet address. Object form is supported for compatibility. Any business resolves by its account ID without authentication; a user account resolves only for that same authenticated user.
    pub destination: CreateDepositsRequestDestination,
    /// Metadata to include with the deposit response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    /// Destination network override. Defaults to the destination wallet's own network.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<CreateDepositsRequestNetwork>,
}

impl CreateDepositsRequest {
    pub fn builder() -> CreateDepositsRequestBuilder {
        <CreateDepositsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateDepositsRequestBuilder {
    amount: Option<f64>,
    destination: Option<CreateDepositsRequestDestination>,
    metadata: Option<HashMap<String, serde_json::Value>>,
    network: Option<CreateDepositsRequestNetwork>,
}

impl CreateDepositsRequestBuilder {
    pub fn amount(mut self, value: f64) -> Self {
        self.amount = Some(value);
        self
    }

    pub fn destination(mut self, value: CreateDepositsRequestDestination) -> Self {
        self.destination = Some(value);
        self
    }

    pub fn metadata(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn network(mut self, value: CreateDepositsRequestNetwork) -> Self {
        self.network = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateDepositsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`destination`](CreateDepositsRequestBuilder::destination)
    pub fn build(self) -> Result<CreateDepositsRequest, BuildError> {
        Ok(CreateDepositsRequest {
            amount: self.amount,
            destination: self
                .destination
                .ok_or_else(|| BuildError::missing_field("destination"))?,
            metadata: self.metadata,
            network: self.network,
        })
    }
}
