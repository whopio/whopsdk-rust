use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub mod experiences;
pub use experiences::ExperiencesClient2;
pub mod topics;
pub use topics::TopicsClient2;
pub struct NotificationsClient2 {
    pub http_client: HttpClient,
    pub experiences: ExperiencesClient2,
    pub topics: TopicsClient2,
}

impl NotificationsClient2 {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            experiences: ExperiencesClient2::new(config.clone())?,
            topics: TopicsClient2::new(config.clone())?,
        })
    }

    /// Sets the authenticated user's notification preferences. Each preference is addressed by `scope`, not by id, so a scope read back from either list endpoint can be sent straight here.
    ///
    /// A scope naming an experience with no topic sets that experience's level, and accepts all three levels. Any other scope sets a topic override, which is binary — `all` or `nothing` — and requires a `channel`.
    ///
    /// `level: null` clears the preference. Preferences are stored as overrides, so clearing one means the scope inherits its default again rather than being switched off.
    ///
    /// The batch is applied in one transaction: if any entry is rejected, none are written. Experience levels are applied before topic overrides, because setting a level replaces every topic preference for that experience — so an override sent alongside a level wins. The response reports what each scope now resolves to, in the order the entries were sent.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use whop_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = Whop::new(config).expect("Failed to build client");
    ///     client
    ///         .users
    ///         .preferences
    ///         .notifications
    ///         .set(
    ///             &SetNotificationsRequest {
    ///                 preferences: vec![SetNotificationsRequestPreferencesItem {
    ///                     scope: SetNotificationsRequestPreferencesItemScope {
    ///                         ..Default::default()
    ///                     },
    ///                     ..Default::default()
    ///                 }],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn set(
        &self,
        request: &SetNotificationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<SetNotificationsResponse, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-08-21-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::PATCH,
                "users/me/preferences/notifications",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
