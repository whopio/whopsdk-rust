pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentMethodsListQueryRequest {
    /// Returns the elements in the list that come after the specified cursor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Returns the elements in the list that come before the specified cursor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// Returns the first _n_ elements from the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<i64>,
    /// Returns the last _n_ elements from the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last: Option<i64>,
    /// The unique identifier of the member to list payment methods for. Omit this and account_id to list your own saved payment methods.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<Direction>,
    /// Only return payment methods created before this timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub created_before: Option<DateTime<FixedOffset>>,
    /// Only return payment methods created after this timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub created_after: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub future_usage: Option<FutureUsageTypes>,
    /// Only return payment methods of these types. Pass the eligible `type` values from the payment method types catalogue so the list holds nothing the purchase cannot take. An empty list returns no payment methods.
    #[serde(default)]
    pub payment_method_types: Vec<Option<PaymentMethodTypes>>,
    /// Only return cards on these networks, such as the networks the seller accepts. Payment methods that are not cards are unaffected.
    #[serde(default)]
    pub card_brands: Vec<Option<CardBrands>>,
    /// Only return cards funded this way. A card whose funding could not be determined is excluded, and payment methods that are not cards are unaffected.
    #[serde(default)]
    pub card_funding_types: Vec<Option<CardFundingTypes>>,
    /// Filter cards by whether they carry the payer identity document their payment provider requires. Payment methods that are not cards are unaffected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_payer_document: Option<bool>,
    /// Filter by expiry. Only a card can expire, so `false` keeps every payment method that is not past its expiration month and `true` returns expired cards alone.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expired: Option<bool>,
    /// Filter by whether the stored credential has permanently stopped charging, such as a vault entry its provider closed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub broken: Option<bool>,
    /// The unique identifier of the company. Provide either this or member_id, not both. Omit both to address your own saved payment methods.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}

impl PaymentMethodsListQueryRequest {
    pub fn builder() -> PaymentMethodsListQueryRequestBuilder {
        <PaymentMethodsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentMethodsListQueryRequestBuilder {
    after: Option<String>,
    before: Option<String>,
    first: Option<i64>,
    last: Option<i64>,
    member_id: Option<String>,
    direction: Option<Direction>,
    created_before: Option<DateTime<FixedOffset>>,
    created_after: Option<DateTime<FixedOffset>>,
    future_usage: Option<FutureUsageTypes>,
    payment_method_types: Option<Vec<Option<PaymentMethodTypes>>>,
    card_brands: Option<Vec<Option<CardBrands>>>,
    card_funding_types: Option<Vec<Option<CardFundingTypes>>>,
    has_payer_document: Option<bool>,
    expired: Option<bool>,
    broken: Option<bool>,
    account_id: Option<String>,
}

impl PaymentMethodsListQueryRequestBuilder {
    pub fn after(mut self, value: impl Into<String>) -> Self {
        self.after = Some(value.into());
        self
    }

    pub fn before(mut self, value: impl Into<String>) -> Self {
        self.before = Some(value.into());
        self
    }

    pub fn first(mut self, value: i64) -> Self {
        self.first = Some(value);
        self
    }

    pub fn last(mut self, value: i64) -> Self {
        self.last = Some(value);
        self
    }

    pub fn member_id(mut self, value: impl Into<String>) -> Self {
        self.member_id = Some(value.into());
        self
    }

    pub fn direction(mut self, value: Direction) -> Self {
        self.direction = Some(value);
        self
    }

    pub fn created_before(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_before = Some(value);
        self
    }

    pub fn created_after(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_after = Some(value);
        self
    }

    pub fn future_usage(mut self, value: FutureUsageTypes) -> Self {
        self.future_usage = Some(value);
        self
    }

    pub fn payment_method_types(mut self, value: Vec<Option<PaymentMethodTypes>>) -> Self {
        self.payment_method_types = Some(value);
        self
    }

    pub fn card_brands(mut self, value: Vec<Option<CardBrands>>) -> Self {
        self.card_brands = Some(value);
        self
    }

    pub fn card_funding_types(mut self, value: Vec<Option<CardFundingTypes>>) -> Self {
        self.card_funding_types = Some(value);
        self
    }

    pub fn has_payer_document(mut self, value: bool) -> Self {
        self.has_payer_document = Some(value);
        self
    }

    pub fn expired(mut self, value: bool) -> Self {
        self.expired = Some(value);
        self
    }

    pub fn broken(mut self, value: bool) -> Self {
        self.broken = Some(value);
        self
    }

    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PaymentMethodsListQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`payment_method_types`](PaymentMethodsListQueryRequestBuilder::payment_method_types)
    /// - [`card_brands`](PaymentMethodsListQueryRequestBuilder::card_brands)
    /// - [`card_funding_types`](PaymentMethodsListQueryRequestBuilder::card_funding_types)
    pub fn build(self) -> Result<PaymentMethodsListQueryRequest, BuildError> {
        Ok(PaymentMethodsListQueryRequest {
            after: self.after,
            before: self.before,
            first: self.first,
            last: self.last,
            member_id: self.member_id,
            direction: self.direction,
            created_before: self.created_before,
            created_after: self.created_after,
            future_usage: self.future_usage,
            payment_method_types: self
                .payment_method_types
                .ok_or_else(|| BuildError::missing_field("payment_method_types"))?,
            card_brands: self
                .card_brands
                .ok_or_else(|| BuildError::missing_field("card_brands"))?,
            card_funding_types: self
                .card_funding_types
                .ok_or_else(|| BuildError::missing_field("card_funding_types"))?,
            has_payer_document: self.has_payer_document,
            expired: self.expired,
            broken: self.broken,
            account_id: self.account_id,
        })
    }
}
