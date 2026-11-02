use crate::{ApiError, ClientConfig, HttpClient};

pub struct EntriesClient {
    pub http_client: HttpClient,
}

impl EntriesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }
}
