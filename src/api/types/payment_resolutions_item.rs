pub use crate::prelude::*;

/// A resolution center case is a dispute or support case between a user and a company, tracking the issue, status, and outcome.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PaymentResolutionsItem {
    /// Whether the customer has filed an appeal after the initial resolution decision.
    #[serde(default)]
    pub customer_appealed: bool,
    /// The list of actions currently available to the customer.
    #[serde(default)]
    pub customer_response_actions: Vec<ResolutionCenterCaseCustomerResponses>,
    /// The deadline by which the next response is required. Null if no deadline is currently active. As a Unix timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub due_date: Option<DateTime<FixedOffset>>,
    /// The unique identifier for the resolution.
    #[serde(default)]
    pub id: String,
    /// The category of the dispute.
    pub issue: ResolutionCenterCaseIssueTypes,
    /// Whether the merchant has filed an appeal after the initial resolution decision.
    #[serde(default)]
    pub merchant_appealed: bool,
    /// The list of actions currently available to the merchant.
    #[serde(default)]
    pub merchant_response_actions: Vec<ResolutionCenterCaseMerchantResponses>,
    /// The list of actions currently available to the Whop platform for moderating this resolution.
    #[serde(default)]
    pub platform_response_actions: Vec<ResolutionCenterCasePlatformResponses>,
    /// The current status of the resolution case, indicating which party needs to respond or if the case is closed.
    pub status: ResolutionCenterCaseStatuses,
}

impl PaymentResolutionsItem {
    pub fn builder() -> PaymentResolutionsItemBuilder {
        <PaymentResolutionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentResolutionsItemBuilder {
    customer_appealed: Option<bool>,
    customer_response_actions: Option<Vec<ResolutionCenterCaseCustomerResponses>>,
    due_date: Option<DateTime<FixedOffset>>,
    id: Option<String>,
    issue: Option<ResolutionCenterCaseIssueTypes>,
    merchant_appealed: Option<bool>,
    merchant_response_actions: Option<Vec<ResolutionCenterCaseMerchantResponses>>,
    platform_response_actions: Option<Vec<ResolutionCenterCasePlatformResponses>>,
    status: Option<ResolutionCenterCaseStatuses>,
}

impl PaymentResolutionsItemBuilder {
    pub fn customer_appealed(mut self, value: bool) -> Self {
        self.customer_appealed = Some(value);
        self
    }

    pub fn customer_response_actions(
        mut self,
        value: Vec<ResolutionCenterCaseCustomerResponses>,
    ) -> Self {
        self.customer_response_actions = Some(value);
        self
    }

    pub fn due_date(mut self, value: DateTime<FixedOffset>) -> Self {
        self.due_date = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn issue(mut self, value: ResolutionCenterCaseIssueTypes) -> Self {
        self.issue = Some(value);
        self
    }

    pub fn merchant_appealed(mut self, value: bool) -> Self {
        self.merchant_appealed = Some(value);
        self
    }

    pub fn merchant_response_actions(
        mut self,
        value: Vec<ResolutionCenterCaseMerchantResponses>,
    ) -> Self {
        self.merchant_response_actions = Some(value);
        self
    }

    pub fn platform_response_actions(
        mut self,
        value: Vec<ResolutionCenterCasePlatformResponses>,
    ) -> Self {
        self.platform_response_actions = Some(value);
        self
    }

    pub fn status(mut self, value: ResolutionCenterCaseStatuses) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaymentResolutionsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`customer_appealed`](PaymentResolutionsItemBuilder::customer_appealed)
    /// - [`customer_response_actions`](PaymentResolutionsItemBuilder::customer_response_actions)
    /// - [`id`](PaymentResolutionsItemBuilder::id)
    /// - [`issue`](PaymentResolutionsItemBuilder::issue)
    /// - [`merchant_appealed`](PaymentResolutionsItemBuilder::merchant_appealed)
    /// - [`merchant_response_actions`](PaymentResolutionsItemBuilder::merchant_response_actions)
    /// - [`platform_response_actions`](PaymentResolutionsItemBuilder::platform_response_actions)
    /// - [`status`](PaymentResolutionsItemBuilder::status)
    pub fn build(self) -> Result<PaymentResolutionsItem, BuildError> {
        Ok(PaymentResolutionsItem {
            customer_appealed: self
                .customer_appealed
                .ok_or_else(|| BuildError::missing_field("customer_appealed"))?,
            customer_response_actions: self
                .customer_response_actions
                .ok_or_else(|| BuildError::missing_field("customer_response_actions"))?,
            due_date: self.due_date,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            issue: self
                .issue
                .ok_or_else(|| BuildError::missing_field("issue"))?,
            merchant_appealed: self
                .merchant_appealed
                .ok_or_else(|| BuildError::missing_field("merchant_appealed"))?,
            merchant_response_actions: self
                .merchant_response_actions
                .ok_or_else(|| BuildError::missing_field("merchant_response_actions"))?,
            platform_response_actions: self
                .platform_response_actions
                .ok_or_else(|| BuildError::missing_field("platform_response_actions"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
