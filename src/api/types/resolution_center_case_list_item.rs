pub use crate::prelude::*;

/// A resolution center case is a dispute or support case between a user and a company, tracking the issue, status, and outcome.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ResolutionCenterCaseListItem {
    /// The company involved in this resolution case. Null if the company no longer exists.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<ResolutionCenterCaseListItemCompany>,
    /// The datetime the resolution was created.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
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
    /// The payment record that is the subject of this resolution case.
    #[serde(default)]
    pub payment: ResolutionCenterCaseListItemPayment,
    /// The current status of the resolution case, indicating which party needs to respond or if the case is closed.
    pub status: ResolutionCenterCaseStatuses,
    /// The datetime the resolution was last updated.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
    /// The customer (buyer) who filed this resolution case.
    #[serde(default)]
    pub user: ResolutionCenterCaseListItemUser,
}

impl ResolutionCenterCaseListItem {
    pub fn builder() -> ResolutionCenterCaseListItemBuilder {
        <ResolutionCenterCaseListItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResolutionCenterCaseListItemBuilder {
    company: Option<ResolutionCenterCaseListItemCompany>,
    created_at: Option<DateTime<FixedOffset>>,
    customer_appealed: Option<bool>,
    customer_response_actions: Option<Vec<ResolutionCenterCaseCustomerResponses>>,
    due_date: Option<DateTime<FixedOffset>>,
    id: Option<String>,
    issue: Option<ResolutionCenterCaseIssueTypes>,
    merchant_appealed: Option<bool>,
    merchant_response_actions: Option<Vec<ResolutionCenterCaseMerchantResponses>>,
    payment: Option<ResolutionCenterCaseListItemPayment>,
    status: Option<ResolutionCenterCaseStatuses>,
    updated_at: Option<DateTime<FixedOffset>>,
    user: Option<ResolutionCenterCaseListItemUser>,
}

impl ResolutionCenterCaseListItemBuilder {
    pub fn company(mut self, value: ResolutionCenterCaseListItemCompany) -> Self {
        self.company = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

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

    pub fn payment(mut self, value: ResolutionCenterCaseListItemPayment) -> Self {
        self.payment = Some(value);
        self
    }

    pub fn status(mut self, value: ResolutionCenterCaseStatuses) -> Self {
        self.status = Some(value);
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn user(mut self, value: ResolutionCenterCaseListItemUser) -> Self {
        self.user = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ResolutionCenterCaseListItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](ResolutionCenterCaseListItemBuilder::created_at)
    /// - [`customer_appealed`](ResolutionCenterCaseListItemBuilder::customer_appealed)
    /// - [`customer_response_actions`](ResolutionCenterCaseListItemBuilder::customer_response_actions)
    /// - [`id`](ResolutionCenterCaseListItemBuilder::id)
    /// - [`issue`](ResolutionCenterCaseListItemBuilder::issue)
    /// - [`merchant_appealed`](ResolutionCenterCaseListItemBuilder::merchant_appealed)
    /// - [`merchant_response_actions`](ResolutionCenterCaseListItemBuilder::merchant_response_actions)
    /// - [`payment`](ResolutionCenterCaseListItemBuilder::payment)
    /// - [`status`](ResolutionCenterCaseListItemBuilder::status)
    /// - [`updated_at`](ResolutionCenterCaseListItemBuilder::updated_at)
    /// - [`user`](ResolutionCenterCaseListItemBuilder::user)
    pub fn build(self) -> Result<ResolutionCenterCaseListItem, BuildError> {
        Ok(ResolutionCenterCaseListItem {
            company: self.company,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
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
            payment: self
                .payment
                .ok_or_else(|| BuildError::missing_field("payment"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
            user: self.user.ok_or_else(|| BuildError::missing_field("user"))?,
        })
    }
}
