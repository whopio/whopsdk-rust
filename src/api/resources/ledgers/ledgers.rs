use crate::{ApiError, ClientConfig, HttpClient};

pub struct LedgersClient {
    pub http_client: HttpClient,
}

impl LedgersClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }
}
